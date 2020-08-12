
use std::collections::HashMap;

pub type Values = HashMap<String, f64>;

pub fn norm_dist(v1 : &Values, v2 : &Values) -> f64 {
    let mut dist = 0.0;
    for (rname, y1) in v1.iter() {
	let y2 = v2.get(rname).unwrap();
	let y = (y1 - y2).abs();
	if y > dist {
	    dist = y
	}
    }
    return dist;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_dist() {
	// (norm {:a 0.1 :b 0.3} {:a 0.2 :b -0.2})
	let mut v1 = HashMap::new();
	v1.insert("a".to_string(), 0.1);
	v1.insert("b".to_string(), 0.3);
	let mut v2 = HashMap::new();
	v2.insert("a".to_string(), 0.2);
	v2.insert("b".to_string(), -0.2);
	assert_eq!(norm_dist(&v1, &v2), 0.5);
    }

}
