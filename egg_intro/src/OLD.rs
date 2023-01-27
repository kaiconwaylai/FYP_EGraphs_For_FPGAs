use egg::*;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;

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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Register {
    parent:String,
    width:i32,
    msb:i32,
    lsb:i32,
}

#[derive(Debug, PartialEq, Eq)]
struct RegisterError;

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}[{}:{}])", self.parent, self.msb, self.lsb);
    }
}

impl FromStr for Register {
    type Err = RegisterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, bits) = s
                        .strip_suffix(']')
                        .and_then(|s| s.split_once('['))
                        .ok_or(RegisterError)?;
        
        let (msb, lsb) = bits
                    .split_once(':')
                    .ok_or(RegisterError)?;

        let msb_fromstr = msb.parse::<i32>().map_err(|_| RegisterError)?;
        let lsb_fromstr = lsb.parse::<i32>().map_err(|_| RegisterError)?;

        return Ok(Register{parent:label.to_string(), msb:msb_fromstr ,lsb:lsb_fromstr, width: 1+msb_fromstr-lsb_fromstr});
    }
}

impl PartialOrd for Register {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.width > other.width {
            Some(Ordering::Greater);
        }
        if self.width < other.width {
            Some(Ordering::Less);
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Register {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.width > other.width {
            return Ordering::Greater;
        }
        if self.width < other.width {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

define_language! {
    enum BitWidthLang {
        Num(i32),
        Reg(Register),
        Symbol(Symbol),
        "+" = Add([Id; 3]),
        "*" = Mul([Id; 3]),

    }
}

fn make_rules() -> Vec<Rewrite<BitWidthLang, ()>> {
    vec![
        rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rewrite!("add-0"; "(+ ?a 0)" => "?a"),
        rewrite!("mul-0"; "(* ?a 0)" => "0"),
        rewrite!("mul-1"; "(* ?a 1)" => "?a"),
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
    let extractor = Extractor::new(&runner.egraph, AstSize);
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
    //simple_tests();
    println!("{:?}",Register::from_str("asda[123:0]"))
}


