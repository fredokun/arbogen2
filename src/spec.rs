
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

/*
enum RHS {
    RElem(Elem),
    RSum([Elem]),
    RProd([Elem])
}

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
}
