use egg::*;
use std::fs;
use std::io::prelude::*;

mod utils;
use utils::{language::*,costs::*, codegen::*};

fn simplify(s: &str)  -> String {
    let expr: RecExpr<BitLanguage> = s.parse().unwrap();
    // simplify the expression using a Runner, which creates an e-graph with the given expression and runs the given rules over it
    let runner: Runner<BitLanguage, ()> = Runner::default().with_expr(&expr).run(&make_rules());
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

static INPUT_BW : u64 = 64;
fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    fs::create_dir_all("./output")?;
    let input = format!("(* {INPUT_BW} IN1 IN2)");
    let dst = fs::File::create("./output/mult.v")?;
    alpha(0.0);
    let result = simplify(&input);
    
    generate_verilog(&result, INPUT_BW, &dst);
    
    let mut dst = fs::File::create("./output/results.txt")?;

    let expr: RecExpr<BitLanguage> = input.parse().unwrap();
    let runner = Runner::default().with_expr(&expr).run(&make_rules());
    let root = runner.roots[0];

    for i in 0..11 {
        let i = i as f64/500.0;
        alpha(i);
        let mut lp_extractor = LpExtractor::new(&runner.egraph, FPGACostFunction{egraph: &runner.egraph});
        let best_sol = lp_extractor.solve(root);
        write!(dst, "Alpha = {}. Result = {}\n\n", alpha(-1.0), best_sol.to_string())?;        
    }


    Ok(())
}
