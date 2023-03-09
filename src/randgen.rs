
use rand::prelude::*;
use rand_xoshiro::Xoshiro256StarStar;

use crate::spec::{Spec, RHS, Elem, btree_spec};
use crate::oracle::{oracle, weighted_spec};

// TODO: introduce a trait so that it's easier
// to change the random generator
type RandGen = Xoshiro256StarStar;

pub fn make_rng(seed : u64) ->  RandGen {
    return RandGen::seed_from_u64(seed);
}

fn choose(rng : &mut RandGen, elems : &Vec<(Elem, f64)>) -> Elem {
    let x : f64 = rng.gen();
    for (elem, proba) in elems.iter() {
	if x <= *proba {
	    return elem.clone();
	}
    }
    panic!("Choose should not fail (please report)");
}

fn next_size(rng : &mut RandGen, spec : &Spec, rname :&str, max_size : u64) -> Option<u64> {
    let mut elems : Vec<Elem> = Vec::new();
    let elem = Elem::Ref(rname.to_string());
    elems.push(elem);
    let mut size : u64 = 0;

    while size < max_size {
	let elem = elems.pop();
	match elem {
	    None => { return Some(size); },
	    Some(Elem::One) => (),
	    Some(Elem::Z) => { size += 1; },
	    Some(Elem::Ref(rname)) => {
		let rhs = &spec.get(&rname.to_string()).unwrap().rhs;
		match rhs {
		    RHS::Elem(elem) => { elems.push(elem.clone()); },
		    RHS::Sum(choices) => { 
			//+ (let [[src' elem'] (choose src args)]
			//   (recur src' elem' size cont))
			
			let elem = choose(rng, choices);
			elems.push(elem);
		    },
		    RHS::Prod(pelems) => {
			for pelem in pelems.iter() {
			    elems.push(pelem.clone());
			}
		    }	
		};
	    }
	};
    }

    // too big
    return None;
}

fn search_size(rng_source : &RandGen, spec : &Spec, rname :&str,
	       min_size : u64, max_size : u64, max_attempts : u32) -> Option<(u64, RandGen)> {
    let mut attempts = 0;

    let mut rng = rng_source.clone();

    while attempts < max_attempts {
	attempts += 1;
	let save_rng = rng.clone();
	match next_size(&mut rng, &spec, rname, max_size) {
	    None => { println!("Not found"); },
	    Some(size) => 
		if min_size <= size && size <= max_size {
		    return Some((size, save_rng));
		}
	};
    }
    
    return None;
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
	let mut rng2 = rng.clone(); // XXX: serialization not need because cloning works...
	let serialized = serde_json::to_string(&rng).unwrap();
	let x : f64 = rng.gen();
	let y : f64 = rng.gen();
	assert!(x != y);
	let mut deserialized : RandGen = serde_json::from_str(&serialized).unwrap();
	let z : f64 = deserialized.gen();
	assert_eq!(x, z);
	let u : f64 = rng2.gen();
	assert_eq!(x, u);
	let v : f64 = rng2.gen();
	assert_eq!(y, v);
    }

    #[test]
    fn test_next_size() {
	let mut rng = make_rng(42);
	let btspec = btree_spec();
	let (z, v) = oracle(&btspec, 0.0, 1.0, 0.00001, 0.000001);
	let btspec = weighted_spec(btspec, z, &v);
	match next_size(&mut rng, &btspec, "btree", 1000) {
	    None => assert!(false),
	    Some(size) => assert_eq!(size, 0)
	};
	match next_size(&mut rng, &btspec, "btree", 1000) {
	    None => assert!(false),
	    Some(size) => assert_eq!(size, 0)
	};
	match next_size(&mut rng, &btspec, "btree", 1000) {
	    None => assert!(false),
	    Some(size) => assert_eq!(size, 410)
	};
	match next_size(&mut rng, &btspec, "btree", 1000) {
	    None => assert!(false),
	    Some(size) => assert_eq!(size, 0)
	};
	match next_size(&mut rng, &btspec, "btree", 1000) {
	    None => assert!(false),
	    Some(size) => assert_eq!(size, 1)
	};
	match next_size(&mut rng, &btspec, "btree", 1000) {
	    None => assert!(false),
	    Some(size) => assert_eq!(size, 13)
	};
	match next_size(&mut rng, &btspec, "btree", 1000) {
	    None => assert!(false),
	    Some(size) => assert_eq!(size, 0)
	};
    }

    #[test]
    fn test_search_size() {
	let rng = make_rng(42);
	let btspec = btree_spec();
	let (z, v) = oracle(&btspec, 0.0, 1.0, 0.00001, 0.000001);
	let btspec = weighted_spec(btspec, z, &v);
	// 1) we search a binary tree of size 1000->10000
	match search_size(&rng, &btspec, "btree", 1000, 10000, 1000) {
	    None => assert!(false),
	    Some((size, mut rng)) => {
		// 2) we found a tree of size 1621
		assert_eq!(size, 1621);
		// 3) we check that the first size with the same randgen 
		//    produces the same tree (same size)
		match next_size(&mut rng, &btspec, "btree", 10000) {
		    None => assert!(false),
		    Some(size) => assert_eq!(size, 1621) // <-- same size !
		};
	    }
	};
    }
}

