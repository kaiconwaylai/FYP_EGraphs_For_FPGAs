use egg::*;
use std::collections::{HashMap,HashSet};
use crate::utils::fpga;
use crate::utils::language::*;

pub fn mul_cost(width : i32) -> fpga::Cost {
    let mul_costs : HashMap<i32, fpga::Cost> = HashMap::from([
        (1, fpga::Cost{dsp: 0, lut: 1}),
        (2, fpga::Cost{dsp: 0, lut: 2}),
        (3, fpga::Cost{dsp: 0, lut: 5}),
        (4, fpga::Cost{dsp: 0, lut: 18}),
        (5, fpga::Cost{dsp: 0, lut: 29}),
        (6, fpga::Cost{dsp: 0, lut: 43}),
        (7, fpga::Cost{dsp: 0, lut: 52}),
        (8, fpga::Cost{dsp: 0, lut: 71}),
        (9, fpga::Cost{dsp: 0, lut: 82}),
        (10, fpga::Cost{dsp: 1, lut: 0}),
        (11, fpga::Cost{dsp: 1, lut: 0}),
        (12, fpga::Cost{dsp: 1, lut: 0}),
        (13, fpga::Cost{dsp: 1, lut: 0}),
        (14, fpga::Cost{dsp: 1, lut: 0}),
        (15, fpga::Cost{dsp: 1, lut: 0}),
        (16, fpga::Cost{dsp: 1, lut: 0}),
        (17, fpga::Cost{dsp: 1, lut: 0}),
        (18, fpga::Cost{dsp: 1, lut: 0}),
        (19, fpga::Cost{dsp: 1, lut: 41}),
        (20, fpga::Cost{dsp: 1, lut: 78}),
        (21, fpga::Cost{dsp: 2, lut: 0}),
        (22, fpga::Cost{dsp: 2, lut: 0}),
        (23, fpga::Cost{dsp: 2, lut: 0}),
        (24, fpga::Cost{dsp: 2, lut: 0}),
        (25, fpga::Cost{dsp: 2, lut: 0}),
        (26, fpga::Cost{dsp: 2, lut: 0}),
        (27, fpga::Cost{dsp: 4, lut: 38}),
        (28, fpga::Cost{dsp: 4, lut: 40}),
        (29, fpga::Cost{dsp: 4, lut: 42}),
        (30, fpga::Cost{dsp: 4, lut: 44}),
        (31, fpga::Cost{dsp: 4, lut: 46}),
        (32, fpga::Cost{dsp: 4, lut: 48}),
        (33, fpga::Cost{dsp: 4, lut: 50}),
        (34, fpga::Cost{dsp: 4, lut: 52}),
        (35, fpga::Cost{dsp: 4, lut: 64}),
        (36, fpga::Cost{dsp: 4, lut: 84}),
        (37, fpga::Cost{dsp: 4, lut: 122}),
        (38, fpga::Cost{dsp: 5, lut: 71}),
        (39, fpga::Cost{dsp: 5, lut: 75}),
        (40, fpga::Cost{dsp: 5, lut: 79}),
        (41, fpga::Cost{dsp: 5, lut: 83}),
        (42, fpga::Cost{dsp: 5, lut: 86}),
        (43, fpga::Cost{dsp: 5, lut: 88}),
        (44, fpga::Cost{dsp: 9, lut: 73}),
        (45, fpga::Cost{dsp: 9, lut: 75}),
        (46, fpga::Cost{dsp: 9, lut: 77}),
        (47, fpga::Cost{dsp: 9, lut: 79}),
        (48, fpga::Cost{dsp: 9, lut: 81}),
        (49, fpga::Cost{dsp: 9, lut: 83}),
        (50, fpga::Cost{dsp: 9, lut: 85}),
        (51, fpga::Cost{dsp: 9, lut: 87}),
        (52, fpga::Cost{dsp: 9, lut: 89}),
        (53, fpga::Cost{dsp: 9, lut: 111}),
        (54, fpga::Cost{dsp: 9, lut: 148}),
        (55, fpga::Cost{dsp: 10, lut: 95}),
        (56, fpga::Cost{dsp: 10, lut: 97}),
        (57, fpga::Cost{dsp: 10, lut: 99}),
        (58, fpga::Cost{dsp: 10, lut: 101}),
        (59, fpga::Cost{dsp: 10, lut: 103}),
        (60, fpga::Cost{dsp: 10, lut: 105}),
        (61, fpga::Cost{dsp: 16, lut: 171}),
        (62, fpga::Cost{dsp: 16, lut: 173}),
        (63, fpga::Cost{dsp: 16, lut: 175}),
        (64, fpga::Cost{dsp: 16, lut: 177}),
        (65, fpga::Cost{dsp: 16, lut: 179}),
        (66, fpga::Cost{dsp: 16, lut: 181}),
        (67, fpga::Cost{dsp: 16, lut: 185}),
        (68, fpga::Cost{dsp: 16, lut: 187}),
        (69, fpga::Cost{dsp: 16, lut: 173}),
        (70, fpga::Cost{dsp: 16, lut: 195}),
        (71, fpga::Cost{dsp: 16, lut: 234}),
        (72, fpga::Cost{dsp: 17, lut: 179}),
        (73, fpga::Cost{dsp: 17, lut: 181}),
        (74, fpga::Cost{dsp: 17, lut: 183}),
        (75, fpga::Cost{dsp: 17, lut: 185}),
        (76, fpga::Cost{dsp: 17, lut: 187}),
        (77, fpga::Cost{dsp: 17, lut: 189}),
        (78, fpga::Cost{dsp: 25, lut: 281}),
        (79, fpga::Cost{dsp: 25, lut: 285}),
        (80, fpga::Cost{dsp: 25, lut: 289}),
        (81, fpga::Cost{dsp: 25, lut: 293}),
        (82, fpga::Cost{dsp: 25, lut: 297}),
        (83, fpga::Cost{dsp: 25, lut: 301}),
        (84, fpga::Cost{dsp: 25, lut: 304}),
        (85, fpga::Cost{dsp: 25, lut: 306}),
        (86, fpga::Cost{dsp: 25, lut: 327}),
        (87, fpga::Cost{dsp: 25, lut: 351}),
        (88, fpga::Cost{dsp: 25, lut: 392}),
        (89, fpga::Cost{dsp: 26, lut: 314}),
        (90, fpga::Cost{dsp: 26, lut: 316}),
        (91, fpga::Cost{dsp: 26, lut: 318}),
        (92, fpga::Cost{dsp: 26, lut: 320}),
        (93, fpga::Cost{dsp: 26, lut: 322}),
        (94, fpga::Cost{dsp: 26, lut: 324}),
        (95, fpga::Cost{dsp: 36, lut: 349}),
        (96, fpga::Cost{dsp: 36, lut: 353}),
        (97, fpga::Cost{dsp: 36, lut: 357}),
        (98, fpga::Cost{dsp: 36, lut: 361}),
        (99, fpga::Cost{dsp: 36, lut: 369}),
        (100, fpga::Cost{dsp: 36, lut: 373}),
        (101, fpga::Cost{dsp: 36, lut: 377}),
        (102, fpga::Cost{dsp: 36, lut: 381}),
        (103, fpga::Cost{dsp: 36, lut: 411}),
        (104, fpga::Cost{dsp: 36, lut: 429}),
        (105, fpga::Cost{dsp: 36, lut: 471}),
        (106, fpga::Cost{dsp: 37, lut: 421}),
        (107, fpga::Cost{dsp: 37, lut: 427}),
        (108, fpga::Cost{dsp: 37, lut: 433}),
        (109, fpga::Cost{dsp: 37, lut: 439}),
        (110, fpga::Cost{dsp: 37, lut: 442}),
        (111, fpga::Cost{dsp: 37, lut: 445}),
        (112, fpga::Cost{dsp: 49, lut: 803}),
        (113, fpga::Cost{dsp: 49, lut: 809}),
        (114, fpga::Cost{dsp: 49, lut: 815}),
        (115, fpga::Cost{dsp: 49, lut: 821}),
        (116, fpga::Cost{dsp: 49, lut: 827}),
        (117, fpga::Cost{dsp: 49, lut: 833}),
        (118, fpga::Cost{dsp: 49, lut: 838}),
        (119, fpga::Cost{dsp: 49, lut: 840}),
        (120, fpga::Cost{dsp: 49, lut: 865}),
        (121, fpga::Cost{dsp: 49, lut: 893}),
        (122, fpga::Cost{dsp: 49, lut: 935}),
        (123, fpga::Cost{dsp: 50, lut: 857}),
        (124, fpga::Cost{dsp: 50, lut: 859}),
        (125, fpga::Cost{dsp: 50, lut: 865}),
        (126, fpga::Cost{dsp: 50, lut: 869}),
        (127, fpga::Cost{dsp: 50, lut: 873}),
        (128, fpga::Cost{dsp: 50, lut: 879}),
    ]);
    
    let cost = mul_costs.get(&width);

    match cost {
        Some(x) => return *x,
        None => {
            let bit_width = width as f64;
            let t1 = ((bit_width-9.)/17.).ceil();
            let t2 = ((bit_width-9.)/(17.*t1-5.)).floor();
            return fpga::Cost{dsp: ((t1).powf(2.0) + t2) as i32, lut: width * 6};    
        }
    }
}

