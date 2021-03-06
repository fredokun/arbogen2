
use std::collections::HashMap;
use crate::utils::Values;

pub trait Eval {
    fn eval(&self, z : f64, prev : &Values) -> f64;
}

#[derive(Debug, Clone)]
pub enum Elem {
    One,
    Z,
    Ref(String)
}

impl Eval for Elem {
 fn eval(&self, z : f64, prev : &Values) -> f64 {
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

#[derive(Debug, Clone)]
pub enum RHS {
    Elem(Elem),
    Sum(Vec<(Elem, f64)>),
    Prod(Vec<Elem>)
}

impl Eval for RHS {
 fn eval(&self, z : f64, prev : &Values) -> f64 {
     match self {
	 RHS::Elem(e) => e.eval(z, prev),
	 RHS::Sum(v) => v.into_iter().map(|(e,_)| e.eval(z, prev)).sum(),
	 RHS::Prod(v) => v.into_iter().map(|e| e.eval(z, prev)).product()
     }
 }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub build : bool,
    pub rhs : RHS
}

pub type Spec = HashMap<String, Rule>;

pub fn eval_spec(spec : &Spec, z : f64, prev : &Values) -> Values {
    let mut ev : Values = HashMap::new();
    for (rname, rule) in spec.iter() {
	ev.insert(rname.clone(), rule.rhs.eval(z, prev));
    }
    return ev;
}

pub fn btree_spec() -> Spec {
    let mut spec : HashMap<String, Rule> = HashMap::new();
    let v1 : Vec<(Elem, f64)> = vec![(Elem::Ref("tip".to_string()), 0.0), (Elem::Ref("node".to_string()), 0.0)];
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
    	let mut prev : Values = HashMap::new();
    	prev.insert("node".to_string(), 2.0);
    	prev.insert("tip".to_string(), 1.0);
    	assert_eq!(Elem::Ref("node".to_string()).eval(4.0, &prev), 2.0);
    }

    #[test]
    fn test_eval_rhs() {
    	let mut prev : Values = HashMap::new();
    	assert_eq!(RHS::Elem(Elem::Z).eval(1.5, &prev), 1.5);

    	prev.insert("node".to_string(), 2.0);
    	prev.insert("tip".to_string(), 1.0);
    	let v2 = vec![(Elem::Ref("node".to_string()), 0.0), (Elem::Ref("tip".to_string()), 0.0), (Elem::Z, 0.0)];
    	let rhs2 = RHS::Sum(v2);
    	assert_eq!(rhs2.eval(1.5, &prev), 4.5);
    	let v3 = vec![Elem::Ref("node".to_string()), Elem::Ref("tip".to_string()), Elem::Z];
    	let rhs3 = RHS::Prod(v3);
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
