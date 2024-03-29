use egg::*;
use std::collections::{HashMap,HashSet};
use crate::utils::{fpga, language::*};
use crate::EGraphVerilogGeneration;

// this is cringe, i dont know how to avoid unsafe {} for global static mutables
// it seems like a bad idea to have an interface like this because caller doesnt know its unsafe inside
// this just removes the globalness of ALPHA - but the function is basically global anyway
pub fn alpha(val : Option<f64>) -> f64 {
    static mut ALPHA : f64 = 0.0;
    match val {
        Some(x) => {
            unsafe {
                if 0.0 <= x && x <= 1.0 {
                    ALPHA = x;
                }
                ALPHA
            }
        }
        None => unsafe {
            ALPHA
        },
    }
}

pub struct FPGACostFunction<'a> {
    pub egraph: &'a EGraph<BitLanguage, ()>,
    pub seen_nodes: HashSet<String>,
}

// this is useless basically
impl<'a> CostFunction<BitLanguage> for FPGACostFunction<'a> {
    type Cost = fpga::Cost;
    fn cost<C>(&mut self, enode: &BitLanguage, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost
    {   
        let op_cost;
        let node_id = enode.to_string();
        if !self.seen_nodes.contains(&node_id) {
            self.seen_nodes.insert(enode.to_string());
            
            let op = enode.to_string();
            op_cost = match op.as_str() {
                "*" => {
                    if let BitLanguage::Mul([a,_b,_c]) = enode {
                        let node = &self.egraph[*a].nodes[0];
                        if let BitLanguage::Num(x) = node {
                            let cost = mul_cost(*x);
                            return cost;
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
                    let node = &egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        let cost = mul_cost(*x);
                        let node_cost = (1.0-alpha(None)) * cost.dsp as f64 + alpha(None) * cost.lut as f64;
                        return node_cost;
                    }
                } else if let BitLanguage::Mul4([a,b,_c,_d]) = enode {
                    let node_a = &self.egraph[*a].nodes[0];
                    let node_b = &self.egraph[*b].nodes[0];
                    if let BitLanguage::Num(x) = node_a {
                        if let BitLanguage::Num(y) = node_b {
                            let cost = mul_cost_2(*x,*y);
                            let node_cost = (1.0-alpha(None)) * cost.dsp as f64 + alpha(None) * cost.lut as f64;
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
                        return bit_width * alpha(None);
                    }
                }
                0.0
            }
            "+" => {
                if let BitLanguage::AddW([a,_b,_c]) = enode {
                    let node = &egraph[*a].nodes[0];
                    if let BitLanguage::Num(x) = node {
                        let bit_width = *x as f64;
                        return bit_width * alpha(None);
                    }
                }
                0.0
            }
            _ => 0.0,
        };
        op_cost
    }
}

// smell from repeat of implemented CostFunction
// should really have one version of node costing and smaller versions of the functions above :///
pub fn cost_node(enode: &BitLanguage,
                    egraph: &EGraphVerilogGeneration)
                 -> fpga::Cost {
    match enode {
        BitLanguage::AddW([a,_b,_c]) | BitLanguage::SubW([a,_b,_c]) => {
            let node = &egraph[*a].nodes[0];
            if let BitLanguage::Num(x) = node {
                return fpga::Cost{dsp:0, lut: *x};
            }
            fpga::Cost{dsp:0, lut: 0}
        }
        BitLanguage::Mul([a,_b,_c]) => {
            let node = &egraph[*a].nodes[0];
            if let BitLanguage::Num(x) = node {
                return mul_cost(*x);
            }
            fpga::Cost{dsp:0, lut:0}
        }
        BitLanguage::Mul4([a,b,_c,_d]) => {
            let a_node = &egraph[*a].nodes[0];
            let b_node = &egraph[*b].nodes[0];

            if let BitLanguage::Num(x) = a_node {
                if let BitLanguage::Num(y) = b_node {
                    return mul_cost_2(*x, *y);
                }
            }
            return fpga::Cost{dsp: 0, lut: 0};
        }
        _ => fpga::Cost{dsp:0, lut: 0}
    }

}

// 
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
            return fpga::Cost{dsp: ((t1).powf(2.0) + t2) as i32, lut: (0.05571*bit_width*bit_width - 1.58571*bit_width + 28.258) as i32};    
        }
    }
}

