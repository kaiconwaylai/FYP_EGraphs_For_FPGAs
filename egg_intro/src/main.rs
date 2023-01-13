// use egg::*;

// define_language! {
//     enum SimpleLanguage {
//         Num(i32),
//         "+" = Add([Id; 2]),
//         "*" = Mul([Id; 2]),
//         Symbol(Symbol),
//     }
// }

// define_language! {
//     enum RegisterLang {
//         Num(i32),
//         "+" = Add([Id; 2]),
//         "*" = Mul([Id; 2]),
//         "<<" = LSL([Id; 2]),
//         ">>" = LSR([Id; 2]),
//         Symbol(Symbol),
//     }
// }

// fn make_rules() -> Vec<Rewrite<SimpleLanguage, ()>> {
//     vec![
//         rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
//         rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
//         rewrite!("add-0"; "(+ ?a 0)" => "?a"),
//         rewrite!("mul-0"; "(* ?a 0)" => "0"),
//         rewrite!("mul-1"; "(* ?a 1)" => "?a"),
//         rewrite!("assoc"; "(* ?a (+ ?b ?c))" => "(+ (* ?a ?b) (* ?a ?c) )")
//     ]
// }

// fn make_rules_rl() -> Vec<Rewrite<RegisterLang, ()>> {
//     vec![
//         rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
//         rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
//         rewrite!("add-0"; "(+ ?a 0)" => "?a"),
//         rewrite!("mul-0"; "(* ?a 0)" => "0"),
//         rewrite!("mul-1"; "(* ?a 1)" => "?a"),
//         rewrite!("assoc"; "(* ?a (+ ?b ?c))" => "(+ (* ?a ?b) (* ?a ?c) )"),
//         rewrite!("mul2shift"; "(* ?a 2)" => "(<< ?a 1)"),
//         rewrite!("shift"; "(<< ?a 0)" => "a"),
//     ]
// }

// /// parse an expression, simplify it using egg, and pretty print it back out
// fn simplify(s: &str) -> String {
//     // parse the expression, the type annotation tells it which Language to use
//     let expr: RecExpr<SimpleLanguage> = s.parse().unwrap();

//     // simplify the expression using a Runner, which creates an e-graph with
//     // the given expression and runs the given rules over it
//     let runner = Runner::default().with_expr(&expr).run(&make_rules());

//     // the Runner knows which e-class the expression given with `with_expr` is in
//     let root = runner.roots[0];

//     // use an Extractor to pick the best element of the root eclass
//     let extractor = Extractor::new(&runner.egraph, AstSize);
//     let (best_cost, best) = extractor.find_best(root);
//     println!("Simplified {} to {} with cost {}", expr, best, best_cost);
//     best.to_string()
// }

// //#[test]
// fn simple_tests() {
//     assert_eq!(simplify("(* 0 42)"), "0");
//     assert_eq!(simplify("(+ 0 (* 1 foo))"), "foo");
//     println!("{}",simplify("(* 2 (+ b 2))"))
    
// }



// fn main() {
//     println!("Hello, world!");
//     simple_tests();
// }

use egg::*;

// struct Operand<const Width: usize>  {
// }

pub struct NotAstSize;
impl<L: Language> CostFunction<L> for NotAstSize {
    type Cost = i32;
    fn cost<C>(&mut self, enode: &L, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        enode.fold(1, |sum, id| sum + costs(id))
    }
}

define_language! {
    enum BitWidthLang {
        Num(i32),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        "$" = Wdt([Id; 3]),
        Symbol(Symbol),
    }
}

fn make_rules() -> Vec<Rewrite<BitWidthLang, ()>> {
    vec![
        rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rewrite!("add-0"; "(+ ?a 0)" => "?a"),
        rewrite!("mul-0"; "(* ?a 0)" => "0"),
        rewrite!("mul-1"; "(* ?a 1)" => "?a"),
        rewrite!("tile1"; "(* ($ ?a 31 0) ($ ?b 31 0))" => "(+ (* ($ ?a 31 5) ($ ?b 31 5)) (* ($ ?a 5 0) ($ ?b 5 0)))")
    ]
}

/// parse an expression, simplify it using egg, and pretty print it back out
fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<BitWidthLang> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&make_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, NotAstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}

//#[test]
fn simple_tests() {
    assert_eq!(simplify("(* 0 42)"), "0");
    assert_eq!(simplify("(+ 0 (* 1 foo))"), "foo");
    println!("{}",simplify("(* ($ a 31 0) ($ b 31 0))"))
}

fn main() {
    println!("Hello, world!");
    simple_tests();
}
