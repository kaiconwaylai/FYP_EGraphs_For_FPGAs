use egg::*;
use std::fs;
use std::io::prelude::*;

mod utils;
use utils::{language::*,costs::*};

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

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let input = "(* 64 IN1 IN2)";
    let result = simplify(input);
    fs::create_dir_all("./output")?;
    let dst = fs::File::create("./output/mult.v")?;
    
    generate_verilog(&result, 64, &dst);
    
    let mut dst = fs::File::create("./output/results.txt")?;

    for i in 0..101 {
        let i = i as f64/100.0;
        alpha(i);
        let result = simplify(input);
        write!(dst, "Alpha = {}. Result = {}\n\n", alpha(-1.0), result)?;
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

        let operator = match enode {
            BitLanguage::AddW(_) => "+",
            BitLanguage::SubW(_) => "-",
            BitLanguage::Mul(_) => "*",
            _ => ""
        };

        match enode {
            BitLanguage::AddW([a,b,c]) | BitLanguage::SubW([a,b,c]) => {
                let node = &egraph[*a].nodes[0];
                if let BitLanguage::Num(x) = node {
                    let bit_width = *x as u64;
                    return (name, bit_width, format!("{} {} {}", get_name(b), operator, get_name(c)));
                }
                else {
                    assert!(false);
                }
                (name, 0, String::default())
            }
            BitLanguage::Mul([_a,b,c]) => {
                return (name, get_bw(b) + get_bw(c), format!("{} {} {}", get_name(b), operator, get_name(c)));
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
                (a.to_string(), 32, a.to_string())
            }
            _ => (name, 32, String::default())
        }
    }

    fn merge(&mut self, _a: &mut Self::Data, _b: Self::Data) -> DidMerge {
        DidMerge(false,false)
    }
}

fn generate_verilog(expr : &String, variable_bitwidth : u64, mut file : &fs::File) {
    let mut generation_egraph = EGraphVerilogGeneration::default();
    let root = generation_egraph.add_expr(&expr.parse().unwrap());
    let get_name       = |i: &Id| generation_egraph[*i].data.0.clone();


    let module_definition = format!("`timescale 1ns / 1ps
    module mult(
            input[{bw}:0] IN1,
            input[{bw}:0] IN2,
            output[{}:0] OUTPUT
        );\n", variable_bitwidth*2-1,  bw = variable_bitwidth-1);
    
    let mut module_body = String::default();

    for class in generation_egraph.classes() {
        let node = &class.nodes[0];
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

    let end_module = format!("assign OUTPUT = {};
    endmodule", get_name(&root));
    println!("{end_module}");
    write!(file, "{}{}{}", module_definition, module_body, end_module).expect("File broke");
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
        BitLanguage::Num(_)  => String::from(""),
        BitLanguage::Symbol(_)  => String::from(""),
        _                    => {
            println!("Paniced: {}", enode);
            String::from("panic")}
    }
}

