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
        (9, fpga::Cost{dsp: 0, lut: 80}),
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
        (20, fpga::Cost{dsp: 1, lut: 75}),
        (21, fpga::Cost{dsp: 2, lut: 0}),
        (22, fpga::Cost{dsp: 2, lut: 0}),
        (23, fpga::Cost{dsp: 2, lut: 0}),
        (24, fpga::Cost{dsp: 2, lut: 0}),
        (25, fpga::Cost{dsp: 2, lut: 0}),
        (26, fpga::Cost{dsp: 2, lut: 0}),
        (27, fpga::Cost{dsp: 4, lut: 37}),
        (28, fpga::Cost{dsp: 4, lut: 39}),
        (29, fpga::Cost{dsp: 4, lut: 41}),
        (30, fpga::Cost{dsp: 4, lut: 43}),
        (31, fpga::Cost{dsp: 4, lut: 45}),
        (32, fpga::Cost{dsp: 4, lut: 47}),
        (33, fpga::Cost{dsp: 4, lut: 49}),
        (34, fpga::Cost{dsp: 4, lut: 51}),
        (35, fpga::Cost{dsp: 4, lut: 63}),
        (36, fpga::Cost{dsp: 4, lut: 83}),
        (37, fpga::Cost{dsp: 4, lut: 118}),
        (38, fpga::Cost{dsp: 5, lut: 70}),
        (39, fpga::Cost{dsp: 5, lut: 74}),
        (40, fpga::Cost{dsp: 5, lut: 78}),
        (41, fpga::Cost{dsp: 5, lut: 82}),
        (42, fpga::Cost{dsp: 5, lut: 85}),
        (43, fpga::Cost{dsp: 5, lut: 87}),
        (44, fpga::Cost{dsp: 9, lut: 72}),
        (45, fpga::Cost{dsp: 9, lut: 74}),
        (46, fpga::Cost{dsp: 9, lut: 76}),
        (47, fpga::Cost{dsp: 9, lut: 78}),
        (48, fpga::Cost{dsp: 9, lut: 80}),
        (49, fpga::Cost{dsp: 9, lut: 82}),
        (50, fpga::Cost{dsp: 9, lut: 84}),
        (51, fpga::Cost{dsp: 9, lut: 86}),
        (52, fpga::Cost{dsp: 9, lut: 88}),
        (53, fpga::Cost{dsp: 9, lut: 110}),
        (54, fpga::Cost{dsp: 9, lut: 144}),
        (55, fpga::Cost{dsp: 10, lut: 94}),
        (56, fpga::Cost{dsp: 10, lut: 96}),
        (57, fpga::Cost{dsp: 10, lut: 98}),
        (58, fpga::Cost{dsp: 10, lut: 100}),
        (59, fpga::Cost{dsp: 10, lut: 102}),
        (60, fpga::Cost{dsp: 10, lut: 104}),
        (61, fpga::Cost{dsp: 16, lut: 155}),
        (62, fpga::Cost{dsp: 16, lut: 157}),
        (63, fpga::Cost{dsp: 16, lut: 159}),
        (64, fpga::Cost{dsp: 16, lut: 161}),
        (65, fpga::Cost{dsp: 16, lut: 162}),
        (66, fpga::Cost{dsp: 16, lut: 164}),
        (67, fpga::Cost{dsp: 16, lut: 167}),
        (68, fpga::Cost{dsp: 16, lut: 168}),
        (69, fpga::Cost{dsp: 16, lut: 171}),
        (70, fpga::Cost{dsp: 16, lut: 193}),
        (71, fpga::Cost{dsp: 16, lut: 228}),
        (72, fpga::Cost{dsp: 17, lut: 177}),
        (73, fpga::Cost{dsp: 17, lut: 179}),
        (74, fpga::Cost{dsp: 17, lut: 181}),
        (75, fpga::Cost{dsp: 17, lut: 183}),
        (76, fpga::Cost{dsp: 17, lut: 185}),
        (77, fpga::Cost{dsp: 17, lut: 187}),
        (78, fpga::Cost{dsp: 25, lut: 279}),
        (79, fpga::Cost{dsp: 25, lut: 283}),
        (80, fpga::Cost{dsp: 25, lut: 287}),
        (81, fpga::Cost{dsp: 25, lut: 291}),
        (82, fpga::Cost{dsp: 25, lut: 295}),
        (83, fpga::Cost{dsp: 25, lut: 299}),
        (84, fpga::Cost{dsp: 25, lut: 302}),
        (85, fpga::Cost{dsp: 25, lut: 304}),
        (86, fpga::Cost{dsp: 25, lut: 324}),
        (87, fpga::Cost{dsp: 25, lut: 349}),
        (88, fpga::Cost{dsp: 25, lut: 386}),
        (89, fpga::Cost{dsp: 26, lut: 312}),
        (90, fpga::Cost{dsp: 26, lut: 314}),
        (91, fpga::Cost{dsp: 26, lut: 316}),
        (92, fpga::Cost{dsp: 26, lut: 318}),
        (93, fpga::Cost{dsp: 26, lut: 320}),
        (94, fpga::Cost{dsp: 26, lut: 322}),
        (95, fpga::Cost{dsp: 36, lut: 381}),
        (96, fpga::Cost{dsp: 36, lut: 385}),
        (97, fpga::Cost{dsp: 36, lut: 389}),
        (98, fpga::Cost{dsp: 36, lut: 393}),
        (99, fpga::Cost{dsp: 36, lut: 397}),
        (100, fpga::Cost{dsp: 36, lut: 401}),
        (101, fpga::Cost{dsp: 36, lut: 405}),
        (102, fpga::Cost{dsp: 36, lut: 409}),
        (103, fpga::Cost{dsp: 36, lut: 440}),
        (104, fpga::Cost{dsp: 36, lut: 458}),
        (105, fpga::Cost{dsp: 36, lut: 496}),
        (106, fpga::Cost{dsp: 37, lut: 449}),
        (107, fpga::Cost{dsp: 37, lut: 455}),
        (108, fpga::Cost{dsp: 37, lut: 461}),
        (109, fpga::Cost{dsp: 37, lut: 468}),
        (110, fpga::Cost{dsp: 37, lut: 471}),
        (111, fpga::Cost{dsp: 37, lut: 474}),
        (112, fpga::Cost{dsp: 49, lut: 616}),
        (113, fpga::Cost{dsp: 49, lut: 622}),
        (114, fpga::Cost{dsp: 49, lut: 628}),
        (115, fpga::Cost{dsp: 49, lut: 634}),
        (116, fpga::Cost{dsp: 49, lut: 640}),
        (117, fpga::Cost{dsp: 49, lut: 646}),
        (118, fpga::Cost{dsp: 49, lut: 651}),
        (119, fpga::Cost{dsp: 49, lut: 655}),
        (120, fpga::Cost{dsp: 49, lut: 677}),
        (121, fpga::Cost{dsp: 49, lut: 704}),
        (122, fpga::Cost{dsp: 49, lut: 744}),
        (123, fpga::Cost{dsp: 50, lut: 671}),
        (124, fpga::Cost{dsp: 50, lut: 675}),
        (125, fpga::Cost{dsp: 50, lut: 679}),
        (126, fpga::Cost{dsp: 50, lut: 683}),
        (127, fpga::Cost{dsp: 50, lut: 687}),
        (128, fpga::Cost{dsp: 50, lut: 691}),
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
