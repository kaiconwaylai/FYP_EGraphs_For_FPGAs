use egg::*;
use std::fs;
use std::env;
use std::io::prelude::*;
use std::collections::HashSet;

mod utils;
use utils::{language::*,costs::*, codegen::*};

static mut INPUT_BW : u64 = 1024;
fn main() -> std::io::Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        unsafe {
            INPUT_BW = args[1].parse::<u64>().unwrap();
        }
    }

    println!("Hello, world!");
    fs::create_dir_all("./output")?;
    fs::remove_dir_all("./output/verilog")?;
    fs::create_dir_all("./output/verilog")?;
    let input;
    unsafe {
        input = format!("(* {INPUT_BW} IN1 IN2)");
    }
    
    let mut results = fs::File::create("./output/results.txt")?;

    let expr: RecExpr<BitLanguage> = input.parse().unwrap();
    let runner = Runner::default().with_expr(&expr).run(&make_rules());
    let root: Id = runner.roots[0];

    let mut unique_solutions = HashSet::new();

    for i in 0..101 {
        alpha(i as f64/500.0);
        let mut lp_extractor = LpExtractor::new(&runner.egraph, FPGACostFunction{egraph: &runner.egraph, seen_nodes: HashSet::new()});
        let best_sol = lp_extractor.solve(root);
        let best = best_sol.to_string();
        if unique_solutions.insert(best.clone()) {
            let cost = FPGACostFunction::cost_rec(&mut FPGACostFunction{egraph: &runner.egraph, seen_nodes: HashSet::new()},&best_sol);
            
            let mut dst = fs::File::create(format!("./output/verilog/mult_{i}.v", ))?;
            write!(dst, "//Alpha = {}. Cost: LUTs = {}. DSPs = {}.  \n\n", alpha(-1.0), cost.lut, cost.dsp)?;
            write!(results, "Alpha = {}. Cost: LUTs = {}. DSPs = {}.  \n\n", alpha(-1.0), cost.lut, cost.dsp)?;
            unsafe {
                generate_verilog(&best, INPUT_BW, &dst);
            }
        }

        if best == input {
            break;
        }
    }



    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
