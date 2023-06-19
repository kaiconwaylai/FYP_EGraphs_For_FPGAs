use egg::*;

// meat here are all the dynamic rewrites
// a lot of repetition within in each one which could maybe be improved
// add a helper function that returns slices from vector input of ranges for both inputs
// i.e. fn generate_slices(Vec<i32, i32> in1, Vec<i32,i32> in2) -> (Vec<String>, Vec<String>);


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
        "<<" = Lsl([Id; 2]),
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
        rewrite!("different_bitwidth"; "(* ?bw1 ?bw2 ?x ?y)" => {
            DifferentBW {
                bw_1 : var("?bw1"),
                bw_2 : var("?bw2"),
            }
        }),
        rewrite!("slice_rewrite"; "(* ?bw ?x ?y)" => {
            SliceRewrite {
                bw : var("?bw"),
            }
        }),
        // rewrite!("32-tiling-3dsps"; "(* 32 ?x ?y)" => {
        //     TilingRewrite{}
        // }),
        // rewrite!("32-tiling-2dsps"; "(* 32 ?x ?y)" => {
        //     TilingRewrite2{}
        // }),
        ]
}


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
        let bw_id = subst.get(self.bw).unwrap();
        let mut bw_val : i32 = 0;

        for node in egraph[*bw_id].nodes.iter() {
            if let BitLanguage::Num(x) = node {
                bw_val = *x;
                break;
            }
        }
        if bw_val < 10 {
            return vec![];
        } 
        let karatsuba_string; 

        let msb = ((bw_val/2)-1).to_string();
        let xlo = format!("(slice ?x {msb} 0)");
        let ylo = format!("(slice ?y {msb} 0)");
        let lsb = bw_val/2;
        let msb = bw_val-1;
        let xhi = format!("(slice ?x {msb} {lsb})");
        let yhi = format!("(slice ?y {msb} {lsb})");
        let z0 = format!("(* {half_bw} {xlo} {ylo})", half_bw = bw_val/2);
        let z2 = format!("(* {half_bw} {xhi} {yhi})", half_bw = bw_val - (bw_val/2));
        let z1;
        if bw_val < 36 {
            if bw_val % 2 == 1 {
                z1 = format!("(+ {add_width} (* {hi_width} {lo_width} {xhi} {ylo}) (* {lo_width} {hi_width} {xlo} {yhi}))", 
                add_width = bw_val+1, hi_width = msb-lsb+1, lo_width = bw_val/2);
            } else {
                z1 = format!("(+ {add_width} (* {mul_width} {xhi} {ylo}) (* {mul_width} {xlo} {yhi}))",
                                add_width = bw_val+1, mul_width = bw_val/2);
            }
        } else {            
            z1 = format!("(- {sub_width} (- {sub_width} (* {mul_bw} (+ {add_width} {xlo} {xhi}) (+ {add_width} {ylo} {yhi})) {z2}) {z0})",
                             sub_width = bw_val+1, add_width = (bw_val - bw_val/2)+1, mul_bw  = (bw_val - bw_val/2)+1);
        }
        karatsuba_string = format!("(concat (+ {add_width} (concat {z2} (slice {z0} {_msb} {half_bw})) {z1}) (slice {z0} {half_z0} 0))",
                                         _msb = 2*(bw_val/2)-1, half_z0 = (bw_val/2)-1, add_width = 1 + bw_val * 3/2, half_bw = bw_val/2); 

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
        let bw_id = subst.get(self.bw).unwrap();
        let mut bw_val : i32 = 0;

        for node in egraph[*bw_id].nodes.iter() {
            if let BitLanguage::Num(x) = node {
                bw_val = *x;
                break;
            }
        }
        if bw_val > 34 || bw_val < 27 {
            return vec![];
        }         

        let slice_size = 18;
        let rem_size = bw_val-slice_size;
        let xlo = format!("(slice ?x {} 0)", slice_size-1);
        let ylo = format!("(slice ?y {} 0)", slice_size-1);
        let xhi = format!("(slice ?x {} {})", bw_val-1, slice_size);
        let yhi = format!("(slice ?y {} {})", bw_val-1, slice_size);

        let z0 = format!("(* {} {xlo} {ylo})", slice_size);
        let z2 = format!("(* {} {xhi} {yhi})", rem_size);
        let z1 = format!("(- {sub_bw} (- {sub_bw} (* {mul_bw} (+ {add_bw} {xlo} {xhi}) (+ {add_bw} {ylo} {yhi})) {z2}) {z0})"
                            , mul_bw = 19, add_bw = 19, sub_bw = 39);

        let slice_string = format!("(concat (+ {add_width} (concat {z2} (slice {z0} {z0_msb} {slice_size})) {z1}) (slice {z0} {ss_sub1} 0))"
                            , add_width = rem_size*2 + slice_size + 1, ss_sub1 = slice_size-1, z0_msb = slice_size*2 - 1);
        
        let (from, did_something) = egraph.union_instantiations(
            &"(* ?bw ?x ?y)".parse().unwrap(),
            &slice_string.parse().unwrap(),
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
pub struct DifferentBW {
    bw_1: Var,
    bw_2: Var,
}

impl Applier<BitLanguage, ()> for DifferentBW {
    fn apply_one(
        &self,
        egraph: &mut EGraph<BitLanguage, ()>,
        _matched_id: Id,
        subst: &Subst,
        _searcher_pattern: Option<&PatternAst<BitLanguage>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
        let bw_1_id = subst.get(self.bw_1).unwrap();
        let bw_2_id = subst.get(self.bw_2).unwrap();
        let mut bw_1_val : i32 = 0;
        let mut bw_2_val : i32 = 0;

        for node in egraph[*bw_1_id].nodes.iter() {
            if let BitLanguage::Num(x) = node {
                bw_1_val = *x;
                break;
            }
        }
        for node in egraph[*bw_2_id].nodes.iter() {
            if let BitLanguage::Num(x) = node {
                bw_2_val = *x;
                break;
            }
        }
        let rewrite; 
        if bw_1_val < 10 && bw_2_val < 10 {
            rewrite = String::from("(* ?bw1 ?bw2 ?x ?y)");
        } else {     
            if (bw_1_val-bw_2_val).abs() == 1 {
                let half_bw;
                if bw_1_val > bw_2_val {
                    half_bw = bw_1_val / 2;
                } else {
                    half_bw = bw_2_val / 2;
                }
                
                let xlo = format!("(slice ?x {} 0)", half_bw-1);
                let ylo = format!("(slice ?y {} 0)", half_bw-1);
                let xhi = format!("(slice ?x {} {})", bw_1_val-1, half_bw);
                let yhi = format!("(slice ?y {} {})", bw_2_val-1, half_bw);

                let z2 = format!("(* {xhi} {yhi})");
                let z0 = format!("(* {} {xlo} {ylo})", half_bw);
                let z1 = format!("(+ (* {xlo} {yhi}) (* {xhi} {ylo}))");

                rewrite = format!("(+ (<< {} {z2}) (+ (<< {} {z1}) {z0}))", (half_bw)*2, half_bw); 
            } else {
                rewrite = String::from("(* ?bw1 ?bw2 ?x ?y)");
            }
        }
        
        let (from, did_something) = egraph.union_instantiations(
            &"(* ?bw1 ?bw2 ?x ?y)".parse().unwrap(),
            &rewrite.parse().unwrap(),
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
pub struct TilingRewrite {
}

impl Applier<BitLanguage, ()> for TilingRewrite {
    fn apply_one(
        &self,
        egraph: &mut EGraph<BitLanguage, ()>,
        _matched_id: Id,
        subst: &Subst,
        _searcher_pattern: Option<&PatternAst<BitLanguage>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
            
        let x_1 = format!("(slice ?x 5 0)"); 
        let x_2 = format!("(slice ?x 31 6)"); 

        let y_1 = format!("(slice ?y 8 0)"); 
        let y_2 = format!("(slice ?y 31 9)"); 
        let y_3 = format!("(slice ?y 17 0)"); 
        let y_4 = format!("(slice ?y 31 18)"); 

        let z0 = format!("(* 6 9 {x_1} {y_1})");
        let z1 = format!("(* 6 23 {x_1} {y_2})");
        let z2 = format!("(* 26 18 {x_2} {y_3})");
        let z3 = format!("(* 26 14 {x_2} {y_4})");

        let rewrite = format!("(+ 65 (<< 24 {z3}) (+ 51 (<< 6 {z2}) (+ 39 (<< 9 {z1}) {z0})))");

        let (from, did_something) = egraph.union_instantiations(
            &"(* 32 ?x ?y)".parse().unwrap(),
            &rewrite.parse().unwrap(),
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
pub struct TilingRewrite2 {
}
impl Applier<BitLanguage, ()> for TilingRewrite2 {
    fn apply_one(
        &self,
        egraph: &mut EGraph<BitLanguage, ()>,
        _matched_id: Id,
        subst: &Subst,
        _searcher_pattern: Option<&PatternAst<BitLanguage>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
        let x_1 = format!("(slice ?x 5 0)"); 
        let x_2 = format!("(slice ?x 31 6)"); 

        let y_1 = format!("(slice ?y 7 0)"); 
        let y_2 = format!("(slice ?y 15 8)"); 
        let y_3 = format!("(slice ?y 23 16)"); 
        let y_4 = format!("(slice ?y 31 24)"); 

        let z0 = format!("(* 26 32 {x_2} ?y)");
        let z1 = format!("(* 6 8 {x_1} {y_1})");
        let z2 = format!("(* 6 8 {x_1} {y_2})");
        let z3 = format!("(* 6 8 {x_1} {y_3})");
        let z4 = format!("(* 6 8 {x_1} {y_4})");

        let rewrite = format!("(+ 64 (<< 6 {z0}) (+ 39 (<< 24 {z4}) (+ 31 (<< 16 {z3}) (+ 23 (<< 8 {z2}) {z1}))) )");
        
        let (from, did_something) = egraph.union_instantiations(
            &"(* 32 ?x ?y)".parse().unwrap(),
            &rewrite.parse().unwrap(),
            subst,
            rule_name.clone(),
        );
        if did_something {
            return vec![from];
        }
        vec![]
    }
}
