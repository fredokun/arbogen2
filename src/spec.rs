
use std::collections::HashMap;

trait Eval {
    fn eval(&self, z : f64, prev : &HashMap<String, f64>) -> f64;
}

enum Elem {
    One,
    Z,
    Ref(String)
}

impl Eval for Elem {
 fn eval(&self, z : f64, prev : &HashMap<String, f64>) -> f64 {
     match self {
	 Elem::One => 1.0,
	 Elem::Z => z,
	 Elem::Ref(s) => match prev.get(s) {
	     Some(&x) => x,
	     None => panic!("Rule not found (please report)")
	 }
     }
 }
}

enum RHS<'a> {
    RElem(Elem),
    RSum(&'a Vec<Elem>),
    RProd(&'a Vec<Elem>)
}

impl<'a> Eval for RHS<'a> {
 fn eval(&self, z : f64, prev : &HashMap<String, f64>) -> f64 {
     match self {
	 RHS::RElem(e) => e.eval(z, prev),
	 RHS::RSum(v) => v.into_iter().map(|e| e.eval(z, prev)).sum(),
	 RHS::RProd(v) => v.into_iter().map(|e| e.eval(z, prev)).product()
     }
 }
}

/*

struct Rule {
    build : bool,
    rhs : RHS
}

*/
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_One() {
	assert_eq!(Elem::One.eval(0.5, &HashMap::new()), 1.0);
	assert_eq!(Elem::One.eval(2.5, &HashMap::new()), 1.0);
    }

    #[test]
    fn test_eval_Z() {
	assert_eq!(Elem::Z.eval(0.5, &HashMap::new()), 0.5);
	assert_eq!(Elem::Z.eval(1.0, &HashMap::new()), 1.0);
    }

    #[test]
    fn test_eval_Ref() {
	let mut prev : HashMap<String, f64> = HashMap::new();
	prev.insert("node".to_string(), 2.0);
	prev.insert("tip".to_string(), 1.0);
	assert_eq!(Elem::Ref("node".to_string()).eval(4.0, &prev), 2.0);
    }

    #[test]
    fn test_eval_RHS() {
	let mut prev : HashMap<String, f64> = HashMap::new();
	assert_eq!(RHS::RElem(Elem::Z).eval(1.5, &prev), 1.5);

	prev.insert("node".to_string(), 2.0);
	prev.insert("tip".to_string(), 1.0);
	let v = vec![Elem::Ref("node".to_string()), Elem::Ref("tip".to_string()), Elem::Z];
	let rhs2 = RHS::RSum(&v);
	assert_eq!(rhs2.eval(1.5, &prev), 4.5);
	let rhs3 = RHS::RProd(&v);
	assert_eq!(rhs3.eval(1.5, &prev), 3.0);
    }
}
