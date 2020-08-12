
use std::collections::HashMap;

use crate::utils::{Values, norm_dist};
use crate::spec::{Spec, eval_spec};

fn iteration(spec : &Spec, z : f64, eps : f64) -> Values {
    let mut v1 = HashMap::new();
    for (rname, _) in spec.iter() {
	v1.insert(rname.to_string(), 0.0);
    }

    let mut v2 = eval_spec(&spec, z, &v1);

    while norm_dist(&v1, &v2) >= eps {
	v1 = v2;
	v2 = eval_spec(&spec, z, &v1);
    }

    return v2;
} 

fn diverge(v : &Values, eps : f64) -> bool {
    for (_, &w) in v.iter() {
	if w < 0.0 || w > 1.0 / eps {
	    return true;
	}
    }
    return false;
}

fn oracle(spec : &Spec, zmin : f64, zmax : f64, eps_iter : f64, eps_div : f64) -> (f64, Values) {
    let mut zinf = zmin;
    let mut zsup = zmax;

    while zsup - zinf >= eps_iter {
	let z = (zsup + zinf) / 2.0;
	let v = iteration(spec, z, eps_div);
	if diverge(&v, eps_div) {
	    zsup = z;
	} else {
	    zinf = z;
	}
    }

    (zinf, iteration(spec, zinf, eps_div))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::btree_spec; 

    #[test]
    fn test_iteration() {
	let spec = btree_spec();
	let v = &iteration(&spec, 0.251953125, 0.01);
	assert_eq!(v.get("tip").unwrap(), &1.0);
	assert_eq!(v.get("btree").unwrap(), &1.8937299943702706);
	assert_eq!(v.get("node").unwrap(), &0.9035576457295188);
    }

    #[test]
    fn test_oracle() {
	let spec = btree_spec();
	let (z, _) = oracle(&spec, 0.0, 1.0, 0.001, 0.000001);
	assert_eq!(z, 0.25);
    }

}
