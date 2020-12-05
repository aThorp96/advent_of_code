use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str;

#[derive(Debug)]
struct Slope {
    rise: usize,
    run: usize,
    width: usize,
    map: Vec<char>,
}

impl Slope {
    fn new(map: Vec<u8>, rise: usize, run: usize) -> Slope {
        let first_newline = map.iter().position(|&b| b as char == '\n').unwrap() as usize;
        Slope {
            rise: rise,
            run: run,
            width: first_newline,
            // Filter out all the newlines so indexing works correctly
            map: map
                .iter()
                .filter_map(|b| {
                    if *b as char != '\n' {
                        Some(*b as char)
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    /// print the map, highlighting points along the slope
    fn print(&self) {
        let mut n = 0;
        for i in 0..self.map.len() {
            let on_slope = i == self.iteration_to_index(n);
            if on_slope {
                print!("[{}]", self.map[i]);
                n += 1;
            } else {
                print!(" {} ", self.map[i]);
            }
            if i % self.width == self.width - 1 {
                println!();
            }
        }
    }

    fn iteration_to_index(&self, n: usize) -> usize {
        (n * self.rise * self.width) + ((n * self.run) % self.width)
    }

    fn num_collisions(&self) -> usize {
        let mut collisions: usize = 0;

        for n in 1..self.map.len() {
            match self.map.get(self.iteration_to_index(n)) {
                Some(&c) => {
                    if c == '#' {
                        collisions += 1;
                    }
                }
                None => {}
            }
        }
        collisions
    }
}

fn get_file_reader(file_name: &str) -> BufReader<File> {
    let path = Path::new(file_name);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", Error::to_string(&e)),
    };

    BufReader::with_capacity(file.metadata().unwrap().len() as usize, file)
}

fn day_3(input_file: &str, part: usize) {
    let mut reader = get_file_reader(&input_file);
    let map = Vec::from(reader.fill_buf().unwrap());

    match part {
        1 => {
            let slope = Slope::new(map, 1, 3);
            println!("Trees collided: {}", slope.num_collisions());
        }
        2 => {
            let mut total = 1;
            let grades: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

            for (run, rise) in grades.iter() {
                let slope = Slope::new(map.clone(), *rise, *run);
                println!(
                    "Right {}, down {}\nTrees collided: {}",
                    run,
                    rise,
                    slope.num_collisions()
                );
                total *= slope.num_collisions();
            }
            println!("Total for collisions: {}", total);
        }
        _ => return,
    };
}

fn main() {
    day_3("input_full.txt", 2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iteration() {
        let map = String::from("..##.......#...#...#..\n.#....#..#...#.#...#.#").into_bytes();
        let slope = Slope::new(map.clone(), 1, 8);
        slope.print();
        println!("{:?}", slope);
        assert_eq!(slope.iteration_to_index(1), 30);
    }

    #[test]
    fn test_slope_1_3_collisions() {
        let map = String::from("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#").into_bytes();
        let slope = Slope::new(map, 1, 3);
        assert_eq!(slope.num_collisions(), 7);
    }
}
