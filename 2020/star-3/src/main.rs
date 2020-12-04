use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
/// Each line gives the password policy and then the password. The password
/// policy indicates the lowest and highest number of times a given letter must
/// appear for the password to be valid. For example, `1-3 a` means that the password
/// must contain a at least 1 time and at most 3 times.
/// `1-3 a: ajaka`
use std::path::Path;
use std::string::String;

struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordRule {
    fn new(rule_str: &str) -> PasswordRule {
        let d1 = rule_str.find('-').unwrap();
        let d2 = rule_str.find(' ').unwrap();

        PasswordRule {
            min: rule_str[..d1].parse::<usize>().unwrap(),
            max: rule_str[d1 + 1..d2].parse::<usize>().unwrap(),
            letter: rule_str[d2 + 1..(d2 + 2)].chars().next().unwrap(),
        }
    }

    fn matches(&self, pw: &str) -> bool {
        let mut count = 0;
        for c in pw.chars() {
            if c == self.letter {
                count += 1;
            }
        }
        count <= self.max && count >= self.min
    }
}

fn validate_data_line(data_line: String) -> bool {
    let delemeter = data_line.find(':').unwrap();
    let rule_str = &data_line[..delemeter];
    let phrase_str = &data_line[delemeter..];

    // This could simply be returned, but I'd like to see the results to validate
    let valid = PasswordRule::new(rule_str).matches(phrase_str);
    if !valid {
        println!("Invalid pw found: {}", data_line);
    }
    valid
}

fn main() {
    let path = Path::new("input.txt");
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", Error::to_string(&e)),
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut num_valid = 0;

    // O(n)
    for l in lines {
        let d = l.unwrap();
        if validate_data_line(d) {
            num_valid += 1;
        }
    }

    println!("Num valid: {}", num_valid);
}
