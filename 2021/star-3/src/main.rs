use std::convert::TryFrom;
use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl TryFrom<String> for Direction {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lower = value.to_ascii_lowercase();

        // Parse forward
        if let Some(forward) = lower.strip_prefix("forward ") {
            let magnitude = forward
                .parse::<i32>()
                .expect("Could not parse number for forward");
            return Ok(Direction::Forward(magnitude));
        // Parse Down
        } else if let Some(down) = lower.strip_prefix("down ") {
            let magnitude = down
                .parse::<i32>()
                .expect("Could not parse number for down");
            return Ok(Direction::Down(magnitude));
        // Parse Up
        } else if let Some(up) = lower.strip_prefix("up ") {
            let magnitude = up.parse::<i32>().expect("Could not parse number for up");
            return Ok(Direction::Up(magnitude));
        }
        Err(String::from("no matching direction"))
    }
}

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let stdin = io::stdin();

    for (_, line) in stdin.lock().lines().enumerate() {
        let input = line.unwrap();

        match Direction::try_from(input) {
            Ok(Direction::Forward(n)) => x = x + n,
            Ok(Direction::Up(n)) => y = y - n,
            Ok(Direction::Down(n)) => y = y + n,
            Err(e) => panic!("{}", e),
        }
    }

    println!("Final coordinates: x: {}, y {}", x, y);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direction_parse() {
        assert_eq!(
            Direction::try_from(String::from("Forward 8")),
            Ok(Direction::Forward(8))
        );
        assert_eq!(
            Direction::try_from(String::from("Down 48")),
            Ok(Direction::Down(48))
        );
        assert_eq!(
            Direction::try_from(String::from("Up 2")),
            Ok(Direction::Up(2))
        );
    }
}
