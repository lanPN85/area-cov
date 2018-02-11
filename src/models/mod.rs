pub mod point;
pub mod eval;

use self::point::*;

#[derive(Debug)]
pub struct Configuration {
	pub w: f32, pub h: f32,
	pub n: i32,
	pub counts: Vec<i32>,
	pub radius: Vec<f32>
}

impl Configuration {
	pub fn new() -> Configuration {
		Configuration {
			w: 0., h: 0., n: 0,
			counts: Vec::new(),
			radius: Vec::new(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f32
}

impl Circle {
	pub fn contains(&self, p: &Point) -> bool {
		let d = self.center.distance(p);
		d <= self.radius
	}

	pub fn from_state(conf: &Configuration, state: &Vec<Point>) -> Vec<Circle> {
		let mut v: Vec<Circle> = Vec::new();

		let mut _i = 0;
		for i in 0..conf.counts.len() {
			let count = conf.counts[i];
			for j in _i..(_i + count) {
				v.push(Circle{
					center: state[j as usize].clone(), 
					radius: conf.radius[i]});
			}
			_i += count;
		}

		v
	}
}
