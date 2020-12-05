use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;

use std::collections::BTreeSet;

fn main() {
	let mut nums: BTreeSet<usize> = BTreeSet::new();
	let mut added: BTreeSet<usize> = BTreeSet::new();
	let mut o = 0;

    let path = Path::new("input.txt");
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", Error::to_string(&e)),
    };

    let reader = BufReader::new(file);
	let lines = reader.lines();

	// O(n)
	for l in lines {
    	let i = l.unwrap().parse::<usize>().unwrap();
    	nums.insert(i);
	};

	// O(n*n*n)
	for i in nums.range(0..2021) {
    	for j in nums.range(0..(2021-i)) {
        	let k = 2020 - i - j;
        	if nums.contains(&k) {
            	println!("{} + {} + {} = {}", i, j, k, i + j + k);
            	println!("{} * {} * {} = {}", i, j, k, i * j * k);
            	return
        	}
    	}
	};
}
