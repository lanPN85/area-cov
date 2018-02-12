use std::f32;
use rand::distributions::{IndependentSample, Range};
use rand;

use models::*;
use models::point::Point;

pub fn blx_alpha(p1: &Vec<Point>, p2: &Vec<Point>, alpha: f32) -> Vec<Point> {
	let mut child: Vec<Point> = Vec::new();
	let mut r = rand::thread_rng();
	
	for i in 0..p1.len() {
		let start_x = f32::min(p1[i].x, p2[i].x);
		let end_x = f32::max(p1[i].x, p2[i].x) + 1e-3; // Prevent error when start_x == end_x
		let start_y = f32::min(p1[i].y, p2[i].y);
		let end_y = f32::max(p1[i].y, p2[i].y) + 1e-3; // Prevent error when start_y == end_y
		let d_x = end_x - start_x;
		let d_y = end_y - start_y;
		let rx = Range::new(start_x - alpha * d_x, end_x + alpha * d_x);
		let ry = Range::new(start_y - alpha * d_y, end_y + alpha * d_y);
		child.push(Point{x: rx.ind_sample(&mut r), y: ry.ind_sample(&mut r)});
	}

	child
}

/// Creates a new pair of parents from p1 and p2 that have the optimal per-type distance
#[allow(unused_variables)]
pub fn homogenize(conf: &Configuration, p1: &Vec<Point>, p2: &Vec<Point>) -> (Vec<Point>, Vec<Point>) {
	// TODO Implement Hugarian method

	(p1.clone(), p2.clone())
}

#[cfg(test)]
mod tests {
	use super::*;
	use init::random_init;

	#[test]
	fn test_blx() {
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
		assert_eq!(pc.len(), p[0].len());
	}
}

