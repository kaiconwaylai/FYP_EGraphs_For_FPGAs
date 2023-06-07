use egg::*;

define_language! {
    pub enum BitLanguage {
        "+" = Add([Id; 2]),
        "+" = AddW([Id; 3]),
        "*" = MulNW([Id; 2]),
        "*" = Mul([Id; 3]),
        "*" = Mul4([Id; 4]),
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
        rewrite!("karatsuba_expansion"; "(* ?bw ?x ?y)" => {
            KaratsubaExpand {
                bw : var("?bw"),
            }
        }),
        // rewrite!("slice_rewrite"; "(* ?bw ?x ?y)" => {
        //     SliceRewrite {
        //         bw : var("?bw"),
        //     }
        // }), 119 -> 78 41
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
        if bw_val < 10 {
            karatsuba_string = String::from("(* ?bw ?x ?y)");
        } else {
            let msb = ((bw_val/2)-1).to_string();
            let lsb = String::from("0");
            let xlo = format!("(slice ?x {msb} {lsb})");
            let ylo = format!("(slice ?y {msb} {lsb})");
            let msb = (bw_val-1).to_string();
            let lsb = (bw_val/2).to_string();
            let xhi = format!("(slice ?x {msb} {lsb})");
            let yhi = format!("(slice ?y {msb} {lsb})");
            let z0 = format!("(* {half_bw} {xlo} {ylo})", half_bw = bw_val/2);
            let z2 = format!("(* {half_bw} {xhi} {yhi})", half_bw = bw_val - (bw_val/2));
            let z1;
            if bw_val < 36 {
                if bw_val % 2 == 1 {
                    z1 = format!("(+ {add_width} (* {xhi} {ylo}) (* {xlo} {yhi}))", add_width = bw_val+2);
                } else {
                    z1 = format!("(+ {add_width} (* {mul_width} {xhi} {ylo}) (* {mul_width} {xlo} {yhi}))", add_width = bw_val+2, mul_width = bw_val/2);
                }
            } else {            
                z1 = format!("(- {sub_width} (- {sub_width} (* {mul_bw} (+ {add_width} {xlo} {xhi}) (+ {add_width} {ylo} {yhi})) {z2}) {z0})", sub_width = bw_val+1, add_width = (bw_val - bw_val/2)+1, mul_bw  = (bw_val - bw_val/2)+1);
            }
            karatsuba_string = format!("(concat (+ {add_width} (concat {z2} (slice {z0} {_msb} {half_bw})) {z1}) (slice {z0} {half_z0} 0))", _msb = 2*(bw_val/2)-1, half_z0 = (bw_val/2)-1, add_width = 1 + bw_val * 3/2, half_bw = bw_val/2); // add_width is a hack with the +1
        }

        
        // TODO : fill this in!
        let (from, did_something) = egraph.union_instantiations(
            &"(* ?bw ?x ?y)".parse().unwrap(),
            &karatsuba_string.parse().unwrap(),
            subst,
            rule_name.clone(),
        );
        if did_something {
            return vec![from];
        }
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceRewrite {
    bw: Var,
}

impl Applier<BitLanguage, ()> for SliceRewrite {
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
        let slice_string; 
        if bw_val < 36 {
            slice_string = String::from("(* ?bw ?x ?y)");
        } else {            
            let slice_size = 18;
            let rem_size = bw_val-slice_size;
            let xlo = format!("(slice ?x {} 0)", slice_size-1);
            let ylo = format!("(slice ?y {} 0)", slice_size-1);
            let xhi = format!("(slice ?x {} {})", bw_val-1, slice_size);
            let yhi = format!("(slice ?y {} {})", bw_val-1, slice_size);

            let z2 = format!("(* {} {xhi} {yhi})", rem_size*2);
            let z0 = format!("(* {} {xlo} {ylo})", slice_size*2);
            let z1 = format!("(+ (* {bw_val} {xlo} {yhi}) (* {bw_val} {xhi} {ylo}))");

            slice_string = format!("(+ {z2} (+ {z1} {z0}))");
        }
        
        //can clean this up + find solution for odd numbers
        // End Karatsuba Dynamic Computation
        // TODO : fill this in!
        let (from, did_something) = egraph.union_instantiations(
            &"(* ?bw ?x ?y)".parse().unwrap(),
            &slice_string.parse().unwrap(),
            subst,
            rule_name.clone(),
        );
        if did_something {
            println!("{}", slice_string);
            return vec![from];
        }
        vec![]
    }
}
// END
