use std::io;
use std::io::prelude::*;

fn main() {
    let mut last: Option<i32> = None;

    let mut increase_counter = 0;

    let stdin = io::stdin();

    for (_, line) in stdin.lock().lines().enumerate() {
        let input = line.unwrap();
		let n = input.parse::<i32>().unwrap();


		if let Some(l) = last {
    		if l < n {
        		println!("{} < {} - increased", l, n);
        		increase_counter += 1;
    		}
		}
		last = Some(n)
    }

    println!("Increased count: {}", increase_counter);
}