pub fn mul_cost_2(width_1 : i32, width_2 : i32) -> fpga::Cost {
    let mul_costs : HashMap<(i32,i32), fpga::Cost> = HashMap::from([
        ((1,2), fpga::Cost{dsp: 0, lut: 1}),
        ((1,3), fpga::Cost{dsp: 0, lut: 2}),
        ((1,4), fpga::Cost{dsp: 0, lut: 2}),
        ((1,5), fpga::Cost{dsp: 0, lut: 3}),
        ((1,6), fpga::Cost{dsp: 0, lut: 3}),
        ((1,7), fpga::Cost{dsp: 0, lut: 4}),
        ((1,8), fpga::Cost{dsp: 0, lut: 4}),
        ((1,9), fpga::Cost{dsp: 0, lut: 5}),
        ((1,10), fpga::Cost{dsp: 0, lut: 5}),
        ((1,11), fpga::Cost{dsp: 0, lut: 6}),
        ((1,12), fpga::Cost{dsp: 0, lut: 6}),
        ((1,13), fpga::Cost{dsp: 0, lut: 7}),
        ((1,14), fpga::Cost{dsp: 0, lut: 7}),
        ((1,15), fpga::Cost{dsp: 0, lut: 8}),
        ((1,16), fpga::Cost{dsp: 0, lut: 8}),
        ((1,17), fpga::Cost{dsp: 0, lut: 9}),
        ((1,18), fpga::Cost{dsp: 0, lut: 9}),
        ((1,19), fpga::Cost{dsp: 0, lut: 10}),
        ((1,20), fpga::Cost{dsp: 0, lut: 10}),
        ((1,21), fpga::Cost{dsp: 0, lut: 11}),
        ((1,22), fpga::Cost{dsp: 0, lut: 11}),
        ((1,23), fpga::Cost{dsp: 0, lut: 12}),
        ((1,24), fpga::Cost{dsp: 0, lut: 12}),
        ((1,25), fpga::Cost{dsp: 0, lut: 13}),
        ((1,26), fpga::Cost{dsp: 0, lut: 13}),
        ((1,27), fpga::Cost{dsp: 0, lut: 14}),
        ((1,28), fpga::Cost{dsp: 0, lut: 14}),
        ((1,29), fpga::Cost{dsp: 0, lut: 15}),
        ((1,30), fpga::Cost{dsp: 0, lut: 15}),
        ((1,31), fpga::Cost{dsp: 0, lut: 16}),
        ((1,32), fpga::Cost{dsp: 0, lut: 16}),
        ((2,2), fpga::Cost{dsp: 0, lut: 2}),
        ((2,3), fpga::Cost{dsp: 0, lut: 3}),
        ((2,4), fpga::Cost{dsp: 0, lut: 5}),
        ((2,5), fpga::Cost{dsp: 0, lut: 6}),
        ((2,6), fpga::Cost{dsp: 0, lut: 8}),
        ((2,7), fpga::Cost{dsp: 0, lut: 9}),
        ((2,8), fpga::Cost{dsp: 0, lut: 11}),
        ((2,9), fpga::Cost{dsp: 0, lut: 10}),
        ((2,10), fpga::Cost{dsp: 0, lut: 11}),
        ((2,11), fpga::Cost{dsp: 0, lut: 12}),
        ((2,12), fpga::Cost{dsp: 0, lut: 13}),
        ((2,13), fpga::Cost{dsp: 0, lut: 14}),
        ((2,14), fpga::Cost{dsp: 0, lut: 15}),
        ((2,15), fpga::Cost{dsp: 0, lut: 16}),
        ((2,16), fpga::Cost{dsp: 0, lut: 17}),
        ((2,17), fpga::Cost{dsp: 0, lut: 18}),
        ((2,18), fpga::Cost{dsp: 0, lut: 19}),
        ((2,19), fpga::Cost{dsp: 0, lut: 20}),
        ((2,20), fpga::Cost{dsp: 0, lut: 21}),
        ((2,21), fpga::Cost{dsp: 0, lut: 22}),
        ((2,22), fpga::Cost{dsp: 0, lut: 23}),
        ((2,23), fpga::Cost{dsp: 0, lut: 24}),
        ((2,24), fpga::Cost{dsp: 0, lut: 25}),
        ((2,25), fpga::Cost{dsp: 0, lut: 26}),
        ((2,26), fpga::Cost{dsp: 0, lut: 27}),
        ((2,27), fpga::Cost{dsp: 0, lut: 30}),
        ((2,28), fpga::Cost{dsp: 0, lut: 31}),
        ((2,29), fpga::Cost{dsp: 0, lut: 33}),
        ((2,30), fpga::Cost{dsp: 0, lut: 33}),
        ((2,31), fpga::Cost{dsp: 0, lut: 49}),
        ((2,32), fpga::Cost{dsp: 0, lut: 51}),
        ((3,3), fpga::Cost{dsp: 0, lut: 5}),
        ((3,4), fpga::Cost{dsp: 0, lut: 10}),
        ((3,5), fpga::Cost{dsp: 0, lut: 19}),
        ((3,6), fpga::Cost{dsp: 0, lut: 18}),
        ((3,7), fpga::Cost{dsp: 0, lut: 17}),
        ((3,8), fpga::Cost{dsp: 0, lut: 20}),
        ((3,9), fpga::Cost{dsp: 0, lut: 22}),
        ((3,10), fpga::Cost{dsp: 0, lut: 25}),
        ((3,11), fpga::Cost{dsp: 0, lut: 27}),
        ((3,12), fpga::Cost{dsp: 0, lut: 30}),
        ((3,13), fpga::Cost{dsp: 0, lut: 32}),
        ((3,14), fpga::Cost{dsp: 0, lut: 35}),
        ((3,15), fpga::Cost{dsp: 0, lut: 37}),
        ((3,16), fpga::Cost{dsp: 0, lut: 40}),
        ((3,17), fpga::Cost{dsp: 0, lut: 42}),
        ((3,18), fpga::Cost{dsp: 0, lut: 45}),
        ((3,19), fpga::Cost{dsp: 0, lut: 47}),
        ((3,20), fpga::Cost{dsp: 0, lut: 50}),
        ((3,21), fpga::Cost{dsp: 0, lut: 52}),
        ((3,22), fpga::Cost{dsp: 0, lut: 55}),
        ((3,23), fpga::Cost{dsp: 0, lut: 57}),
        ((3,24), fpga::Cost{dsp: 0, lut: 60}),
        ((3,25), fpga::Cost{dsp: 0, lut: 62}),
        ((3,26), fpga::Cost{dsp: 0, lut: 65}),
        ((3,27), fpga::Cost{dsp: 0, lut: 69}),
        ((3,28), fpga::Cost{dsp: 0, lut: 79}),
        ((3,29), fpga::Cost{dsp: 0, lut: 83}),
        ((3,30), fpga::Cost{dsp: 0, lut: 99}),
        ((3,31), fpga::Cost{dsp: 0, lut: 105}),
        ((3,32), fpga::Cost{dsp: 0, lut: 108}),
        ((4,4), fpga::Cost{dsp: 0, lut: 18}),
        ((4,5), fpga::Cost{dsp: 0, lut: 18}),
        ((4,6), fpga::Cost{dsp: 0, lut: 26}),
        ((4,7), fpga::Cost{dsp: 0, lut: 30}),
        ((4,8), fpga::Cost{dsp: 0, lut: 34}),
        ((4,9), fpga::Cost{dsp: 0, lut: 38}),
        ((4,10), fpga::Cost{dsp: 0, lut: 42}),
        ((4,11), fpga::Cost{dsp: 0, lut: 44}),
        ((4,12), fpga::Cost{dsp: 0, lut: 48}),
        ((4,13), fpga::Cost{dsp: 0, lut: 52}),
        ((4,14), fpga::Cost{dsp: 0, lut: 56}),
        ((4,15), fpga::Cost{dsp: 0, lut: 60}),
        ((4,16), fpga::Cost{dsp: 0, lut: 64}),
        ((4,17), fpga::Cost{dsp: 0, lut: 68}),
        ((4,18), fpga::Cost{dsp: 0, lut: 72}),
        ((4,19), fpga::Cost{dsp: 0, lut: 76}),
        ((4,20), fpga::Cost{dsp: 0, lut: 80}),
        ((4,21), fpga::Cost{dsp: 0, lut: 84}),
        ((4,22), fpga::Cost{dsp: 0, lut: 88}),
        ((4,23), fpga::Cost{dsp: 0, lut: 92}),
        ((4,24), fpga::Cost{dsp: 0, lut: 96}),
        ((4,25), fpga::Cost{dsp: 0, lut: 100}),
        ((4,26), fpga::Cost{dsp: 0, lut: 104}),
        ((4,27), fpga::Cost{dsp: 0, lut: 109}),
        ((4,28), fpga::Cost{dsp: 1, lut: 12}),
        ((4,29), fpga::Cost{dsp: 1, lut: 19}),
        ((4,30), fpga::Cost{dsp: 1, lut: 26}),
        ((4,31), fpga::Cost{dsp: 2, lut: 0}),
        ((4,32), fpga::Cost{dsp: 2, lut: 0}),
        ((4,33), fpga::Cost{dsp: 2, lut: 0}),
        ((5,5), fpga::Cost{dsp: 0, lut: 29}),
        ((5,6), fpga::Cost{dsp: 0, lut: 31}),
        ((5,7), fpga::Cost{dsp: 0, lut: 32}),
        ((5,8), fpga::Cost{dsp: 0, lut: 36}),
        ((5,9), fpga::Cost{dsp: 0, lut: 41}),
        ((5,10), fpga::Cost{dsp: 1, lut: 0}),
        ((5,11), fpga::Cost{dsp: 1, lut: 0}),
        ((5,12), fpga::Cost{dsp: 1, lut: 0}),
        ((5,13), fpga::Cost{dsp: 1, lut: 0}),
        ((5,14), fpga::Cost{dsp: 1, lut: 0}),
        ((5,15), fpga::Cost{dsp: 1, lut: 0}),
        ((5,16), fpga::Cost{dsp: 1, lut: 0}),
        ((5,17), fpga::Cost{dsp: 1, lut: 0}),
        ((5,18), fpga::Cost{dsp: 1, lut: 0}),
        ((5,19), fpga::Cost{dsp: 1, lut: 0}),
        ((5,20), fpga::Cost{dsp: 1, lut: 0}),
        ((5,21), fpga::Cost{dsp: 1, lut: 0}),
        ((5,22), fpga::Cost{dsp: 1, lut: 0}),
        ((5,23), fpga::Cost{dsp: 1, lut: 0}),
        ((5,24), fpga::Cost{dsp: 1, lut: 0}),
        ((5,25), fpga::Cost{dsp: 1, lut: 0}),
        ((5,26), fpga::Cost{dsp: 1, lut: 0}),
        ((5,27), fpga::Cost{dsp: 1, lut: 0}),
        ((5,28), fpga::Cost{dsp: 1, lut: 15}),
        ((5,29), fpga::Cost{dsp: 1, lut: 25}),
        ((5,30), fpga::Cost{dsp: 1, lut: 28}),
        ((5,31), fpga::Cost{dsp: 2, lut: 0}),
        ((5,32), fpga::Cost{dsp: 2, lut: 0}),
        ((5,33), fpga::Cost{dsp: 2, lut: 0}),
        ((6,6), fpga::Cost{dsp: 0, lut: 43}),
        ((6,7), fpga::Cost{dsp: 0, lut: 44}),
        ((6,8), fpga::Cost{dsp: 0, lut: 50}),
        ((6,9), fpga::Cost{dsp: 0, lut: 56}),
        ((6,10), fpga::Cost{dsp: 1, lut: 0}),
        ((6,11), fpga::Cost{dsp: 1, lut: 0}),
        ((6,12), fpga::Cost{dsp: 1, lut: 0}),
        ((6,13), fpga::Cost{dsp: 1, lut: 0}),
        ((6,14), fpga::Cost{dsp: 1, lut: 0}),
        ((6,15), fpga::Cost{dsp: 1, lut: 0}),
        ((6,16), fpga::Cost{dsp: 1, lut: 0}),
        ((6,17), fpga::Cost{dsp: 1, lut: 0}),
        ((6,18), fpga::Cost{dsp: 1, lut: 0}),
        ((6,19), fpga::Cost{dsp: 1, lut: 0}),
        ((6,20), fpga::Cost{dsp: 1, lut: 0}),
        ((6,21), fpga::Cost{dsp: 1, lut: 0}),
        ((6,22), fpga::Cost{dsp: 1, lut: 0}),
        ((6,23), fpga::Cost{dsp: 1, lut: 0}),
        ((6,24), fpga::Cost{dsp: 1, lut: 0}),
        ((6,25), fpga::Cost{dsp: 1, lut: 0}),
        ((6,26), fpga::Cost{dsp: 1, lut: 0}),
        ((6,27), fpga::Cost{dsp: 1, lut: 0}),
        ((6,28), fpga::Cost{dsp: 1, lut: 17}),
        ((6,29), fpga::Cost{dsp: 1, lut: 22}),
        ((6,30), fpga::Cost{dsp: 1, lut: 33}),
        ((6,31), fpga::Cost{dsp: 2, lut: 0}),
        ((6,32), fpga::Cost{dsp: 2, lut: 0}),
        ((6,33), fpga::Cost{dsp: 2, lut: 0}),
        ((7,7), fpga::Cost{dsp: 0, lut: 52}),
        ((7,8), fpga::Cost{dsp: 0, lut: 59}),
        ((7,9), fpga::Cost{dsp: 0, lut: 67}),
        ((7,10), fpga::Cost{dsp: 1, lut: 0}),
        ((7,11), fpga::Cost{dsp: 1, lut: 0}),
        ((7,12), fpga::Cost{dsp: 1, lut: 0}),
        ((7,13), fpga::Cost{dsp: 1, lut: 0}),
        ((7,14), fpga::Cost{dsp: 1, lut: 0}),
        ((7,15), fpga::Cost{dsp: 1, lut: 0}),
        ((7,16), fpga::Cost{dsp: 1, lut: 0}),
        ((7,17), fpga::Cost{dsp: 1, lut: 0}),
        ((7,18), fpga::Cost{dsp: 1, lut: 0}),
        ((7,19), fpga::Cost{dsp: 1, lut: 0}),
        ((7,20), fpga::Cost{dsp: 1, lut: 0}),
        ((7,21), fpga::Cost{dsp: 1, lut: 0}),
        ((7,22), fpga::Cost{dsp: 1, lut: 0}),
        ((7,23), fpga::Cost{dsp: 1, lut: 0}),
        ((7,24), fpga::Cost{dsp: 1, lut: 0}),
        ((7,25), fpga::Cost{dsp: 1, lut: 0}),
        ((7,26), fpga::Cost{dsp: 1, lut: 0}),
        ((7,27), fpga::Cost{dsp: 1, lut: 0}),
        ((7,28), fpga::Cost{dsp: 1, lut: 13}),
        ((7,29), fpga::Cost{dsp: 1, lut: 28}),
        ((7,30), fpga::Cost{dsp: 1, lut: 38}),
        ((7,31), fpga::Cost{dsp: 2, lut: 0}),
        ((7,32), fpga::Cost{dsp: 2, lut: 0}),
        ((7,33), fpga::Cost{dsp: 2, lut: 0}),
        ((8,8), fpga::Cost{dsp: 0, lut: 71}),
        ((8,9), fpga::Cost{dsp: 0, lut: 80}),
        ((8,10), fpga::Cost{dsp: 1, lut: 0}),
        ((8,11), fpga::Cost{dsp: 1, lut: 0}),
        ((8,12), fpga::Cost{dsp: 1, lut: 0}),
        ((8,13), fpga::Cost{dsp: 1, lut: 0}),
        ((8,14), fpga::Cost{dsp: 1, lut: 0}),
        ((8,15), fpga::Cost{dsp: 1, lut: 0}),
        ((8,16), fpga::Cost{dsp: 1, lut: 0}),
        ((8,17), fpga::Cost{dsp: 1, lut: 0}),
        ((8,18), fpga::Cost{dsp: 1, lut: 0}),
        ((8,19), fpga::Cost{dsp: 1, lut: 0}),
        ((8,20), fpga::Cost{dsp: 1, lut: 0}),
        ((8,21), fpga::Cost{dsp: 1, lut: 0}),
        ((8,22), fpga::Cost{dsp: 1, lut: 0}),
        ((8,23), fpga::Cost{dsp: 1, lut: 0}),
        ((8,24), fpga::Cost{dsp: 1, lut: 0}),
        ((8,25), fpga::Cost{dsp: 1, lut: 0}),
        ((8,26), fpga::Cost{dsp: 1, lut: 0}),
        ((8,27), fpga::Cost{dsp: 1, lut: 0}),
        ((8,28), fpga::Cost{dsp: 1, lut: 15}),
        ((8,29), fpga::Cost{dsp: 1, lut: 32}),
        ((8,30), fpga::Cost{dsp: 1, lut: 43}),
        ((8,31), fpga::Cost{dsp: 2, lut: 0}),
        ((8,32), fpga::Cost{dsp: 2, lut: 0}),
        ((8,33), fpga::Cost{dsp: 2, lut: 0}),
        ((9,9), fpga::Cost{dsp: 0, lut: 82}),
        ((9,10), fpga::Cost{dsp: 1, lut: 0}),
        ((9,11), fpga::Cost{dsp: 1, lut: 0}),
        ((9,12), fpga::Cost{dsp: 1, lut: 0}),
        ((9,13), fpga::Cost{dsp: 1, lut: 0}),
        ((9,14), fpga::Cost{dsp: 1, lut: 0}),
        ((9,15), fpga::Cost{dsp: 1, lut: 0}),
        ((9,16), fpga::Cost{dsp: 1, lut: 0}),
        ((9,17), fpga::Cost{dsp: 1, lut: 0}),
        ((9,18), fpga::Cost{dsp: 1, lut: 0}),
        ((9,19), fpga::Cost{dsp: 1, lut: 0}),
        ((9,20), fpga::Cost{dsp: 1, lut: 0}),
        ((9,21), fpga::Cost{dsp: 1, lut: 0}),
        ((9,22), fpga::Cost{dsp: 1, lut: 0}),
        ((9,23), fpga::Cost{dsp: 1, lut: 0}),
        ((9,24), fpga::Cost{dsp: 1, lut: 0}),
        ((9,25), fpga::Cost{dsp: 1, lut: 0}),
        ((9,26), fpga::Cost{dsp: 1, lut: 0}),
        ((9,27), fpga::Cost{dsp: 1, lut: 0}),
        ((9,28), fpga::Cost{dsp: 1, lut: 21}),
        ((9,29), fpga::Cost{dsp: 1, lut: 35}),
        ((9,30), fpga::Cost{dsp: 2, lut: 0}),
        ((9,31), fpga::Cost{dsp: 2, lut: 0}),
        ((9,32), fpga::Cost{dsp: 2, lut: 0}),
        ((9,33), fpga::Cost{dsp: 2, lut: 0}),
        ((10,10), fpga::Cost{dsp: 1, lut: 0}),
        ((10,11), fpga::Cost{dsp: 1, lut: 0}),
        ((10,12), fpga::Cost{dsp: 1, lut: 0}),
        ((10,13), fpga::Cost{dsp: 1, lut: 0}),
        ((10,14), fpga::Cost{dsp: 1, lut: 0}),
        ((10,15), fpga::Cost{dsp: 1, lut: 0}),
        ((10,16), fpga::Cost{dsp: 1, lut: 0}),
        ((10,17), fpga::Cost{dsp: 1, lut: 0}),
        ((10,18), fpga::Cost{dsp: 1, lut: 0}),
        ((10,19), fpga::Cost{dsp: 1, lut: 0}),
        ((10,20), fpga::Cost{dsp: 1, lut: 0}),
        ((10,21), fpga::Cost{dsp: 1, lut: 0}),
        ((10,22), fpga::Cost{dsp: 1, lut: 0}),
        ((10,23), fpga::Cost{dsp: 1, lut: 0}),
        ((10,24), fpga::Cost{dsp: 1, lut: 0}),
        ((10,25), fpga::Cost{dsp: 1, lut: 0}),
        ((10,26), fpga::Cost{dsp: 1, lut: 0}),
        ((10,27), fpga::Cost{dsp: 1, lut: 0}),
        ((10,28), fpga::Cost{dsp: 1, lut: 23}),
        ((10,29), fpga::Cost{dsp: 1, lut: 39}),
        ((10,30), fpga::Cost{dsp: 2, lut: 0}),
        ((10,31), fpga::Cost{dsp: 2, lut: 0}),
        ((10,32), fpga::Cost{dsp: 2, lut: 0}),
        ((10,33), fpga::Cost{dsp: 2, lut: 0}),
        ((11,11), fpga::Cost{dsp: 1, lut: 0}),
        ((11,12), fpga::Cost{dsp: 1, lut: 0}),
        ((11,13), fpga::Cost{dsp: 1, lut: 0}),
        ((11,14), fpga::Cost{dsp: 1, lut: 0}),
        ((11,15), fpga::Cost{dsp: 1, lut: 0}),
        ((11,16), fpga::Cost{dsp: 1, lut: 0}),
        ((11,17), fpga::Cost{dsp: 1, lut: 0}),
        ((11,18), fpga::Cost{dsp: 1, lut: 0}),
        ((11,19), fpga::Cost{dsp: 1, lut: 0}),
        ((11,20), fpga::Cost{dsp: 1, lut: 0}),
        ((11,21), fpga::Cost{dsp: 1, lut: 0}),
        ((11,22), fpga::Cost{dsp: 1, lut: 0}),
        ((11,23), fpga::Cost{dsp: 1, lut: 0}),
        ((11,24), fpga::Cost{dsp: 1, lut: 0}),
        ((11,25), fpga::Cost{dsp: 1, lut: 0}),
        ((11,26), fpga::Cost{dsp: 1, lut: 0}),
        ((11,27), fpga::Cost{dsp: 1, lut: 0}),
        ((11,28), fpga::Cost{dsp: 1, lut: 25}),
        ((11,29), fpga::Cost{dsp: 1, lut: 42}),
        ((11,30), fpga::Cost{dsp: 2, lut: 0}),
        ((11,31), fpga::Cost{dsp: 2, lut: 0}),
        ((11,32), fpga::Cost{dsp: 2, lut: 0}),
        ((11,33), fpga::Cost{dsp: 2, lut: 0}),
        ((12,12), fpga::Cost{dsp: 1, lut: 0}),
        ((12,13), fpga::Cost{dsp: 1, lut: 0}),
        ((12,14), fpga::Cost{dsp: 1, lut: 0}),
        ((12,15), fpga::Cost{dsp: 1, lut: 0}),
        ((12,16), fpga::Cost{dsp: 1, lut: 0}),
        ((12,17), fpga::Cost{dsp: 1, lut: 0}),
        ((12,18), fpga::Cost{dsp: 1, lut: 0}),
        ((12,19), fpga::Cost{dsp: 1, lut: 0}),
        ((12,20), fpga::Cost{dsp: 1, lut: 0}),
        ((12,21), fpga::Cost{dsp: 1, lut: 0}),
        ((12,22), fpga::Cost{dsp: 1, lut: 0}),
        ((12,23), fpga::Cost{dsp: 1, lut: 0}),
        ((12,24), fpga::Cost{dsp: 1, lut: 0}),
        ((12,25), fpga::Cost{dsp: 1, lut: 0}),
        ((12,26), fpga::Cost{dsp: 1, lut: 0}),
        ((12,27), fpga::Cost{dsp: 1, lut: 0}),
        ((12,28), fpga::Cost{dsp: 1, lut: 27}),
        ((12,29), fpga::Cost{dsp: 1, lut: 50}),
        ((12,30), fpga::Cost{dsp: 2, lut: 0}),
        ((12,31), fpga::Cost{dsp: 2, lut: 0}),
        ((12,32), fpga::Cost{dsp: 2, lut: 0}),
        ((12,33), fpga::Cost{dsp: 2, lut: 0}),
        ((13,13), fpga::Cost{dsp: 1, lut: 0}),
        ((13,14), fpga::Cost{dsp: 1, lut: 0}),
        ((13,15), fpga::Cost{dsp: 1, lut: 0}),
        ((13,16), fpga::Cost{dsp: 1, lut: 0}),
        ((13,17), fpga::Cost{dsp: 1, lut: 0}),
        ((13,18), fpga::Cost{dsp: 1, lut: 0}),
        ((13,19), fpga::Cost{dsp: 1, lut: 0}),
        ((13,20), fpga::Cost{dsp: 1, lut: 0}),
        ((13,21), fpga::Cost{dsp: 1, lut: 0}),
        ((13,22), fpga::Cost{dsp: 1, lut: 0}),
        ((13,23), fpga::Cost{dsp: 1, lut: 0}),
        ((13,24), fpga::Cost{dsp: 1, lut: 0}),
        ((13,25), fpga::Cost{dsp: 1, lut: 0}),
        ((13,26), fpga::Cost{dsp: 1, lut: 0}),
        ((13,27), fpga::Cost{dsp: 1, lut: 0}),
        ((13,28), fpga::Cost{dsp: 1, lut: 29}),
        ((13,29), fpga::Cost{dsp: 1, lut: 53}),
        ((13,30), fpga::Cost{dsp: 2, lut: 0}),
        ((13,31), fpga::Cost{dsp: 2, lut: 0}),
        ((13,32), fpga::Cost{dsp: 2, lut: 0}),
        ((13,33), fpga::Cost{dsp: 2, lut: 0}),
        ((14,14), fpga::Cost{dsp: 1, lut: 0}),
        ((14,15), fpga::Cost{dsp: 1, lut: 0}),
        ((14,16), fpga::Cost{dsp: 1, lut: 0}),
        ((14,17), fpga::Cost{dsp: 1, lut: 0}),
        ((14,18), fpga::Cost{dsp: 1, lut: 0}),
        ((14,19), fpga::Cost{dsp: 1, lut: 0}),
        ((14,20), fpga::Cost{dsp: 1, lut: 0}),
        ((14,21), fpga::Cost{dsp: 1, lut: 0}),
        ((14,22), fpga::Cost{dsp: 1, lut: 0}),
        ((14,23), fpga::Cost{dsp: 1, lut: 0}),
        ((14,24), fpga::Cost{dsp: 1, lut: 0}),
        ((14,25), fpga::Cost{dsp: 1, lut: 0}),
        ((14,26), fpga::Cost{dsp: 1, lut: 0}),
        ((14,27), fpga::Cost{dsp: 1, lut: 0}),
        ((14,28), fpga::Cost{dsp: 1, lut: 31}),
        ((14,29), fpga::Cost{dsp: 1, lut: 57}),
        ((14,30), fpga::Cost{dsp: 2, lut: 0}),
        ((14,31), fpga::Cost{dsp: 2, lut: 0}),
        ((14,32), fpga::Cost{dsp: 2, lut: 0}),
        ((14,33), fpga::Cost{dsp: 2, lut: 0}),
        ((15,15), fpga::Cost{dsp: 1, lut: 0}),
        ((15,16), fpga::Cost{dsp: 1, lut: 0}),
        ((15,17), fpga::Cost{dsp: 1, lut: 0}),
        ((15,18), fpga::Cost{dsp: 1, lut: 0}),
        ((15,19), fpga::Cost{dsp: 1, lut: 0}),
        ((15,20), fpga::Cost{dsp: 1, lut: 0}),
        ((15,21), fpga::Cost{dsp: 1, lut: 0}),
        ((15,22), fpga::Cost{dsp: 1, lut: 0}),
        ((15,23), fpga::Cost{dsp: 1, lut: 0}),
        ((15,24), fpga::Cost{dsp: 1, lut: 0}),
        ((15,25), fpga::Cost{dsp: 1, lut: 0}),
        ((15,26), fpga::Cost{dsp: 1, lut: 0}),
        ((15,27), fpga::Cost{dsp: 1, lut: 0}),
        ((15,28), fpga::Cost{dsp: 1, lut: 33}),
        ((15,29), fpga::Cost{dsp: 1, lut: 60}),
        ((15,30), fpga::Cost{dsp: 2, lut: 0}),
        ((15,31), fpga::Cost{dsp: 2, lut: 0}),
        ((15,32), fpga::Cost{dsp: 2, lut: 0}),
        ((15,33), fpga::Cost{dsp: 2, lut: 0}),
        ((16,16), fpga::Cost{dsp: 1, lut: 0}),
        ((16,17), fpga::Cost{dsp: 1, lut: 0}),
        ((16,18), fpga::Cost{dsp: 1, lut: 0}),
        ((16,19), fpga::Cost{dsp: 1, lut: 0}),
        ((16,20), fpga::Cost{dsp: 1, lut: 0}),
        ((16,21), fpga::Cost{dsp: 1, lut: 0}),
        ((16,22), fpga::Cost{dsp: 1, lut: 0}),
        ((16,23), fpga::Cost{dsp: 1, lut: 0}),
        ((16,24), fpga::Cost{dsp: 1, lut: 0}),
        ((16,25), fpga::Cost{dsp: 1, lut: 0}),
        ((16,26), fpga::Cost{dsp: 1, lut: 0}),
        ((16,27), fpga::Cost{dsp: 1, lut: 0}),
        ((16,28), fpga::Cost{dsp: 1, lut: 35}),
        ((16,29), fpga::Cost{dsp: 1, lut: 64}),
        ((16,30), fpga::Cost{dsp: 2, lut: 0}),
        ((16,31), fpga::Cost{dsp: 2, lut: 0}),
        ((16,32), fpga::Cost{dsp: 2, lut: 0}),
        ((16,33), fpga::Cost{dsp: 2, lut: 0}),
        ((17,17), fpga::Cost{dsp: 1, lut: 0}),
        ((17,18), fpga::Cost{dsp: 1, lut: 0}),
        ((17,19), fpga::Cost{dsp: 1, lut: 0}),
        ((17,20), fpga::Cost{dsp: 1, lut: 0}),
        ((17,21), fpga::Cost{dsp: 1, lut: 0}),
        ((17,22), fpga::Cost{dsp: 1, lut: 0}),
        ((17,23), fpga::Cost{dsp: 1, lut: 0}),
        ((17,24), fpga::Cost{dsp: 1, lut: 0}),
        ((17,25), fpga::Cost{dsp: 1, lut: 0}),
        ((17,26), fpga::Cost{dsp: 1, lut: 0}),
        ((17,27), fpga::Cost{dsp: 1, lut: 0}),
        ((17,28), fpga::Cost{dsp: 1, lut: 37}),
        ((17,29), fpga::Cost{dsp: 1, lut: 67}),
        ((17,30), fpga::Cost{dsp: 2, lut: 0}),
        ((17,31), fpga::Cost{dsp: 2, lut: 0}),
        ((17,32), fpga::Cost{dsp: 2, lut: 0}),
        ((17,33), fpga::Cost{dsp: 2, lut: 0}),
        ((18,18), fpga::Cost{dsp: 1, lut: 0}),
        ((18,19), fpga::Cost{dsp: 1, lut: 0}),
        ((18,20), fpga::Cost{dsp: 1, lut: 0}),
        ((18,21), fpga::Cost{dsp: 1, lut: 0}),
        ((18,22), fpga::Cost{dsp: 1, lut: 0}),
        ((18,23), fpga::Cost{dsp: 1, lut: 0}),
        ((18,24), fpga::Cost{dsp: 1, lut: 0}),
        ((18,25), fpga::Cost{dsp: 1, lut: 0}),
        ((18,26), fpga::Cost{dsp: 1, lut: 0}),
        ((18,27), fpga::Cost{dsp: 2, lut: 0}),
        ((18,28), fpga::Cost{dsp: 2, lut: 0}),
        ((18,29), fpga::Cost{dsp: 2, lut: 0}),
        ((18,30), fpga::Cost{dsp: 2, lut: 0}),
        ((18,31), fpga::Cost{dsp: 2, lut: 0}),
        ((18,32), fpga::Cost{dsp: 2, lut: 0}),
        ((18,33), fpga::Cost{dsp: 2, lut: 0}),
        ((19,19), fpga::Cost{dsp: 1, lut: 41}),
        ((19,20), fpga::Cost{dsp: 1, lut: 43}),
        ((19,21), fpga::Cost{dsp: 1, lut: 45}),
        ((19,22), fpga::Cost{dsp: 1, lut: 47}),
        ((19,23), fpga::Cost{dsp: 1, lut: 49}),
        ((19,24), fpga::Cost{dsp: 1, lut: 51}),
        ((19,25), fpga::Cost{dsp: 1, lut: 53}),
        ((19,26), fpga::Cost{dsp: 1, lut: 55}),
        ((19,27), fpga::Cost{dsp: 2, lut: 0}),
        ((19,28), fpga::Cost{dsp: 2, lut: 0}),
        ((19,29), fpga::Cost{dsp: 2, lut: 0}),
        ((19,30), fpga::Cost{dsp: 2, lut: 0}),
        ((19,31), fpga::Cost{dsp: 2, lut: 0}),
        ((19,32), fpga::Cost{dsp: 2, lut: 0}),
        ((19,33), fpga::Cost{dsp: 2, lut: 0}),
        ((20,20), fpga::Cost{dsp: 1, lut: 78}),
        ((20,21), fpga::Cost{dsp: 1, lut: 81}),
        ((20,22), fpga::Cost{dsp: 1, lut: 85}),
        ((20,23), fpga::Cost{dsp: 1, lut: 88}),
        ((20,24), fpga::Cost{dsp: 1, lut: 92}),
        ((20,25), fpga::Cost{dsp: 1, lut: 95}),
        ((20,26), fpga::Cost{dsp: 1, lut: 99}),
        ((20,27), fpga::Cost{dsp: 2, lut: 0}),
        ((20,28), fpga::Cost{dsp: 2, lut: 0}),
        ((20,29), fpga::Cost{dsp: 2, lut: 0}),
        ((20,30), fpga::Cost{dsp: 2, lut: 0}),
        ((20,31), fpga::Cost{dsp: 2, lut: 0}),
        ((20,32), fpga::Cost{dsp: 2, lut: 0}),
        ((20,33), fpga::Cost{dsp: 2, lut: 0}),
        ((21,21), fpga::Cost{dsp: 2, lut: 0}),
        ((21,22), fpga::Cost{dsp: 2, lut: 0}),
        ((21,23), fpga::Cost{dsp: 2, lut: 0}),
        ((21,24), fpga::Cost{dsp: 2, lut: 0}),
        ((21,25), fpga::Cost{dsp: 2, lut: 0}),
        ((21,26), fpga::Cost{dsp: 2, lut: 0}),
        ((21,27), fpga::Cost{dsp: 2, lut: 0}),
        ((21,28), fpga::Cost{dsp: 2, lut: 0}),
        ((21,29), fpga::Cost{dsp: 2, lut: 0}),
        ((21,30), fpga::Cost{dsp: 2, lut: 0}),
        ((21,31), fpga::Cost{dsp: 2, lut: 0}),
        ((21,32), fpga::Cost{dsp: 2, lut: 0}),
        ((21,33), fpga::Cost{dsp: 2, lut: 0}),
        ((22,22), fpga::Cost{dsp: 2, lut: 0}),
        ((22,22), fpga::Cost{dsp: 2, lut: 0}),
        ((22,23), fpga::Cost{dsp: 2, lut: 0}),
        ((22,24), fpga::Cost{dsp: 2, lut: 0}),
        ((22,25), fpga::Cost{dsp: 2, lut: 0}),
        ((22,26), fpga::Cost{dsp: 2, lut: 0}),
        ((22,27), fpga::Cost{dsp: 2, lut: 0}),
        ((22,28), fpga::Cost{dsp: 2, lut: 0}),
        ((22,29), fpga::Cost{dsp: 2, lut: 0}),
        ((22,30), fpga::Cost{dsp: 2, lut: 0}),
        ((22,31), fpga::Cost{dsp: 2, lut: 0}),
        ((22,32), fpga::Cost{dsp: 2, lut: 0}),
        ((22,33), fpga::Cost{dsp: 2, lut: 0}),
        ((23,23), fpga::Cost{dsp: 2, lut: 0}),
        ((23,24), fpga::Cost{dsp: 2, lut: 0}),
        ((23,25), fpga::Cost{dsp: 2, lut: 0}),
        ((23,26), fpga::Cost{dsp: 2, lut: 0}),
        ((23,27), fpga::Cost{dsp: 2, lut: 0}),
        ((23,28), fpga::Cost{dsp: 2, lut: 0}),
        ((23,29), fpga::Cost{dsp: 2, lut: 0}),
        ((23,30), fpga::Cost{dsp: 2, lut: 0}),
        ((23,31), fpga::Cost{dsp: 2, lut: 0}),
        ((23,32), fpga::Cost{dsp: 2, lut: 0}),
        ((23,33), fpga::Cost{dsp: 2, lut: 0}),
        ((24,24), fpga::Cost{dsp: 2, lut: 0}),
        ((24,25), fpga::Cost{dsp: 2, lut: 0}),
        ((24,26), fpga::Cost{dsp: 2, lut: 0}),
        ((24,27), fpga::Cost{dsp: 2, lut: 0}),
        ((24,28), fpga::Cost{dsp: 2, lut: 0}),
        ((24,29), fpga::Cost{dsp: 2, lut: 0}),
        ((24,30), fpga::Cost{dsp: 2, lut: 0}),
        ((24,31), fpga::Cost{dsp: 2, lut: 0}),
        ((24,32), fpga::Cost{dsp: 2, lut: 0}),
        ((24,33), fpga::Cost{dsp: 2, lut: 0}),
        ((25,25), fpga::Cost{dsp: 2, lut: 0}),
        ((25,26), fpga::Cost{dsp: 2, lut: 0}),
        ((25,27), fpga::Cost{dsp: 2, lut: 0}),
        ((25,28), fpga::Cost{dsp: 2, lut: 0}),
        ((25,29), fpga::Cost{dsp: 2, lut: 0}),
        ((25,30), fpga::Cost{dsp: 2, lut: 0}),
        ((25,31), fpga::Cost{dsp: 2, lut: 0}),
        ((25,32), fpga::Cost{dsp: 2, lut: 0}),
        ((25,33), fpga::Cost{dsp: 2, lut: 0}),
        ((26,26), fpga::Cost{dsp: 2, lut: 0}),
        ((26,27), fpga::Cost{dsp: 2, lut: 0}),
        ((26,28), fpga::Cost{dsp: 2, lut: 0}),
        ((26,29), fpga::Cost{dsp: 2, lut: 0}),
        ((26,30), fpga::Cost{dsp: 2, lut: 0}),
        ((26,31), fpga::Cost{dsp: 2, lut: 0}),
        ((26,32), fpga::Cost{dsp: 2, lut: 0}),
        ((26,33), fpga::Cost{dsp: 2, lut: 0}),
        ((27,27), fpga::Cost{dsp: 4, lut: 38}),
        ((27,28), fpga::Cost{dsp: 4, lut: 39}),
        ((27,29), fpga::Cost{dsp: 4, lut: 40}),
        ((27,30), fpga::Cost{dsp: 4, lut: 41}),
        ((27,31), fpga::Cost{dsp: 4, lut: 42}),
        ((27,32), fpga::Cost{dsp: 4, lut: 43}),
        ((27,33), fpga::Cost{dsp: 4, lut: 44}),
        ((28,28), fpga::Cost{dsp: 4, lut: 40}),
        ((28,29), fpga::Cost{dsp: 4, lut: 41}),
        ((28,30), fpga::Cost{dsp: 4, lut: 42}),
        ((28,31), fpga::Cost{dsp: 4, lut: 43}),
        ((28,32), fpga::Cost{dsp: 4, lut: 44}),
        ((28,33), fpga::Cost{dsp: 4, lut: 45}),
        ((29,29), fpga::Cost{dsp: 4, lut: 42}),
        ((29,30), fpga::Cost{dsp: 4, lut: 43}),
        ((29,31), fpga::Cost{dsp: 4, lut: 44}),
        ((29,32), fpga::Cost{dsp: 4, lut: 45}),
        ((29,33), fpga::Cost{dsp: 4, lut: 46}),
        ((30,30), fpga::Cost{dsp: 4, lut: 44}),
        ((30,31), fpga::Cost{dsp: 4, lut: 45}),
        ((30,32), fpga::Cost{dsp: 4, lut: 46}),
        ((30,33), fpga::Cost{dsp: 4, lut: 47}),
        ((31,31), fpga::Cost{dsp: 4, lut: 46}),
        ((31,32), fpga::Cost{dsp: 4, lut: 47}),
        ((31,33), fpga::Cost{dsp: 4, lut: 48}),
        ((32,32), fpga::Cost{dsp: 4, lut: 48}),
        ((32,33), fpga::Cost{dsp: 4, lut: 49}),
        ((33,33), fpga::Cost{dsp: 4, lut: 50}),        
        ]);
    
    let cost = mul_costs.get(&(width_1,width_2));

    match cost {
        Some(x) => return *x,
        None => {
            let cost_again = mul_costs.get(&(width_2, width_1));
            match cost_again {
                Some(x) => return *x,
                None => return mul_cost(std::cmp::max(width_1, width_2))
            }
        }
    }
}
