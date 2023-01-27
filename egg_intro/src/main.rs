use egg::*;

define_language! {
    enum FPGALang {
        Num(i32),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 3]),
        "-" = Sub([Id; 2]),
        "<<" = Lsl([Id; 2]),
        ">>" = Lsr([Id; 2]),
        "slice" = Slc([Id; 3]),
        Symbol(Symbol),
    }
}


fn as_str(node: &FPGALang) -> &'static str {
    return "bar";
}

struct FPGACostFunction;
impl CostFunction<FPGALang> for FPGACostFunction {
    type Cost = f64;
    fn cost<C>(&mut self, enode: &FPGALang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost
    {
        let op_cost = match as_str(enode) {
            "* 64" => 99999999.9,
            "bar" => 0.7,
            _ => 1.0
        };
        enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}


fn make_rules() -> Vec<Rewrite<FPGALang, ()>> {
    vec![
        rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("commute-mul"; "(* ?num ?a ?b)" => "(* ?num ?b ?a)"),
        rewrite!("add-0"; "(+ ?a 0)" => "?a"),
        rewrite!("mul-0"; "(* ?num ?a 0)" => "0"),
        rewrite!("mul-1"; "(* ?num ?a 1)" => "?a"),
        rewrite!("karatsuba64";"(* 64 ?a ?b)" => "(+ (- (* 33 (+ (slice ?a 63 32) (slice ?a 31 0)) (+ (slice ?b 63 32) (slice ?b 31 0))) (+ (* 32 (slice ?a 63 32) (slice ?b 63 32)) (* 32 (slice ?a 31 0) (slice ?b 31 0)))) (+ (<< 64 (* 32 (slice ?a 63 32) (slice ?b 63 32))) (* 32 (slice ?a 31 0) (slice ?b 31 0))))"),
    ]
}

/// parse an expression, simplify it using egg, and pretty print it back out
fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<FPGALang> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&make_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, FPGACostFunction);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}

#[test]
fn simple_tests() {
    assert_eq!(simplify("(* 0 42)"), "0");
    assert_eq!(simplify("(+ 0 (* 1 foo))"), "foo");
}

fn main() {
    println!("Hello, world!");
    //simple_tests();
    println!("{}", simplify("* 64 in1 in2"))
}

//rewrite!("krt-1"; "(* 64 ?a ?b)" => "(+ (+ (* 32 ?a ?b) (<< 128 (* 32 (>> 32 ?a) (>> 32 ?b))))  (<< 32 (- (* 32 (+ (a)) ()) (+  ) ) )   )"),


//(+ (- (* 33 (+ (slice ?a 63 32) (slice ?a 31 0)) (+ (slice ?b 63 32) (slice ?b 31 0))) (+ (* 32 (slice ?a 63 32) (slice ?b 63 32)) (* 32 (slice ?a 31 0) (slice ?b 31 0)))) (+ (<< 64 (* 32 (slice ?a 63 32) (slice ?b 63 32))) (* 32 (slice ?a 31 0) (slice ?b 31 0))))