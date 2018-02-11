use quickersort::sort_by_key;

use models::*;
use models::point::Point;

pub fn best(adapt_fn: &Fn(&Configuration, &Vec<Point>)->f32, conf: &Configuration, 
		states: &Vec<Vec<Point>>, size: usize) -> Vec<Vec<Point>> {
	let mut new_states = states.clone();
	// Convert float key to integer with 1e-5 accuracy
	sort_by_key(&mut new_states, |s| (-1e5 * adapt_fn(conf, s)) as i32);
	new_states = new_states[0..size].to_vec();

	new_states
}

#[cfg(test)]
mod tests {
	use super::*;
	use models::adaptive::overlap_fs;
	use init::heuristic_init;

	#[test]
	fn test_best_selector() {
		let mut conf = Configuration::new();
		conf.w = 60.; conf.h = 100.;
		conf.n = 3;
		conf.counts = vec![1, 2];
		conf.radius = vec![10., 20.];
		let v = heuristic_init(&conf, 10);
		println!("{:?}", &v[0]);

		let nv = best(&overlap_fs, &conf, &v, 5);
		println!("{:?}", &nv[0]);
		assert_eq!(nv.len(), 5);
	}
}
