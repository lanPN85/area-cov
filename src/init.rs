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
	static PUSH_ALPHA: Point = Point{x: 1., y: 1.};
	static PULL_ALPHA: Point = Point{x: 1., y: 1.};
	static ONE: Point = Point{x: 1., y: 1.};

	for state in states {
		let circles = Circle::from_state(conf, state);

		// Adds boundary edges to list of candidates
		let mut cand = circles.clone();
		cand.extend(vec![
			Circle{center: Point{x: 0., y: 0.}, radius: 0.},
			Circle{center: Point{x: 0., y: conf.h}, radius: 0.},
			Circle{center: Point{x: conf.w, y: 0.}, radius: 0.},
			Circle{center: Point{x: conf.w, y: conf.h}, radius: 0.} 
		]);

		for i in 0..circles.len() {
			let c = &circles[i];
			let mut fpull = Point::wrap(0.); 
			let mut fpush = Point::wrap(0.);
			let mut num_pull = Point::wrap(0.);
			let mut num_push = Point::wrap(0.);

			for j in 0..cand.len() {
				let ca = &cand[j];

				let d = c.center.distance(&ca.center);
				let _d = Point{x: d, y: d};
				let sum = Point{x: c.radius + ca.radius, y: c.radius + ca.radius};
				if d == 0. {
					continue;
				}

				if d < (c.radius + ca.radius) {	
					fpush = fpush + (ONE - sum / _d) * (ca.center - c.center);
					num_push += ONE;
				} else {
					fpull = fpull + (ONE - sum / _d) * (ca.center - c.center);
					num_pull += ONE;
				}
			}

			let mut push = Point::wrap(0.);
			if !num_push.equals(&Point::wrap(0.)) {
				push = PUSH_ALPHA * fpush / num_push;
			};
			let mut pull = Point::wrap(0.);
			if !num_pull.equals(&Point::wrap(0.)) {
				push = PULL_ALPHA * fpull / num_pull;
			};
			state[i] = state[i] + push + pull;
		}
	}
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
		println!("{:?}", &v[0]);
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
