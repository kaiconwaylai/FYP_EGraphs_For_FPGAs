use egg::*;

define_language! {
    pub enum BitLanguage {
        "+" = Add([Id; 2]),
        "+" = AddW([Id; 3]),
        "*" = Mul([Id; 3]),
        "*64" = Mul64([Id; 2]),
        "*128" = Mul128([Id; 2]),
        "-" = Sub([Id; 2]),
        "-" = SubW([Id; 3]),
        "slice" = Slc([Id; 3]),
        "concat" = Cct([Id; 2]),
        Num(i32),        
        Symbol(Symbol),
    }
}

fn var(s: &str) -> Var {
    s.parse().unwrap()
}

pub fn make_rules() -> Vec<Rewrite<BitLanguage, ()>> {
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

//-----------------------------------------------------------------------------------
// DYNAMIC REWRITE CALCULATIONS
//-----------------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaratsubaExpand {
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