pub fn alpha(val : f64) -> f64 {
    static mut ALPHA : f64 = 0.0;
    unsafe {
        if 0.0 <= val && val <= 1.0 {
            ALPHA = val;
        }
        ALPHA
    }
}

pub struct FPGACostFunction<'a> {
    pub egraph: &'a EGraph<BitLanguage, ()>,
    pub seen_nodes: HashSet<String>,
}

// solely used to try and get the cost of a given expression
// doesn't quite work rn because it recounts reused nodes
// it also is weird with the way it finds bw in children nodes
impl<'a> CostFunction<BitLanguage> for FPGACostFunction<'a> {
    type Cost = fpga::Cost;
    fn cost<C>(&mut self, enode: &BitLanguage, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost
    {   
        let op_cost;
        let node_id = enode.to_string();
        // /println!("Costing node: {}", node_id);
        if !self.seen_nodes.contains(&node_id) {
            self.seen_nodes.insert(enode.to_string());
            
            let op = enode.to_string();
            op_cost = match op.as_str() {
                "*" => {
                    if let BitLanguage::Mul([a,b,c]) = enode {
                        for child in [a,b,c] {
                            let node = &self.egraph[*child].nodes[0];
                            if let BitLanguage::Num(x) = node {
                                let cost = mul_cost(*x);
                            return cost;
                        }
                    }                    
                }
                fpga::Cost{dsp: 0, lut: 0}
            },
            "-" => {
                if let BitLanguage::SubW([a,b,c]) = enode {
                    for child in [a,b,c] {
                        let node = &self.egraph[*child].nodes[0];
                        if let BitLanguage::Num(x) = node {
                            return fpga::Cost{dsp: 0, lut: *x};
                        }
                    }
                }
                fpga::Cost{dsp: 0, lut: 0}
            }
            "+" => {
                if let BitLanguage::AddW([a,b,c]) = enode {
                    for child in [a,b,c] {
                        let node = &self.egraph[*child].nodes[0];
                        if let BitLanguage::Num(x) = node {
                            return fpga::Cost{dsp: 0, lut: *x};
                        }
                    }
                }
                fpga::Cost{dsp: 0, lut: 0}
            }
            _ => {
                fpga::Cost{dsp:0, lut:0}
            }
        };
    } else { 
        //println!("Already seen");
        op_cost = fpga::Cost{dsp:0, lut:0};
    }
    enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}

impl<'a> LpCostFunction<BitLanguage, ()> for FPGACostFunction<'a> {
    fn node_cost(&mut self, egraph: &EGraph<BitLanguage, ()>, _eclass: Id, enode: &BitLanguage) -> f64
    {
        let op = enode.to_string();
        let op_cost = match op.as_str() {
            "*" => {
                if let BitLanguage::Mul([a,_b,_c]) = enode {
                    for child in [a,_b,_c] {
                        let node = &egraph[*child].nodes[0];
                        if let BitLanguage::Num(x) = node {
                            let cost = mul_cost(*x);
                            let node_cost = (1.0-alpha(-1.0)) * cost.dsp as f64 + alpha(-1.0) * cost.lut as f64;
                            //println!("DSPs: {}, LUTs: {}. Node cost is: {}", cost.dsp, cost.lut, node_cost);
                            return node_cost;
                        }
                    } 
                }
                0.0
            },
            "-" => {
                if let BitLanguage::SubW([a,_b,_c]) = enode {
                    let node = &egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        let bit_width = *x as f64;
                        return bit_width * alpha(-1.0);
                    }
                }
                0.0
            }
            "+" => {
                if let BitLanguage::AddW([a,_b,_c]) = enode {
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
