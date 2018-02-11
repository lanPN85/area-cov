use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::prelude::*;
use std::path::Path;

use models::*;
use init;

pub fn config_from_file(path: &Path) -> Configuration {
	let mut c = Configuration::new();

	let f = File::open(path)
		.expect("File not found.");
	let reader = io::BufReader::new(f); 
	let lines : Vec<_> = reader.lines()
		.map(|l| l.expect("Could not parse line"))
		.collect();

	// W H
	let l = &lines[0];
	let split : Vec<_> = l.split(" ").collect();
	c.w = split[0].parse::<f32>()
		.expect("Failed to parse line");
	c.h = split[1].parse::<f32>()
		.expect("Failed to parse line");

	// k n
	let l = &lines[1];
	let split : Vec<_> = l.split(" ").collect();
	let k = split[0].parse::<i32>()
		.expect("Failed to parse line");
	let n = split[1].parse::<i32>()
		.expect("Failed to parse line");
	c.n = n;

	// n1 ... nk
	for i in 0 as usize..k as usize {
		let n = lines[i+2].parse::<i32>()
			.expect("Failed to parse line");
		c.counts.push(n);
	}

	// r1 ... rk
	for i in 0 as usize ..k as usize {
		let r = lines[i + k as usize + 2].parse::<f32>()
			.expect("Failed to parse line");
		c.radius.push(r);
	}

	c
}

pub fn result_to_file(conf: &Configuration, result: &Vec<Point>, path: &Path) -> Result<(), io::Error> {
	let mut f = File::create(path).expect("Can't create file.");
	let mut s = String::new();
	let circles = Circle::from_state(conf, result);

	s += &format!("{} {}\n", conf.w, conf.h);
	for c in circles {
		s += &format!("{} {} {}\n", c.center.x, c.center.y, c.radius);
	}

	f.write_all(s.as_bytes())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_file_inp() {
		let fname = Path::new("data/ega/s1-07.in");
		let cf = config_from_file(&fname);
		
		println!("{:?}", cf);

		assert_eq!(cf.w, 100.);
		assert_eq!(cf.h, 100.);
		assert_eq!(cf.n, 17);
		assert_eq!(cf.counts.len(), 3);
		assert_eq!(cf.radius.len(), 3);
	}

	#[test]
	fn test_file_out() {
		let fname = Path::new("data/test.out");
		let cf = config_from_file(&Path::new("data/ega/s1-07.in"));
		let results = &init::random_init(&cf, 1)[0];

		match result_to_file(&cf, results, &fname) {
			Err(_) => panic!("File reading failed"),
			Ok(_) => {}
		};
	}
}
