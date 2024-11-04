mod my_io {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error as IOError;

    pub fn read_file_to_string(file_path: &str) -> Result<String,IOError> {
        let mut s: String = String::new();
        let mut file: File = File::open(file_path)?;
        file.read_to_string(&mut s)?;
        Ok(s)
    }
}

pub use my_io::read_file_to_string;

enum Hand {
    Rock,
    Paper,
    Scissors
}

use Hand::{Rock,Paper,Scissors};

impl Hand {
    fn build(inp: &str) -> Self {
        match inp {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("not a valid input letter")
        }
    }
    fn worth(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        } 
    }
    fn round_score(&self, other: &Hand) ->  {
        match self {
            Rock => {
                match other {
                    Paper => 0,
                    Rock => 3,
                    Scissors => 6
                }
            }
        }
    }
}


pub fn run(text: &str) -> u64 {
    0
}
fn calc_round_score() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_run() {
        let s: &'static str = "\
A Y
B X
C Z";
        assert_eq!(15, run(s));
    }
}
