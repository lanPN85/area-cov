use std::fs::File;
use std::io;
use std::io::BufRead;

use models;

pub fn config_from_file(path: String) -> models::Configuration {
	let mut c = models::Configuration::new();

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_file_inp() {
		let fname = String::from("data/ega/s1-07.in");
		let cf = config_from_file(fname);
		
		println!("{:?}", cf);

		assert_eq!(cf.w, 100.);
		assert_eq!(cf.h, 100.);
		assert_eq!(cf.n, 17);
		assert_eq!(cf.counts.len(), 3);
		assert_eq!(cf.radius.len(), 3);
	}
}
