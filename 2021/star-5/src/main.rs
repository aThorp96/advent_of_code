use std::io;
use std::io::prelude::*;

/**
Each bit in the gamma rate can be determined by finding the most common bit in
the corresponding position of all numbers in the diagnostic report. For
example, given the following diagnostic report:

00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010

Considering only the first bit of each number, there are five 0 bits and seven
1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.
The most common second bit of the numbers in the diagnostic report is 0, so the
second bit of the gamma rate is 0.
The most common value of the third, fourth, and fifth bits are 1, 1, and 0,
respectively, and so the final three bits of the gamma rate are 110.
So, the gamma rate is the binary number 10110, or 22 in decimal.
The epsilon rate is calculated in a similar way; rather than use the most
common bit, the least common bit from each position is used. So, the epsilon
rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon
rate (9) produces the power consumption, 198.
**/

fn main() {
    let mut num_lines = 0;
    let mut sums = Vec::new();

    let stdin = io::stdin();

	// Iterate over each line
    // Sum the number of ones in each bit of each line.
    // If the sum of each bit's ones is more than half the number of lines,
    // then the gamma bit should be 1 and the epsilon bit should be zero
    for (i, line) in stdin.lock().lines().enumerate() {
        let input = line.unwrap();
        num_lines = num_lines + 1;
        for (j, c) in input.chars().enumerate() {
            // On the first run, grow the vector to size
            if i < 1 {
				sums.push(0)
            }
            if c == '1' {
                sums[j] = sums[j] + 1
            };
        }
    }
    // Set the gamma bits accordingly: if the sum of the bit position is greater than n/2, then 1 is the average bit.
    let gamma_bits: Vec<i32> = sums
        .iter()
        .map(|&n| if n > (num_lines / 2) { 1 } else { 0 })
        .collect();

	// Invert the gamma bits to get the epsilon bits
    let epsilon_bits: Vec<i32> = gamma_bits
        .iter()
        .map(|&c| match c {
            1 => 0,
            _ => 1,
        })
        .collect();


    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

	// Build the numbers: add the current bit and shift over
    gamma_bits.iter().for_each(|n| {
        gamma = gamma << 1;
        gamma |= *n as u32;
    });

    epsilon_bits.iter().for_each(|n| {
        epsilon = epsilon << 1;
        epsilon |= *n as u32;
    });

	println!("Gamma binary: {:b}: {}", gamma, gamma);
	println!("Espilon binary: {:b}: {}", epsilon, epsilon);
	println!("Final number: {}", epsilon * gamma);
}
