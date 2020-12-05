/// Each line gives the password policy and then the password. The password
/// policy indicates the two indices the  given letter must appear exacly once in
/// for the password to be valid (1 indexed). For example, `1-3 a` means that the password
/// must contain 'a' at index 1 exclusive or at index 3.
/// `2-3 a: ajaka` // valid
/// `1-3 a: ajaka` // invalid
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::path::Path;
use std::string::String;

struct PasswordRule {
    p1: usize,
    p2: usize,
    letter: char,
}

impl PasswordRule {
    fn new(rule_str: &str) -> PasswordRule {
        // Get delemeters
        let d1 = rule_str.find('-').unwrap();
        let d2 = rule_str.find(' ').unwrap();

        PasswordRule {
            p1: rule_str[..d1].parse::<usize>().unwrap(),
            p2: rule_str[d1 + 1..d2].parse::<usize>().unwrap(),
            letter: rule_str.as_bytes()[d2 + 1] as char,
        }
    }

    fn matches(&self, pw: &str) -> bool {
        let c1 = pw.as_bytes()[self.p1 - 1] as char;
        let c2 = pw.as_bytes()[self.p2 - 1] as char;
        !(c1 == self.letter && c2 == self.letter) && (c1 == self.letter || c2 == self.letter)
    }
}

fn validate_data_line(data_line: String) -> bool {
    let delemeter = data_line.find(':').unwrap();
    let rule_str = &data_line[..delemeter];
    let phrase_str = &data_line[delemeter + 2..];

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

    for l in lines {
        let d = l.unwrap();
        if validate_data_line(d) {
            num_valid += 1;
        }
    }

    println!("Num valid: {}", num_valid);
}
