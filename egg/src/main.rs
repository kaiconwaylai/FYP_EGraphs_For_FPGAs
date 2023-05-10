use egg::*;
use std::fs;
use std::io::prelude::*;


mod utils;
use utils::{fpga,costs::*};

#[macro_use]
extern crate fstrings;



define_language! {
    enum BitLanguage {
        "+" = Add([Id; 2]),
        "+" = AddW([Id; 3]),
        "*" = Mul([Id; 3]),
        "*64" = Mul64([Id; 2]),
        "*128" = Mul128([Id; 2]),
        "-" = Sub([Id; 2]),
        "-" = SubW([Id; 3]),
        "<<" = Lsl([Id; 2]),
        ">>" = Lsr([Id; 2]),
        "slice" = Slc([Id; 3]),
        "concat" = Cct([Id; 2]),
        Num(i32),        
        Symbol(Symbol),
    }
}


fn var(s: &str) -> Var {
    s.parse().unwrap()
}

fn make_rules() -> Vec<Rewrite<BitLanguage, ()>> {
    vec![
        //rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        //rewrite!("commute-mul"; "(* ?num ?a ?b)" => "(* ?num ?b ?a)"),
        //rewrite!("karatsuba64"; "(*64 ?a ?b)" => "(+ (<< 32 (- (* 33 (+ (slice ?a 63 32) (slice ?a 31 0)) (+ (slice ?b 63 32) (slice ?b 31 0))) (+ (* 32 (slice ?a 63 32) (slice ?b 63 32)) (* 32 (slice ?a 31 0) (slice ?b 31 0))))) (+ (<< 64 (* 32 (slice ?a 63 32) (slice ?b 63 32))) (* 32 (slice ?a 31 0) (slice ?b 31 0))))"),
        //rewrite!("karatsuba128"; "(*128 ?a ?b)" => "(+ (<< 64 (- (* 65 (+ (slice ?a 127 64) (slice ?a 63 0)) (+ (slice ?b 127 64) (slice ?b 63 0))) (+ (*64 (slice ?a 127 64) (slice ?b 127 64)) (*64 (slice ?a 63 0) (slice ?b 63 0))))) (+ (<< 128 (* 32 (slice ?a 127 64) (slice ?b 127 64))) (*64 (slice ?a 63 0) (slice ?b 63 0))))"),

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
    
    if true {
        let mut lp_extractor = LpExtractor::new(&runner.egraph, FPGACostFunction{egraph: &runner.egraph});
        let best_sol = lp_extractor.solve(root);
        println!("LP Simplified {} to {}", expr, best_sol);
        return best_sol.to_string();
    } else {
        let extractor = Extractor::new(&runner.egraph, FPGACostFunction{egraph: &runner.egraph});
        let (best_cost, best) = extractor.find_best(root);
        println!("Simplified {} to {} with cost {}", expr, best, best_cost);
        return best.to_string();
    }
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
        let mut bw_val : i32 = 0;

        for node in egraph[*bw_id].nodes.iter() {
            if let BitLanguage::Num(x) = node {
                bw_val = *x;
                break;
            }
        }
        // Compute Karasuba String Dynamically 
        let karatsuba_string; 
        if bw_val < 32 {
            karatsuba_string = String::from("(* ?bw ?x ?y)");
        } else {
            let msb = ((bw_val/2)-1).to_string();
            let lsb = String::from("0");
            let xlo = f!("(slice ?x {msb} {lsb})");
            let ylo = f!("(slice ?y {msb} {lsb})");
            let msb = (bw_val-1).to_string();
            let lsb = (bw_val/2).to_string();
            let xhi = f!("(slice ?x {msb} {lsb})");
            let yhi = f!("(slice ?y {msb} {lsb})");
            
            let half_bw = (bw_val/2).to_string();
            let z0 = f!("(* {half_bw} {xlo} {ylo})");
            let z2 = f!("(* {half_bw} {xhi} {yhi})");
            let z1 = f!("(- {sub_width} (* {mul_bw} (+ {add_width} {xlo} {xhi}) (+ {add_width} {ylo} {yhi})) (+ {add_width_2} {z2} {z0}))", mul_bw = bw_val/2 + 1, sub_width = bw_val+1, add_width = half_bw, add_width_2 = bw_val);
            
            karatsuba_string = f!("(concat (+ {add_width} (concat {z2} (slice {z0} {msb} {half_bw})) {z1}) (slice {z0} {half_z0} 0))", msb = bw_val-1, half_z0 = (bw_val/2)-1, add_width = bw_val * 3/2);
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
// END

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let input = "(* 2048 IN1 IN2)";
    let result = simplify(input);
    let mut dst = fs::File::create("results.txt")?;

    generate_verilog(result, 2048);

    for i in 0..11 {
        let i = i as f64/10.0;
        alpha(i);
        let result = simplify(input);
        write!(dst, "Alpha = {}. Result = {}\n\n", i, result)?;
    }

    Ok(())
}

pub type EGraphVerilogGeneration = egg::EGraph<BitLanguage, VerilogGeneration>;

#[derive(Default)]
// VerilogGeneration
// An e-class analysis that evaluates and builds up the verilog operations in the e-graph itself.
pub struct VerilogGeneration;
impl Analysis<BitLanguage> for VerilogGeneration {
    type Data = ( 
        String, //name
        u64,    // bw
        String  // verilog op
    );
   
    fn make(egraph: &EGraphVerilogGeneration, enode: &BitLanguage) -> Self::Data {
        let get_name     = |i: &Id| egraph[*i].data.0.clone();
        let get_bw       = |i: &Id| egraph[*i].data.1.clone();

        let enode_name = bitlanguage_to_name(enode);

        let name = format!("{}_{}", enode_name, egraph.classes().len());

        match enode {
            BitLanguage::AddW([a,b,c]) => {
                let node = &egraph[*a].nodes[0];
                if let BitLanguage::Num(x) = node {
                    let bit_width = *x as u64;
                    return (name, bit_width, format!("{} + {}", get_name(b), get_name(c)));
                }
                else {
                    assert!(false);
                }
                (name, 0, String::default())
            }
            BitLanguage::Symbol(a) => {
                (a.to_string(), 0, a.to_string())
            }
            _ => (name, 0, String::default())
        }
    }

    fn merge(&mut self, _a: &mut Self::Data, _b: Self::Data) -> DidMerge {
        DidMerge(false,false)
    }
}

fn generate_verilog(expr : String, variable_bitwidth : u64) {
    let mut generation_egraph = EGraphVerilogGeneration::default();
    let root = generation_egraph.add_expr(&expr.parse().unwrap());

    for class in generation_egraph.classes() {
        let node = &class.nodes[0];
        match node {
            BitLanguage::Symbol(_) => (),
            _ => println!("logic [{}:0] {};", class.data.1 - 1, class.data.0),
        }
    }

    for class in generation_egraph.classes() {
        let node = &class.nodes[0];
        match node {
            BitLanguage::Symbol(_) => (),
            _ => println!("{} = {};", class.data.0, class.data.2),
        }
    }
}

fn bitlanguage_to_name(enode: &BitLanguage) -> String {
    match enode {
        BitLanguage::Add(_) => "add".to_string(),
        _                   => "panic".to_string()
        // "+" = AddW([Id; 3]),
        // "*" = Mul([Id; 3]),
        // "*64" = Mul64([Id; 2]),
        // "*128" = Mul128([Id; 2]),
        // "-" = Sub([Id; 2]),
        // "-" = SubW([Id; 3]),
        // "<<" = Lsl([Id; 2]),
        // ">>" = Lsr([Id; 2]),
        // "slice" = Slc([Id; 3]),
        // "concat" = Cct([Id; 2]),
    }
}

struct FPGACostFunction<'a> {
    egraph: &'a EGraph<BitLanguage, ()>,
}

impl<'a> CostFunction<BitLanguage> for FPGACostFunction<'a> {
    type Cost = fpga::Cost;
    fn cost<C>(&mut self, enode: &BitLanguage, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost
    {
        let op_cost = match enode.to_string().as_str() {
            "*" => {
                if let BitLanguage::Mul([a,_b,_c]) = enode {
                    let node = &self.egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        let bit_width = *x as f64;
                        let t1 = ((bit_width-9.)/17.).ceil();
                        let t2 = ((bit_width-9.)/(17.*t1-5.)).floor();
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

impl<'a> LpCostFunction<BitLanguage, ()> for FPGACostFunction<'a> {
    fn node_cost(&mut self, egraph: &EGraph<BitLanguage, ()>, _eclass: Id, enode: &BitLanguage) -> f64
    {
        let op_cost = match enode.to_string().as_str() {
            "*" => {
                if let BitLanguage::Mul([a,_b,_c]) = enode {
                    let node = &egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        //let bit_width = *x as f64;
                        let cost = mul_cost(*x);
                        return (1.0-alpha(-1.0)) * cost.dsp as f64 + alpha(-1.0) * cost.lut as f64;
                    }
                }
                0.0
            },
            "-" => {
                if let BitLanguage::AddW([a,_b,_c]) = enode {
                    let node = &egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        let bit_width = *x as f64;
                        return bit_width * alpha(-1.0);
                    }
                }
                0.0
            }
            "+" => {
                if let BitLanguage::SubW([a,_b,_c]) = enode {
                    let node = &egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        let bit_width = *x as f64;
                        return bit_width * alpha(-1.0);
                    }
                }
                0.0
            }
            _ => 0.0,
        };
        op_cost
    }
}
