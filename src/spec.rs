
use std::collections::HashMap;

trait Eval {
    fn eval(&self, z : f64, prev : &HashMap<String, f64>) -> f64;
}

#[derive(Clone)]
pub enum Elem {
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

pub enum RHS {
    Elem(Elem),
    Sum(Vec<Elem>),
    Prod(Vec<Elem>)
}

impl Eval for RHS {
 fn eval(&self, z : f64, prev : &HashMap<String, f64>) -> f64 {
     match self {
	 RHS::Elem(e) => e.eval(z, prev),
	 RHS::Sum(v) => v.into_iter().map(|e| e.eval(z, prev)).sum(),
	 RHS::Prod(v) => v.into_iter().map(|e| e.eval(z, prev)).product()
     }
 }
}

pub struct Rule {
    build : bool,
    rhs : RHS
}

type Spec = HashMap<String, Rule>;

pub fn eval_spec(spec : &Spec, z : f64, prev : &HashMap<String, f64>) -> HashMap<String, f64> {
    let mut ev : HashMap<String, f64> = HashMap::new();
    for (rname, rule) in spec.iter() {
	ev.insert(rname.clone(), rule.rhs.eval(z, prev));
    }
    return ev;
}

fn btree_spec() -> Spec {
    let mut spec : HashMap<String, Rule> = HashMap::new();
    let v1 : Vec<Elem> = vec![Elem::Ref("tip".to_string()), Elem::Ref("node".to_string())];
    let rhs1 = RHS::Sum(v1);
    spec.insert("btree".to_string(),Rule { build: false, rhs: rhs1 });
    let v2 : Vec<Elem> = vec![Elem::Z, Elem::Ref("btree".to_string()), Elem::Ref("btree".to_string())];
    let rhs2 = RHS::Prod(v2);
    spec.insert("node".to_string(), Rule { build: true, rhs: rhs2 });
    spec.insert("tip".to_string(), Rule { build: true, rhs: RHS::Elem(Elem::One) });
    return spec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_one() {
	assert_eq!(Elem::One.eval(0.5, &HashMap::new()), 1.0);
	assert_eq!(Elem::One.eval(2.5, &HashMap::new()), 1.0);
    }

    #[test]
    fn test_eval_z() {
    	assert_eq!(Elem::Z.eval(0.5, &HashMap::new()), 0.5);
    	assert_eq!(Elem::Z.eval(1.0, &HashMap::new()), 1.0);
    }

    #[test]
    fn test_eval_ref() {
    	let mut prev : HashMap<String, f64> = HashMap::new();
    	prev.insert("node".to_string(), 2.0);
    	prev.insert("tip".to_string(), 1.0);
    	assert_eq!(Elem::Ref("node".to_string()).eval(4.0, &prev), 2.0);
    }

    #[test]
    fn test_eval_rhs() {
    	let mut prev : HashMap<String, f64> = HashMap::new();
    	assert_eq!(RHS::Elem(Elem::Z).eval(1.5, &prev), 1.5);

    	prev.insert("node".to_string(), 2.0);
    	prev.insert("tip".to_string(), 1.0);
    	let v = vec![Elem::Ref("node".to_string()), Elem::Ref("tip".to_string()), Elem::Z];
	let v2 = v.clone();
    	let rhs2 = RHS::Sum(v);
    	assert_eq!(rhs2.eval(1.5, &prev), 4.5);
    	let rhs3 = RHS::Prod(v2);
    	assert_eq!(rhs3.eval(1.5, &prev), 3.0);
    }

    #[test]
    fn test_eval_spec() {
	let btspec = btree_spec();
    	let mut prev : HashMap<String, f64> = HashMap::new();
    	prev.insert("btree".to_string(), 1.0);
    	prev.insert("node".to_string(), 2.0);
    	prev.insert("tip".to_string(), 1.0);
	let ev = eval_spec(&btspec, 1.0, &prev);
	let x = ev.get("tip").unwrap();
	assert_eq!(*x, 1.0);
    }
}
