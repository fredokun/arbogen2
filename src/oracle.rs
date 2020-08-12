
use std::collections::HashMap;

use crate::utils::{Values, norm_dist};
use crate::spec::{Spec, eval_spec};

/*
(defn iter [gram z eps debug]
  (loop [v1 (mapkv (fn [k _] [k 0.0]) gram)]
    (let [v2 (eval-grammar gram z v1)]
      ;; (when debug (println "[iter] v2=" v2 "norm=" (norm v1 v2)))
      (if (<= (norm v1 v2) eps)
        v2
        (recur v2)))))
*/

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
}
