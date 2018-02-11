use std::f32;
use init::{random_init, random_points};

use super::*;

/// Calculates coverage area using Monte Carlo method
pub fn coverage_area(conf: &Configuration, state: &Vec<Point>) -> f32 {
	let l = 10000.;
	let a_s = conf.h * conf.w / l;
	let mut total = 0.0;

	let circles: Vec<Circle> = Circle::from_state(conf, state);

	let points = random_points(conf, l as i32);
	for p in points {
		let mut covered = false;
		for j in 0..circles.len() {
			let c = &circles[j];
			if c.contains(&p) {
				covered = true;
				break;
			}
		}
		if covered {
			total += 1.;
		}
	}

	a_s * total
}

/// Calculates overlap index
pub fn overlap(conf: &Configuration, state: &Vec<Point>) -> f32 {
	let mut ov = 0.;
	let circles = Circle::from_state(conf, state);
	let corners = vec![
		Point{x: 0., y: 0.},
		Point{x: 0., y: conf.h},
		Point{x: conf.w, y: 0.},
		Point{x: conf.w, y: conf.h}
	];

	// Calculates beta
	let mut _r1 = 0.; let mut _r2 = 0.;
	let mut _rk = f32::MAX;
	for c in &circles {
		if c.radius >= _r1 {
			_r2 = _r1;
			_r1 = c.radius;
		}
		if c.radius < _rk {
			_rk = c.radius;
		}
	}
	let beta = ((_r1 + _r2) * _r2) / (_r1 * _rk) + 0.01;

	for c1 in &circles {
		for c2 in &circles {
			let d = c1.center.distance(&c2.center);
			if d == 0. {
				continue;
			}

			if d < (c1.radius + c2.radius) && d >= (c1.radius - c2.radius).abs() {
				let gamma = ((c1.radius + c2.radius) * f32::min(c1.radius, c2.radius)) / 
							(_r1 * f32::max(c1.radius, c2.radius));
				ov += gamma * (c1.radius + c2.radius - d);
			} else if d < (c1.radius - c2.radius).abs() {
				ov += beta * f32::min(c1.radius, c2.radius);
			}
		}
		for p in &corners {
			let d = c1.center.distance(&p);
			if d < c1.radius {
				ov += (c1.radius - d) * c1.radius;
			}
		}
	}
	
	ov
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_coverage_area() {
		let conf = Configuration {
			w: 20., h: 50., n: 3,
			counts: vec![1, 2],
			radius: vec![10., 20.]
		};
		let state = &random_init(&conf, 1)[0];
		println!("{:?}", state);

		let ca = coverage_area(&conf, &state);
		println!("{:?}", ca);
	}

	#[test]
	fn test_overlap() {
		let conf = Configuration {
			w: 20., h: 50., n: 3,
			counts: vec![1, 2],
			radius: vec![10., 20.]
		};
		let state = &random_init(&conf, 1)[0];
		println!("{:?}", state);

		let ov = overlap(&conf, &state);
		println!("{:?}", ov);
	}
}
