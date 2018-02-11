use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Point {
	pub x: f32, pub y: f32
}

impl Point {
	pub fn distance(&self, other: &Point) -> f32 {
		let x = (self.x - other.x) * (self.x - other.x);
		let y = (self.y - other.y) * (self.y - other.y);
		(x + y).sqrt()
	}

	pub fn equals(&self, other: &Point) -> bool {
		self.x == other.x && self.y == other.y
	}

	pub fn wrap(value: f32) -> Point {
		Point {
			x: value, y: value
		}
	}

	pub fn all_equal(v1: &Vec<Point>, v2: &Vec<Point>) -> bool {
		let mut b = true;
		for i in 0..v1.len() {
			if !v1[i].equals(&v2[i]) {
				b = false;
				break;
			}
		}
		b
	}
}

impl Add for Point {
	type Output = Point;
	fn add(self, other: Point) -> Point {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl AddAssign for Point {
	fn add_assign(&mut self, other: Point) {
		self.x += other.x;
		self.y += other.y;
	}
}

impl Sub for Point {
	type Output = Point;
	fn sub(self, other: Point) -> Point {
		Point{x: self.x - other.x, y: self.y - other.y}
	}
}

impl SubAssign for Point {
	fn sub_assign(&mut self, other: Point) {
		self.x -= other.x;
		self.y -= other.y;
	}
}

impl Div for Point {
	type Output = Point;
	fn div(self, other: Point) -> Point {
		Point{x: self.x / other.x, y: self.y / other.y}
	}
}

impl DivAssign for Point {
	fn div_assign(&mut self, other: Point) {
		self.x /= other.x;
		self.y /= other.y;
	}
}

impl Mul for Point {
	type Output = Point;
	fn mul(self, other: Point) -> Point {
		Point{x: self.x * other.x, y: self.y * other.y}
	}
}

impl MulAssign for Point {
	fn mul_assign(&mut self, other: Point) {
		self.x *= other.x;
		self.y *= other.y;
	}
}
