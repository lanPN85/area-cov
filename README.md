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

## About
Author: Phan Ngoc Lan (<phan.ngoclan58@gmail.com>)
