use rand::distributions::{IndependentSample, Normal};
use rand;

use models::*;
use models::point::Point;

pub fn dynamic_gaussian(state: &Vec<Point>, p1: &Vec<Point>, p2: &Vec<Point>) -> Vec<Point> {
	let mut new_state = state.clone();
	let mut r = rand::thread_rng();
	let mean = 0.;

	for i in 0..new_state.len() {
		let sdev = (p1[i].x - p2[i].x).abs();
		let rn = Normal::new(mean as f64, sdev as f64);
		new_state[i].x += rn.ind_sample(&mut r) as f32;
		new_state[i].y += rn.ind_sample(&mut r) as f32;
	}

	new_state
}

pub fn static_gaussian(state: &Vec<Point>, _p1: &Vec<Point>, _p2: &Vec<Point>) -> Vec<Point> {
	let mut new_state = state.clone();
	let mut r = rand::thread_rng();
	let mean = 0.;
	let sdev = 50.;

	for i in 0..new_state.len() {
		let rn = Normal::new(mean as f64, sdev as f64);
		new_state[i].x += rn.ind_sample(&mut r) as f32;
		new_state[i].y += rn.ind_sample(&mut r) as f32;
	}

	new_state
}

#[cfg(test)]
mod tests {
	use super::*;
	use init::random_init;
	use ga::cross::blx_alpha;

	#[test]
	fn test_dynamic_gaussian() {
		let mut conf = Configuration::new();
		conf.w = 60.; conf.h = 100.;
		conf.n = 3;
		conf.counts = vec![1, 2];
		conf.radius = vec![10., 20.];
		
		let p = random_init(&conf, 2);
		println!("{:?}", &p[0]);
		println!("{:?}", &p[1]);

		let pc = blx_alpha(&p[0], &p[1], 0.5);
		println!("{:?}", pc);

		let px = dynamic_gaussian(&pc, &p[0], &p[1]);
		println!("{:?}", px);
		assert_eq!(px.len(), pc.len());
	}

	#[test]
	fn test_static_gaussian() {
		let mut conf = Configuration::new();
		conf.w = 60.; conf.h = 100.;
		conf.n = 3;
		conf.counts = vec![1, 2];
		conf.radius = vec![10., 20.];
		
		let p = random_init(&conf, 2);
		println!("{:?}", &p[0]);
		println!("{:?}", &p[1]);

		let pc = blx_alpha(&p[0], &p[1], 0.5);
		println!("{:?}", pc);

		let px = static_gaussian(&pc, &p[0], &p[1]);
		println!("{:?}", px);
		assert_eq!(px.len(), pc.len());
	}
}
