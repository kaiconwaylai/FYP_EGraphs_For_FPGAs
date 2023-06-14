use super::{language::*, super::INPUT_BW,costs::*,fpga::Cost};
use egg::*;
use std::fs;
use std::io::prelude::*;


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

        let operator = match enode {
            BitLanguage::AddW(_) | BitLanguage::Add(_) => "+",
            BitLanguage::SubW(_) | BitLanguage::Sub(_) => "-",
            BitLanguage::Lsl(_) => "<<",
            BitLanguage::Mul(_) | BitLanguage::MulNW(_) | BitLanguage::Mul4(_) => "*",
            _ => ""
        };

        match enode {
            BitLanguage::Add([a,b]) | BitLanguage::Sub([a,b]) => {
                return (name, std::cmp::max(get_bw(b),get_bw(a)) + 1 , format!("{} {} {}", get_name(a), operator, get_name(b)));
            }
            BitLanguage::AddW([a,b,c]) | BitLanguage::SubW([a,b,c]) => {
                return (name, std::cmp::max(get_bw(b),get_bw(a)) + 1, format!("{} {} {}", get_name(b), operator, get_name(c)));
            }
            BitLanguage::Mul([_a,b,c]) => {
                return (name, get_bw(b) + get_bw(c), format!("{} {} {}", get_name(b), operator, get_name(c)));
            }
            BitLanguage::MulNW([b,c]) => {
                return (name, get_bw(b) + get_bw(c), format!("{} {} {}", get_name(b), operator, get_name(c)));
            }
            BitLanguage::Mul4([_a,_b,c,d]) => {
                return (name, get_bw(c) + get_bw(d), format!("{} {} {}", get_name(c), operator, get_name(d)));
            }
            BitLanguage::Lsl([a,b]) => {
                return (name, get_bw(a) + get_bw(b), format!("{} {} {}", get_name(b), operator, get_name(a)));
            }
            BitLanguage::Slc([a,b,c]) => {
                let msb_node = &egraph[*b].nodes[0];
                let lsb_node = &egraph[*c].nodes[0];

                if let BitLanguage::Num(x) = msb_node {
                    let msb = *x as u64;
                    if let BitLanguage::Num(y) = lsb_node {
                        let lsb = *y as u64;
                        return (name, msb-lsb+1, format!("{}[{}:{}]", get_name(a), msb, lsb));
                    }
                }
                else {
                    assert!(false);
                }
                (name, 0, String::default())
            }
            BitLanguage::Cct([a,b]) => {
                return (name, get_bw(a) + get_bw(b), format!("{{{},{}}}", get_name(a), get_name(b)));
            }

            BitLanguage::Symbol(a) => {
                unsafe {
                    (a.to_string(), INPUT_BW, a.to_string())
                }
            }
            BitLanguage::Num(x) => {
                (x.to_string(), *x as u64, x.to_string())
            }
            //_ => (name, 32, String::default())
        }
    }

    fn merge(&mut self, _a: &mut Self::Data, _b: Self::Data) -> DidMerge {
        DidMerge(false,false)
    }
}

pub fn generate_verilog(expr : &String, variable_bitwidth : u64, mut file : &fs::File) -> Cost {
    let mut generation_egraph = EGraphVerilogGeneration::default();
    let root = generation_egraph.add_expr(&expr.parse().unwrap());
    let get_name       = |i: &Id| generation_egraph[*i].data.0.clone();
    let mut expr_cost = Cost{dsp: 0, lut: 0};

    let mut module_body = String::default();

    for class in generation_egraph.classes() {
        let node = &class.nodes[0];
        let node_cost = cost_node(node, &generation_egraph);
        expr_cost = expr_cost + node_cost;
        match node {
            BitLanguage::Symbol(_) | BitLanguage::Num(_) => (),
            _ => module_body.push_str(&format!("wire [{}:0] {};\n", class.data.1 - 1, class.data.0)),
        }
    }

    for class in generation_egraph.classes() {
        let node = &class.nodes[0];
        match node {
            BitLanguage::Symbol(_) | BitLanguage::Num(_) => (),
            _ => module_body.push_str(&format!("assign {} = {};\n", class.data.0, class.data.2)),
        }
    }

    write!(file, "//Alpha = {}. Cost: LUTs = {}. DSPs = {}. \n\n", alpha(None), expr_cost.lut, expr_cost.dsp).expect("File broke");

    let module_definition = format!("`timescale 1ns / 1ps
    module mult(
        input[{bw}:0] IN1,
        input[{bw}:0] IN2,
        output[{}:0] OUTPUT
    );\n", variable_bitwidth*2-1,  bw = variable_bitwidth-1);
    write!(file, "{}", module_definition).expect("File broke");

    write!(file, "{}", module_body).expect("File broke");

    let end_module = format!("assign OUTPUT = {};
    endmodule", get_name(&root));
    write!(file, "{}", end_module).expect("File broke");
    expr_cost
}

fn bitlanguage_to_name(enode: &BitLanguage) -> String {
    match enode {
        BitLanguage::Add(_)  => String::from("add"),
        BitLanguage::AddW(_) => String::from("addW"),
        BitLanguage::Mul(_)  => String::from("mul"),
        BitLanguage::SubW(_) => String::from("subW"),
        BitLanguage::Sub(_)  => String::from("sub"),
        BitLanguage::Slc(_)  => String::from("slice"),
        BitLanguage::Cct(_)  => String::from("concat"),
        BitLanguage::Symbol(_)  => String::from(""),
        BitLanguage::Mul4(_)  => String::from("mul4"),
        BitLanguage::MulNW(_)  => String::from("mulnw"),
        BitLanguage::Lsl(_)  => String::from("lsl"),
        BitLanguage::Num(_x)  => String::from("num"),
        // _                    => {
        //     println!("Paniced: {}", enode);
        //     String::from("panic")}
    }
}

