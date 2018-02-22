use std::fmt::Debug;
use std::collections::VecDeque;
use std::{f32, i32};

struct HugarianSolver<T: Clone+Debug> {
	pub g1: Vec<T>, pub g2: Vec<T>,
	matches: i32, n: i32,
	lx: Vec<f32>, ly: Vec<f32>,
	pub xy: Vec<i32>, pub yx: Vec<i32>,
	q: VecDeque<i32>, prev: Vec<i32>,
	s: Vec<bool>, t: Vec<bool>,
	slack: Vec<f32>, slackx: Vec<i32>
}

impl<T: Clone+Debug> HugarianSolver<T> {
	pub fn new(g1: &Vec<T>, g2: &Vec<T>,
			weight_fn: &Fn(&T, &T)->f32) -> HugarianSolver<T> {
		let _n = g1.len();

		// Initial labeling
		let mut _lx: Vec<f32> = Vec::new();
		let mut _ly: Vec<f32> = Vec::new();
		for i in 0.._n {
			_lx.push(0.); _ly.push(0.);
			for j in 0.._n {
				_lx[i] = f32::max(_lx[i], weight_fn(&g1[i], &g2[j]));
			}
		}

		HugarianSolver {
			g1: g1.clone(), g2: g2.clone(), 
			matches: 0, n: _n as i32,
			lx: _lx, ly: _ly,
			xy: vec![-1; _n],
			yx: vec![-1; _n],
			q: VecDeque::new(),
			s: vec![false; _n], t: vec![false; _n],
			slack: vec![0.;_n], slackx: vec![0; _n],
			prev: vec![-1; _n]
		}
	}

	pub fn augment(&mut self, weight_fn: &Fn(&T, &T)->f32) -> bool {
		if self.matches >= self.n {
			return true;
		}

		// Find root
		let mut root: i32 = 0;
		let mut x = 0;
		while x < self.n {
			let _x = x as usize; 
			if self.xy[_x] == -1 {
				root = x;
				self.q.push_back(root);
				self.prev[_x] = -2;
				self.s[_x] = true;
				break;
			}
			x += 1;
		}

		// Initialize slack
		for y in 0..self.n {
			let _y = y as usize;
			let _root = root as usize;
			self.slack[_y] = self.lx[_root] + self.ly[_y] - weight_fn(&self.g1[_root], &self.g2[_y]);
			self.slackx[_y] = root;
		}

		let mut stop = false;
		let mut y = 0;
		let threshold: i32 = i32::pow(self.n, 3);
		let mut runs = 0;
		loop {
			if runs >= threshold {
				return false;
			}
			runs += 1;

			// Build tree w/ BFS
			while self.q.len() > 0 {
				x = self.q.pop_front().unwrap();
				let _x = x as usize;
				y = 0;
				while y < self.n {
					let _y = y as usize;
					if (weight_fn(&self.g1[_x], &self.g2[_y]) == self.lx[_x] + self.ly[_y]) && !self.t[_y] {
						if self.yx[_y] == -1 {
							// Found exposed vertex
							stop = true;
							break;
						}

						// Add to T
						self.t[_y] = true;
						self.q.push_back(self.yx[_y]);
						let _v = self.yx[_y];
						self.add_to_tree(_v, x, weight_fn);
					}
					y += 1;
				}
				if stop {break;}
			}
			if stop {break;}

			// Augmenting path not found
			self.update_labels();
			self.q.clear();

			y = 0;
			while y < self.n {
				let _y = y as usize;
				if !self.t[_y] && (self.slack[_y].abs() < 1e-4) {
					if self.yx[_y] == -1 {
						x = self.slackx[_y];
						break;
					} else {
						self.t[_y] = true;
						if !self.s[self.yx[_y] as usize] {
							self.q.push_back(self.yx[_y]);
							let _v1 = self.yx[_y]; let _v2 = self.slackx[_y];
							self.add_to_tree(_v1, _v2, weight_fn);
						}
					}
				}
				y += 1;
			}
			if y < self.n {
				stop = true;
				break;
			}
		}

		// Found augmenting path
		if stop { 
			self.matches += 1;
			let mut cx = x;
			let mut cy = y;
			while cx != -2 {
				let ty = self.xy[cx as usize];
				self.yx[cy as usize] = cx;
				self.xy[cx as usize] = cy;
				cx = self.prev[cx as usize];
				cy = ty;
			}
			return self.augment(weight_fn);
		} else { return false; }
	}

	fn add_to_tree(&mut self, x: i32, prevx: i32, weight_fn: &Fn(&T, &T)->f32) {
		let _x = x as usize;
		self.s[_x] = true;
		self.prev[_x] = prevx;
		for y in 0..self.n {
			let _y = y as usize;
			let cost = weight_fn(&self.g1[_x], &self.g2[_y]);
			if self.lx[_x] + self.ly[_y] - cost < self.slack[_y] {
				self.slack[_y] = self.lx[_x] + self.ly[_y] - cost;
				self.slackx[_y] = x as i32;
			}
		}
	}

	fn update_labels(&mut self) {
		let mut delta = f32::MAX;
		for y in 0..self.n {
			let _y = y as usize;
			if !self.t[_y] {
				delta = f32::min(delta, self.slack[_y]);
			}
		}
		for x in 0..self.n {
			let _x = x as usize;
			if self.s[_x] {
				self.lx[_x] -= delta;
			}
		}
		for y in 0..self.n {
			let _y = y as usize;
			if self.t[_y] {
				self.ly[_y] += delta;
			}
		}
		for y in 0..self.n {
			let _y = y as usize;
			if !self.t[_y] {
				self.slack[_y] -= delta;
			}
		}
	}
}

pub fn apply_hungarian<T: Clone+Debug>(g1: &mut Vec<T>, g2: &mut Vec<T>, weight_fn: &Fn(&T, &T)->f32) {
	let mut solver = HugarianSolver::new(g1, g2, weight_fn);
	let success = solver.augment(weight_fn);

	if success {
		for i in 0..g1.len() {
			g2[i] = solver.g2[solver.xy[i] as usize].clone();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use models::point::Point;

	#[test]
	fn test_hungarian() {
		let mut v1 = vec![
			Point{x: 1., y: 2.},
			Point{x: 1.5, y: 3.5},
			Point{x: 10., y: -2.}
		];
		let mut v2 = vec![
			Point{x: 1.5, y: 3.5},
			Point{x: 1., y: 2.},
			Point{x: 10.1, y: -4.},
		];

		apply_hungarian(&mut v1, &mut v2, &|p1, p2| -p1.distance(p2));
		assert_eq!(v2[0], Point{x: 1., y: 2.});
		assert_eq!(v2[1], Point{x: 1.5, y: 3.5});
		assert_eq!(v2[2], Point{x: 10.1, y: -4.});

		println!("{:?}", v1);
		println!("{:?}", v2);
	}
}
