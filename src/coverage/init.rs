use rand::distributions::{Range, IndependentSample};
use rand;

use coverage::models::*;

pub fn random_init(conf: &Configuration, size: i32) -> Vec<Point> {
	let mut state: Vec<Point> = Vec::new();
	let mut r = rand::thread_rng();
	let rngx = Range::new(0 as f64, conf.w as f64);
	let rngy = Range::new(0 as f64, conf.h as f64);

	for _ in 0..size {
		let p = Point{
			x: rngx.ind_sample(&mut r) as f32, 
			y: rngy.ind_sample(&mut r) as f32
		};
		state.push(p);
	}

	state
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_random_init() {
		let mut conf = Configuration::new();
		conf.w = 100.; conf.h = 100.;
		let v = random_init(&conf, 20);
		println!("{:?}", v);
		assert_eq!(v.len(), 20);
	}
}
