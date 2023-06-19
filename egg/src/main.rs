use egg::*;
use std::fs;
use std::env;
use std::io::prelude::*;
use std::collections::HashSet;
use std::time::{Duration,Instant};

mod utils;
use utils::{language::*,costs::*, codegen::*};

static mut INPUT_BW : u64 = 1024;
fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        unsafe {
            INPUT_BW = args[1].parse::<u64>().unwrap();
        }
    }

    let runner_iteration_limit = 10000000;
    let egraph_node_limit = 25000000000;
    let iterations = 1000;
    let step = 1.0/1000.0;
    let cbc_timeout = 300.0;

    println!("Hello, world!");
    fs::remove_dir_all("./output")?;
    fs::create_dir_all("./output")?;
    fs::create_dir_all("./output/verilog")?;
    let input;
    unsafe {
        input = format!("(* {INPUT_BW} IN1 IN2)");
    }
    
    let mut results = fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open("./output/results.txt")?;

    let expr: RecExpr<BitLanguage> = input.parse().unwrap();

    let start = Instant::now();
    let runner = Runner::default()
        .with_iter_limit(runner_iteration_limit)
        .with_node_limit(egraph_node_limit)
        .with_time_limit(Duration::new(100000, 0))
        .with_expr(&expr)
        .run(&make_rules());
    let duration = start.elapsed();

    write!(results, "Runner stopped: {:?}. Time take for runner: {:?}, Classes: {}, Nodes: {}, Size: {}\n\n",
            runner.stop_reason, duration, runner.egraph.number_of_classes(),
            runner.egraph.total_number_of_nodes(), runner.egraph.total_size())?;
    let root: Id = runner.roots[0];

    let mut unique_solutions = HashSet::new();

    for i in 0..iterations+1 {
        alpha(Some(i as f64*step));
        let mut lp_extractor = LpExtractor::new(&runner.egraph, FPGACostFunction{egraph: &runner.egraph, seen_nodes: HashSet::new()});
        lp_extractor.timeout(cbc_timeout);
        let best_sol = lp_extractor.solve(root);
        let best = best_sol.to_string();
        if unique_solutions.insert(best.clone()) {            
            let dst = fs::File::create(format!("./output/verilog/mult_{i}.v", ))?;
            let cost;            
            unsafe {
                cost = generate_verilog(&best, INPUT_BW, &dst);
            }
            write!(results, "Alpha = {}. Cost: LUTs = {}. DSPs = {}. \n\n", alpha(None), cost.lut, cost.dsp)?;

        }

        if best == input {
            break;
        }
    }

    Ok(())
}
