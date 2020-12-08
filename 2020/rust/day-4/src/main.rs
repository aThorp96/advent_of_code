use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str;

use clap::Clap;

trait Field {
    fn new(value: String) -> Self;
    fn key() -> String;
}

macro_rules! field {
    ($field_name:ident) => {
        #[derive(Debug)]
        struct $field_name {
            value: String,
        }
        impl Field for $field_name {
            fn new(value: String) -> Self {
                Self { value }
            }
            fn key() -> String {
                String::from(stringify!($field_name)).to_ascii_lowercase()
            }
        }
    };
}

field!(Byr);
field!(Iyr);
field!(Eyr);
field!(Hgt);
field!(Hcl);
field!(Pid);
field!(Cid);

fn parse_field<T: Field>(fields: &Vec<String>) -> Result<T> {
    let keyword = T::key();
    let field_string = fields
        .iter()
        .filter(|f| f.starts_with(&keyword))
        .next()
        .ok_or(anyhow!("Field {} not found", keyword))?;

    let delimiter = match field_string.find(':') {
        Some(d) => d,
        None => return Err(anyhow!("Malformed field: '{}'", field_string)),
    };

    Ok(T::new(String::from(&field_string[delimiter + 1..])))
}

#[derive(Debug)]
struct Id {
    byr: Byr,
    iyr: Iyr,
    eyr: Eyr,
    hgt: Hgt,
    hcl: Hcl,
    pid: Pid,
    cid: Option<Cid>,
}

impl Id {
    fn new(
        byr: Byr,
        iyr: Iyr,
        eyr: Eyr,
        hgt: Hgt,
        hcl: Hcl,
        pid: Pid,
        cid: Option<Cid>,
    ) -> Result<Id> {
        Ok(Id {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            pid,
            cid,
        })
    }

    fn parse(id_str: String) -> Result<Id> {
        let mut fields = Id::get_fields(id_str)?;
        Id::new(
            parse_field(&fields)?,
            parse_field(&fields)?,
            parse_field(&fields)?,
            parse_field(&fields)?,
            parse_field(&fields)?,
            parse_field(&fields)?,
            parse_field(&fields).ok(),
        )
    }

    fn get_fields(data: String) -> Result<Vec<String>> {
        let fields: Vec<String> = data
            .split(|c| c == ' ' || c == '\n')
            .map(|s| String::from(s))
            .collect();
        Ok(fields)
    }
}

fn day_4(mut input: BufReader<File>, part: i32) {
    let ids = str::from_utf8(input.fill_buf().unwrap())
        .unwrap()
        .split("\n\n");
    let mut num_seen = 0;
    let mut num_valid = 0;
    match part {
        1 => {
            for id in ids {
                println!("\nData {}: \n{}", num_seen, id);
                num_seen += 1;
                match Id::parse(String::from(id)) {
                    Ok(id) => {
                        //println!("Id: {:?}", id);
                        num_valid += 1
                    }
                    Err(e) => println!("Invalid passport: {}", e),
                }
            }
        }
        _ => {}
    }
    println!("Number of valid IDs seen: {}/{}", num_valid, num_seen);
}

fn get_file_reader(file_name: &str) -> BufReader<File> {
    let path = Path::new(file_name);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", anyhow!(e)),
    };

    BufReader::with_capacity(file.metadata().unwrap().len() as usize, file)
}

#[derive(Clap)]
struct Opt {
    #[clap(short, long, default_value = "input.txt")]
    input_file: String,
    #[clap(short, long, default_value = "1")]
    part: i32,
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let opt = Opt::parse();
    let reader = get_file_reader(&opt.input_file);
    day_4(reader, opt.part);
}
