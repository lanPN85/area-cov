use rand::distributions::{Range, IndependentSample};
use rand;

use models::*;

pub fn random_points(conf: &Configuration, size: i32) -> Vec<Point> {
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

pub fn random_state(conf: &Configuration) -> Vec<Point> {
	random_points(conf, conf.n)
}

pub fn random_init(conf: &Configuration, size: i32) -> Vec<Vec<Point>> {
	let mut states: Vec<Vec<Point>> = Vec::new();
	for _ in 0..size {
		let s = random_state(&conf);
		states.push(s);
	}

	vfa(conf, &mut states);
	normalize(conf, &mut states);

	states
}

fn vfa(conf: &Configuration, states: &mut Vec<Vec<Point>>) {
	/* Applies the virtual force algorithm to a set of states */
}

fn normalize(conf: &Configuration, states: &mut Vec<Vec<Point>>) {
	/* Normalizes states to conform to area boundaries */
	for state in states {
		let mut _i = 0;
		for i in 0..conf.counts.len() {
			let count = conf.counts[i];
			let radius = conf.radius[i];
			for j in _i..(_i + count) {
				let p = &mut state[j as usize];
				if (p.x + radius) > conf.w {
					p.x = conf.w - radius;
				}
				if (p.x - radius) < 0. {
					p.x = radius;
				}
				if (p.y + radius) > conf.h {
					p.y = conf.h - radius;
				}
				if (p.y - radius) < 0. {
					p.y = radius;
				}
			}
			_i += count;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_random_init() {
		let mut conf = Configuration::new();
		conf.w = 100.; conf.h = 100.;
		conf.n = 3;
		let v = random_init(&conf, 20);
		println!("{:?}", v);
		assert_eq!(v.len(), 20);
	}

	#[test]
	fn test_random_state() {
		let mut conf = Configuration::new();
		conf.w = 100.; conf.h = 100.;
		conf.n = 10;
		let v = random_state(&conf);
		println!("{:?}", v);
		assert_eq!(v.len(), conf.n as usize);
	}
}
