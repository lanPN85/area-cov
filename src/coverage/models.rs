#[derive(Debug)]
pub struct Point {
	pub x: f32, pub y: f32
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

pub fn coverage_area(conf: Configuration, state: Vec<Point>) -> f32 {
	/* Calculates coverage area using Monte Carlo method */
	let L = 10000;
	let aS = conf.h * conf.w / (L as f32);
	let mut total = 0.0;

	0.0
}
