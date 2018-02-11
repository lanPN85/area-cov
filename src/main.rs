#[macro_use]
extern crate clap;
extern crate rand;
extern crate quickersort;
extern crate time;

mod fileio;
#[allow(dead_code)]
mod init;
mod models;
mod ga;

use std::path::Path;
use clap::App;

use models::adaptive;

fn main() {
    let yaml = load_yaml!("cli.yml");
	let args = App::from_yaml(yaml).get_matches();

	let fname = Path::new(args.value_of("INPUT").unwrap());
	let size = value_t!(args, "size", i32).unwrap();
	let cross_ratio = value_t!(args, "cross_ratio", f32).unwrap();
	let mutate_ratio = value_t!(args, "mutate_ratio", f32).unwrap();
	let iters = value_t!(args, "iters", i32).unwrap();

	let conf = fileio::config_from_file(&fname);
	let start = time::get_time();
	let results = ga::genetic_algorithm(&conf, size, iters, 
		cross_ratio, mutate_ratio);
	let elapsed = time::get_time() - start;
	let m_el = elapsed.num_milliseconds();
	let cov = adaptive::coverage_area(&conf, &results);
	
	println!("");
	println!("Elapsed time: {}ms", m_el);
	println!("Coverage area: {}/{}", cov, conf.h * conf.w);
}
