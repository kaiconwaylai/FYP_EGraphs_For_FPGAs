use egg::*;
use std::collections::HashMap;
use crate::utils::fpga;
use crate::utils::language::*;

pub fn mul_cost(width : i32) -> fpga::Cost {

    let mul_costs : HashMap<i32, fpga::Cost> = HashMap::from([(1, fpga::Cost{dsp: 1, lut: 0}),
                (2, fpga::Cost{dsp: 2, lut: 0}),
                (3, fpga::Cost{dsp: 5, lut: 0}),
                (4, fpga::Cost{dsp: 18, lut: 0}),
                (5, fpga::Cost{dsp: 29, lut: 0}),
                (6, fpga::Cost{dsp: 43, lut: 0}),
                (7, fpga::Cost{dsp: 52, lut: 0}),
                (8, fpga::Cost{dsp: 71, lut: 0}),
                (9, fpga::Cost{dsp: 82, lut: 0}),
                (10, fpga::Cost{dsp: 0, lut: 1}),
                (11, fpga::Cost{dsp: 0, lut: 1}),
                (12, fpga::Cost{dsp: 0, lut: 1}),
                (13, fpga::Cost{dsp: 0, lut: 1}),
                (14, fpga::Cost{dsp: 0, lut: 1}),
                (15, fpga::Cost{dsp: 0, lut: 1}),
                (16, fpga::Cost{dsp: 0, lut: 1}),
                (17, fpga::Cost{dsp: 0, lut: 1}),
                (18, fpga::Cost{dsp: 0, lut: 1}),
                (19, fpga::Cost{dsp: 41, lut: 1}),
                (20, fpga::Cost{dsp: 78, lut: 1}),
                (21, fpga::Cost{dsp: 0, lut: 2}),
                (22, fpga::Cost{dsp: 0, lut: 2}),
                (23, fpga::Cost{dsp: 0, lut: 2}),
                (24, fpga::Cost{dsp: 0, lut: 2}),
                (25, fpga::Cost{dsp: 0, lut: 2}),
                (26, fpga::Cost{dsp: 0, lut: 2}),
                (27, fpga::Cost{dsp: 38, lut: 4}),
                (28, fpga::Cost{dsp: 40, lut: 4}),
                (29, fpga::Cost{dsp: 42, lut: 4}),
                (30, fpga::Cost{dsp: 44, lut: 4}),
                (31, fpga::Cost{dsp: 46, lut: 4}),
                (32, fpga::Cost{dsp: 48, lut: 4}),
                (33, fpga::Cost{dsp: 50, lut: 4}),
                (34, fpga::Cost{dsp: 52, lut: 4}),
                (35, fpga::Cost{dsp: 64, lut: 4}),
                (36, fpga::Cost{dsp: 84, lut: 4}),
                (37, fpga::Cost{dsp: 122, lut: 4}),
                (38, fpga::Cost{dsp: 71, lut: 5}),
                (39, fpga::Cost{dsp: 75, lut: 5}),
                (40, fpga::Cost{dsp: 79, lut: 5}),
                (41, fpga::Cost{dsp: 83, lut: 5}),
                (42, fpga::Cost{dsp: 86, lut: 5}),
                (43, fpga::Cost{dsp: 88, lut: 5}),
                (44, fpga::Cost{dsp: 73, lut: 9}),
                (45, fpga::Cost{dsp: 75, lut: 9}),
                (46, fpga::Cost{dsp: 77, lut: 9}),
                (47, fpga::Cost{dsp: 79, lut: 9}),
                (48, fpga::Cost{dsp: 81, lut: 9}),
                (49, fpga::Cost{dsp: 83, lut: 9}),
                (50, fpga::Cost{dsp: 85, lut: 9}),
                (51, fpga::Cost{dsp: 87, lut: 9}),
                (52, fpga::Cost{dsp: 89, lut: 9}),
                (53, fpga::Cost{dsp: 111, lut: 9}),
                (54, fpga::Cost{dsp: 148, lut: 9}),
                (55, fpga::Cost{dsp: 95, lut: 10}),
                (56, fpga::Cost{dsp: 97, lut: 10}),
                (57, fpga::Cost{dsp: 99, lut: 10}),
                (58, fpga::Cost{dsp: 101, lut: 10}),
                (59, fpga::Cost{dsp: 103, lut: 10}),
                (60, fpga::Cost{dsp: 105, lut: 10}),
                (61, fpga::Cost{dsp: 171, lut: 16}),
                (62, fpga::Cost{dsp: 173, lut: 16}),
                (63, fpga::Cost{dsp: 175, lut: 16}),
                (64, fpga::Cost{dsp: 177, lut: 16}),
                (65, fpga::Cost{dsp: 179, lut: 16}),
                (66, fpga::Cost{dsp: 181, lut: 16}),
                (67, fpga::Cost{dsp: 185, lut: 16}),
                (68, fpga::Cost{dsp: 187, lut: 16}),
                (69, fpga::Cost{dsp: 173, lut: 16}),
                (70, fpga::Cost{dsp: 195, lut: 16}),
                (71, fpga::Cost{dsp: 234, lut: 16}),
                (72, fpga::Cost{dsp: 179, lut: 17}),
                (73, fpga::Cost{dsp: 181, lut: 17}),
                (74, fpga::Cost{dsp: 183, lut: 17}),
                (75, fpga::Cost{dsp: 185, lut: 17}),
                (76, fpga::Cost{dsp: 187, lut: 17}),
                (77, fpga::Cost{dsp: 189, lut: 17}),
                (78, fpga::Cost{dsp: 281, lut: 25}),
                (79, fpga::Cost{dsp: 285, lut: 25}),
                (80, fpga::Cost{dsp: 289, lut: 25}),
                (81, fpga::Cost{dsp: 293, lut: 25}),
                (82, fpga::Cost{dsp: 297, lut: 25}),
                (83, fpga::Cost{dsp: 301, lut: 25}),
                (84, fpga::Cost{dsp: 304, lut: 25}),
                (85, fpga::Cost{dsp: 306, lut: 25}),
                (86, fpga::Cost{dsp: 327, lut: 25}),
                (87, fpga::Cost{dsp: 351, lut: 25}),
                (88, fpga::Cost{dsp: 392, lut: 25}),
                (89, fpga::Cost{dsp: 314, lut: 26}),
                (90, fpga::Cost{dsp: 316, lut: 26}),
                (91, fpga::Cost{dsp: 318, lut: 26}),
                (92, fpga::Cost{dsp: 320, lut: 26}),
                (93, fpga::Cost{dsp: 322, lut: 26}),
                (94, fpga::Cost{dsp: 324, lut: 26}),
                (95, fpga::Cost{dsp: 349, lut: 36}),
                (96, fpga::Cost{dsp: 353, lut: 36}),
                (97, fpga::Cost{dsp: 357, lut: 36}),
                (98, fpga::Cost{dsp: 361, lut: 36}),
                (99, fpga::Cost{dsp: 369, lut: 36}),
                (100, fpga::Cost{dsp: 373, lut: 36}),
                (101, fpga::Cost{dsp: 377, lut: 36}),
                (102, fpga::Cost{dsp: 381, lut: 36}),
                (103, fpga::Cost{dsp: 411, lut: 36}),
                (104, fpga::Cost{dsp: 429, lut: 36}),
                (105, fpga::Cost{dsp: 471, lut: 36}),
                (106, fpga::Cost{dsp: 421, lut: 37}),
                (107, fpga::Cost{dsp: 427, lut: 37}),
                (108, fpga::Cost{dsp: 433, lut: 37}),
                (109, fpga::Cost{dsp: 439, lut: 37}),
                (110, fpga::Cost{dsp: 442, lut: 37}),
                (111, fpga::Cost{dsp: 445, lut: 37}),
                (112, fpga::Cost{dsp: 803, lut: 49}),
                (113, fpga::Cost{dsp: 809, lut: 49}),
                (114, fpga::Cost{dsp: 815, lut: 49}),
                (115, fpga::Cost{dsp: 821, lut: 49}),
                (116, fpga::Cost{dsp: 827, lut: 49}),
                (117, fpga::Cost{dsp: 833, lut: 49}),
                (118, fpga::Cost{dsp: 838, lut: 49}),
                (119, fpga::Cost{dsp: 840, lut: 49}),
                (120, fpga::Cost{dsp: 865, lut: 49}),
                (121, fpga::Cost{dsp: 893, lut: 49}),
                (122, fpga::Cost{dsp: 935, lut: 49}),
                (123, fpga::Cost{dsp: 857, lut: 50}),
                (124, fpga::Cost{dsp: 859, lut: 50}),
                (125, fpga::Cost{dsp: 865, lut: 50}),
                (126, fpga::Cost{dsp: 869, lut: 50}),
                (127, fpga::Cost{dsp: 873, lut: 50}),
                (128, fpga::Cost{dsp: 879, lut: 50})]);


    mul_costs[&width]
}

pub fn alpha(val : f64) -> f64 {
    static mut A : f64 = 0.0;
    unsafe {
        if 0.0 <= val && val <= 1.0 {
            A = val;
        }
        A
    }
}

pub struct FPGACostFunction<'a> {
    pub egraph: &'a EGraph<BitLanguage, ()>,
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