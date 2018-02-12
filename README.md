# Optimizing sensor network coverage with genetic algorithm
This project is an assignment for HUST IT4141 - Evolutionary Computing.
It utilizes GA to find near-optimal placements for nodes in a sensor network. The entire implementation is written in Rust, along with plotting in Python and Matplotlib.

## Requirements
- Rust & Cargo (follow instructions [here](https://www.rust-lang.org/en-US/install.html))
- Python 2.7/3.4

## Quickstart
```bash
git clone https://github.com/lanPN85/area-cov
cd area-cov

# Install Python dependencies
sudo pip install -r src/plot/requirements.txt

# Run tests to validate
cargo test

# Run and get results on EGA data. May take a while.
# Results will be saved in 'out/'
./run_all.sh
```

## Details
The GA pipeline is implemented as follow:
- The input values are contained in a struct called Configuration.
- Point is the core struct which represents a sensor's position. Point allows element-wise ops like addition, multiplications,... For details, see `src/models/point.rs`
- Encoding: Each individual (state) is a Vec\<Point>.
- Initialization: Includes random initialization with VFA adjustment and heuristic initialization. Only uses heuristic. See `src/ga/init.rs`
- Crossover: Implements BLX-&#945; crossover, with a homogenize step to alleviate encoding redundancy. See `src/ga/cross.rs`
- Mutation: Implements static and dynamic Gaussian mutation. Only uses dynamic. See `src/ga/mutation.rs`
- Selection: Selects k-best. Allows passing arbitrary metric function. See `src/ga/select.rs`

Test data is generated according to Yoon et al, _An Efficient Genetic Algorithm for Maximum
Coverage Deployment in Wireless Sensor Networks_.
 
## About
Author: Phan Ngoc Lan (<phan.ngoclan58@gmail.com>)
