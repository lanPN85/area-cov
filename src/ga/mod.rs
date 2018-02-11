#[allow(dead_code)]
pub mod cross;
#[allow(dead_code)]
pub mod mutate;
#[allow(dead_code)]
pub mod select;

use std::f32;
use rand;
use rand::distributions::{IndependentSample, Range};

use init;
use models::*;
use models::adaptive::*;
use models::point::*;
use self::cross::*;
use self::mutate::*;
use self::select::*;

pub fn genetic_algorithm(conf: &Configuration, size: i32, iters: i32,
						cross_ratio: f32, mutate_ratio: f32) -> Vec<Point> {
	const INIT_ALG: fn(&Configuration, i32)->Vec<Vec<Point>> = init::heuristic_init;
	const MUTATE_ALG: fn(&Vec<Point>, &Vec<Point>, &Vec<Point>) -> Vec<Point> = dynamic_gaussian;
	const SCORING_ALG: fn(&Configuration, &Vec<Point>)->f32 = overlap_fs;
	const SELECT_ALG: fn(&Fn(&Configuration, &Vec<Point>)->f32, &Configuration, &Vec<Vec<Point>>, usize)->Vec<Vec<Point>> = best;

	let mut r = rand::thread_rng();
	let rng = Range::new(0., 1.);

	println!("Initializing states...");
	let mut pool = INIT_ALG(conf, size);
	let mut best_state: Vec<Point> = Vec::new();
	let mut best_score = f32::MIN;

	for it in 0..iters {
		if best_score == f32::MAX {
			println!("Maximum score reached. Stopping early...");
			break;
		}
		let mut new_states: Vec<Vec<Point>> = Vec::new();
		
		for s1 in &pool {
			for s2 in &pool {
				if Point::all_equal(&s1, &s2) {
					continue;
				}

				// Cross step
				if rng.ind_sample(&mut r) < cross_ratio {
					let (_s1, _s2) = homogenize(conf, &s1, &s2);
					let mut ch = blx_alpha(&_s1, &s2, 0.5);
					
					// Mutate step
					if rng.ind_sample(&mut r) < mutate_ratio {
						ch = MUTATE_ALG(&ch, &_s1, &_s2);
						init::vfa(conf, &mut ch);
					}
					// Add to new state
					new_states.push(ch);
				}
			}
		}

		// Normalize
		init::normalize(conf, &mut new_states);

		// Merge with pool and select
		pool.append(&mut new_states);
		pool = SELECT_ALG(&SCORING_ALG, conf, &pool, size as usize);

		let score = SCORING_ALG(conf, &pool[0]);
		if score > best_score {
			best_score = score;
			best_state = pool[0].clone();
		}
		println!("Iter {}: Best score = {:e}", it+1, best_score);
	}

	best_state
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ga() {
		let mut conf = Configuration::new();
		conf.w = 60.; conf.h = 100.;
		conf.n = 3;
		conf.counts = vec![1, 2];
		conf.radius = vec![10., 20.];
		
		let s = genetic_algorithm(&conf, 20, 10, 0.8, 0.05);
		println!("{:?}", s);
		println!("Coverage: {:?}", coverage_area(&conf, &s));
	}
}
