use egg::*;
use std::collections::HashSet;

mod fpga;

#[macro_use]
extern crate fstrings;

define_language! {
    enum BitLanguage {
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 3]),
        "*64" = Mul64([Id; 2]),
        "*128" = Mul128([Id; 2]),
        "-" = Sub([Id; 2]),
        "<<" = Lsl([Id; 2]),
        ">>" = Lsr([Id; 2]),
        "slice" = Slc([Id; 3]),
        Num(i32),        
        Symbol(Symbol),
    }
}

// fn is_common_expr(seen_exprs: &HashSet, egraph: &EGraph, enode: &BitLanguage) {

//     false;
// }

struct FPGACostFunction<'a> {
    egraph: &'a EGraph<BitLanguage, ()>,
}
impl<'a> CostFunction<BitLanguage> for FPGACostFunction<'a> {
    type Cost = fpga::Cost;

    fn cost<C>(&mut self, enode: &BitLanguage, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost
    {
        let mut common_expr = HashSet::new();
        let op_cost = match enode.to_string().as_str() {
            "*" => {
                if let BitLanguage::Mul([a,b,c]) = enode {

                    if !common_expr.insert([*a,*b,*c]) {
                        return fpga::Cost{dsp: 0, lut: 0};
                    }
                    println!("inserted {} {} {}", a,b,c);

                    let node = &self.egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        let bit_width = *x as f64;
                        let t1 = ((bit_width-9.)/17.).ceil();
                        let t2 = ((bit_width-9.)/(17.*t1-5.)).floor();
                        println!("{}, {}, {}", a,b,c);
                        println!("dsp: {}, lut: {}", ((t1).powf(2.0) + t2), x*6);
                        return fpga::Cost{dsp: ((t1).powf(2.0) + t2) as i32, lut: x * 6};
                    }
                }
                return fpga::Cost{dsp: 0, lut: 0};
            },
            _ => Self::Cost {dsp: 0, lut: 1},
        };
        enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

fn make_rules() -> Vec<Rewrite<BitLanguage, ()>> {
    vec![
        //rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        //rewrite!("commute-mul"; "(* ?num ?a ?b)" => "(* ?num ?b ?a)"),
        rewrite!("karatsuba64"; "(*64 ?a ?b)" => "(+ (<< 32 (- (* 33 (+ (slice ?a 63 32) (slice ?a 31 0)) (+ (slice ?b 63 32) (slice ?b 31 0))) (+ (* 32 (slice ?a 63 32) (slice ?b 63 32)) (* 32 (slice ?a 31 0) (slice ?b 31 0))))) (+ (<< 64 (* 32 (slice ?a 63 32) (slice ?b 63 32))) (* 32 (slice ?a 31 0) (slice ?b 31 0))))"),
        rewrite!("karatsuba128"; "(*128 ?a ?b)" => "(+ (<< 64 (- (* 65 (+ (slice ?a 127 64) (slice ?a 63 0)) (+ (slice ?b 127 64) (slice ?b 63 0))) (+ (*64 (slice ?a 127 64) (slice ?b 127 64)) (*64 (slice ?a 63 0) (slice ?b 63 0))))) (+ (<< 128 (* 32 (slice ?a 127 64) (slice ?b 127 64))) (*64 (slice ?a 63 0) (slice ?b 63 0))))"),

        rewrite!("karatsuba_expansion"; "(* ?bw ?x ?y)" => {
            KaratsubaExpand {
                bw : var("?bw"),
            }
        }),
    ]
}

fn simplify(s: &str) -> String {
    let expr: RecExpr<BitLanguage> = s.parse().unwrap();
    // simplify the expression using a Runner, which creates an e-graph with the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&make_rules());
    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];
    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, FPGACostFunction{egraph: &runner.egraph});
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}

fn main() {
    println!("Hello, world!");
    simplify("(* 64 in1 in2)");
}


//-----------------------------------------------------------------------------------
// DYNAMIC REWRITE CALCULATIONS
//-----------------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
struct KaratsubaExpand {
    bw: Var,
}

impl Applier<BitLanguage, ()> for KaratsubaExpand {
    fn apply_one(
        &self,
        egraph: &mut EGraph<BitLanguage, ()>,
        _matched_id: Id,
        subst: &Subst,
        _searcher_pattern: Option<&PatternAst<BitLanguage>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
        //Id's of the class containing the operators bitwidth
        let bw_id = subst.get(self.bw).unwrap();
        let mut bw_val : i32 = 0; // need to get this value from egraph node ?

        for node in egraph[*bw_id].nodes.iter() {
            if let BitLanguage::Num(x) = node {
                bw_val = *x;
                break;
            }
        }
        // Compute Karasuba String Dynamically 
        let karatsuba_string; 
        if bw_val < 32 {
            karatsuba_string = "(* ?bw ?x ?y)".to_string();
        } else {
            let half_bw = (bw_val/2).to_string();
            let xlo = f!("(slice ?x {msb} {lsb})", msb = ((bw_val/2) - 1).to_string(), lsb = "0");
            let ylo = f!("(slice ?y {msb} {lsb})", msb = ((bw_val/2) - 1).to_string(), lsb = "0");
            let xhi = f!("(slice ?x {msb} {lsb})", msb = (bw_val - 1).to_string(), lsb = (bw_val/2).to_string());
            let yhi = f!("(slice ?y {msb} {lsb})", msb = (bw_val - 1).to_string(), lsb = (bw_val/2).to_string());
    
            let z0 = f!("(* {half_bw} {xlo} {ylo})");
            let z2 = f!("(* {half_bw} {xhi} {yhi})");
            let z1 = f!("(- (* {mul_bw} (+ {xlo} {xhi}) (+ {ylo} {yhi})) (+ {z2} {z0}))", mul_bw = (bw_val/2 + 1).to_string());
    
            karatsuba_string = f!("(+ (<< {bw} {z2}) (+ {z0} (<< {half_bw} {z1})))", bw = bw_val.to_string());
        }

        //can clean this up + find solution for odd numbers
        // End Karatsuba Dynamic Computation
        // TODO : fill this in!
        let (from, did_something) = egraph.union_instantiations(
                &"(* ?bw ?x ?y)".parse().unwrap(),
                &karatsuba_string.parse().unwrap(),
                subst,
                rule_name.clone(),
            );
        if did_something {
            println!("{}", karatsuba_string);
            return vec![from];
        }
        vec![]
    }
}
