use std::io;
use std::io::prelude::*;

fn main() {
    // Boxes represent the three scan indexes
    let mut i: i32 = -2;
    let mut j: i32 = -1;

    let mut sums: Vec<i32> = Vec::new();

    let mut increase_counter = 0;

    let stdin = io::stdin();

    for (_, line) in stdin.lock().lines().enumerate() {
        let input = line.unwrap();
        let n = input.parse::<i32>().unwrap();

        sums.push(n);

        if i >= 0 {
            sums[i as usize] = sums[i as usize] + n;
        }
        i = i + 1;
        if j >= 0 {
            sums[j as usize] = sums[j as usize] + n;
        }
        j = i + 1;
    }

    let mut last: Option<i32> = None;
    for n in sums.into_iter() {
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
