use init::{random_init, random_points};

#[derive(Debug, Clone)]
pub struct Point {
	pub x: f32, pub y: f32
}

impl Point {
	pub fn distance(&self, other: &Point) -> f32 {
		let x = (self.x - other.x) * (self.x - other.x);
		let y = (self.y - other.y) * (self.y - other.y);
		(x + y).sqrt()
	}
}

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

#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f32
}

impl Circle {
	pub fn contains(&self, p: &Point) -> bool {
		let d = self.center.distance(p);
		d <= self.radius
	}
}

pub fn coverage_area(conf: &Configuration, state: &Vec<Point>) -> f32 {
	/* Calculates coverage area using Monte Carlo method */
	let l = 10000.;
	let a_s = conf.h * conf.w / l;
	let mut total = 0.0;

	let mut circles: Vec<Circle> = Vec::new();
	{
		let mut _i = 0;
		for i in 0..conf.counts.len() {
			let count = conf.counts[i];
			for j in _i..(_i + count) {
				circles.push(Circle{
					center: state[j as usize].clone(), 
					radius: conf.radius[i]});
			}
			_i += count;
		}
	}

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
		let state = random_init(&conf, 1);
		println!("{:?}", state);

		let ca = coverage_area(&conf, &state[0]);
		println!("{:?}", ca);
	}
}
