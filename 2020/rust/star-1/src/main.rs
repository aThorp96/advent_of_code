use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;

/// Return the corresponding number for any number n.
fn counterpart(i: usize) -> usize {
    2020 - i
}

fn main() {
	// Map of numbers to whether or not that number has been found
    // 		note: This method only works with non-negative lists
	let mut nums = vec![false; 2020];

    let path = Path::new("input.txt");
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", Error::to_string(&e)),
    };

    let reader = BufReader::new(file);
	let lines = reader.lines();

	// Iterate over the numbers
	for l in lines {
    	let i = l.unwrap().parse::<usize>().unwrap();
    	let i_prime = counterpart(i);
    	// set the current number to "found"
    	nums[i] = true;
    	// If the current number's counterpart has been found, print the result
        // and short circuit
    	if nums[i_prime] == true {
        	println!("{} + {} = {}", i, i_prime, i + i_prime);
        	println!("{} * {} = {}", i, i_prime, i * i_prime);
        	return
    	}
	};
}
