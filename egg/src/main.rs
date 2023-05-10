use egg::*;
use std::fs;
use std::io::prelude::*;

mod utils;
use utils::{language::*,costs::*};


#[macro_use]
extern crate fstrings;

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
    generate_verilog("(+ 32 IN1 IN2)".to_string(), 32);
    let input = "(* 32 IN1 IN2)";
    let result = simplify(input);
    let mut dst = fs::File::create("results.txt")?;

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
                (a.to_string(), 32, a.to_string())
            }
            _ => (name, 32, String::default())
        }
    }

    fn merge(&mut self, _a: &mut Self::Data, _b: Self::Data) -> DidMerge {
        DidMerge(false,false)
    }
}

fn generate_verilog(expr : String, variable_bitwidth : u64) {
    let mut generation_egraph = EGraphVerilogGeneration::default();
    let root = generation_egraph.add_expr(&expr.parse().unwrap());

    println!("Expr is {expr} \n EGraph is {:?}", generation_egraph);

    for class in generation_egraph.classes() {
        let node = &class.nodes[0];
        println!("Node is {}", node);
        match node {
            BitLanguage::Symbol(_) => (),
            _ => println!("logic [{}:0] {};", class.data.1 - 1, class.data.0),
        }
    }

    for class in generation_egraph.classes() {
        let node = &class.nodes[0];
        println!("Node is {}", node);
        match node {
            BitLanguage::Symbol(_) => (),
            _ => println!("{} = {};", class.data.0, class.data.2),
        }
    }
}

fn bitlanguage_to_name(enode: &BitLanguage) -> String {
    match enode {
        BitLanguage::Add(_)  => "add".to_string(),
        BitLanguage::AddW(_) => "addW".to_string(),
        BitLanguage::Mul(_)  => "mul".to_string(),
        BitLanguage::SubW(_) => "subW".to_string(),
        BitLanguage::Sub(_)  => "sub".to_string(),
        BitLanguage::Slc(_)  => "slice".to_string(),
        BitLanguage::Cct(_)  => "concat".to_string(),
        _                    => "panic".to_string()
    }
}

