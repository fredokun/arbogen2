
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

