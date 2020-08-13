
use rand::prelude::*;
use rand_xoshiro::Xoshiro256StarStar;

use crate::spec::{Elem};

pub fn make_rng(seed : u64) -> Xoshiro256StarStar {
    return Xoshiro256StarStar::seed_from_u64(seed);
}

fn choose(rng : &mut Xoshiro256StarStar, elems : &Vec<(Elem, f64)>) -> Elem {
    let x : f64 = rng.gen();
    for (elem, proba) in elems.iter() {
	if x <= *proba {
	    return elem.clone();
	}
    }
    panic!("Choose should not fail (please report)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose() {
	let mut rng = make_rng(42);
	let elems = vec![(Elem::Ref(":one".to_string()), 0.5),
			 (Elem::Ref(":two".to_string()), 0.8),
			 (Elem::Ref(":one".to_string()), 1.0)];
	let elem1 = choose(&mut rng, &elems);
	assert!(match elem1 {
	    Elem::Ref(_) => true,
	    _ => false
	});
    }

    #[test]
    fn test_serialize() {
	let mut rng = make_rng(42);
	let serialized = serde_json::to_string(&rng).unwrap();
	let x : f64 = rng.gen();
	let y : f64 = rng.gen();
	assert!(x != y);
	let mut deserialized : Xoshiro256StarStar = serde_json::from_str(&serialized).unwrap();
	let z : f64 = deserialized.gen();
	assert_eq!(x, z);
    }
}

