#[macro_use]
extern crate clap;
extern crate rand;
extern crate quickersort;
extern crate time;

mod fileio;
mod models;
mod ga;

use std::path::Path;
use clap::App;

use models::adaptive;
use models::point::Point;

fn main() {
    let yaml = load_yaml!("cli.yml");
	let args = App::from_yaml(yaml).get_matches();

	let fname = Path::new(args.value_of("INPUT").unwrap());
	let size = value_t!(args, "size", i32).unwrap();
	let cross_ratio = value_t!(args, "cross_ratio", f32).unwrap();
	let mutate_ratio = value_t!(args, "mutate_ratio", f32).unwrap();
	let iters = value_t!(args, "iters", i32).unwrap();
	let runs = value_t!(args, "runs", i32).unwrap();
	
	eprintln!("Reading '{}'...", fname.to_str().unwrap());
	let conf = fileio::config_from_file(&fname);
	let mut best_result: Vec<Point> = Vec::new();
	let mut best_cov: f32 = 0.;

	for r in 0..runs {
		println!("Run {}/{}", r+1, runs);
		let start = time::get_time();
		let results = ga::genetic_algorithm(&conf, size, iters, 
			cross_ratio, mutate_ratio);
		let elapsed = time::get_time() - start;
		let m_el = elapsed.num_milliseconds();
		let cov = adaptive::coverage_area(&conf, &results);
		
		if cov > best_cov {
			best_result = results;
			best_cov = cov;
		}

		eprintln!("-------------");
		eprintln!("Elapsed time: {}ms", m_el);
		eprintln!("Coverage area: {}/{}", cov, conf.h * conf.w);
		eprintln!("-------------");
	}

	if let Some(s) = args.value_of("out") {
		let oname = Path::new(&s);
		println!("Saving best result to {}", s);
		let success = fileio::result_to_file(&conf, &best_result, best_cov, &oname);
		match success {
			Err(_) => panic!("Unable to save output"),
			Ok(_) => println!("Done.")
		};
	}

	println!("** Best coverage: {}/{} **", best_cov, conf.h * conf.w);
}
