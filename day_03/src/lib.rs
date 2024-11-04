mod my_io {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error as IOError;

    pub fn read_file_to_string(file_path: &str) -> Result<String, IOError> {
        let mut s: String = String::new();
        let mut file: File = File::open(file_path)?;
        file.read_to_string(&mut s)?;
        Ok(s)
    }
}

pub use my_io::read_file_to_string;

pub fn run(text: &str) -> u64 {
    text.lines()
        .map(|line| (&line[0..line.len() / 2], &line[line.len() / 2..line.len()]))
        .inspect(|(comp1, comp2)| println!("{comp1}, {comp2}"))
        .map(get_duplicate)
        .map(priority)
        .sum()
}

fn get_duplicate((comp1, comp2): (&str, &str)) -> char {
    comp1
        .chars()
        .flat_map(|c| comp2.find(c.clone()).and_then(|_i| Some(c)))
        .last()
        .expect("always has a duplicate")
}

fn priority(c: char) -> u64 {
    println!("{c}");
    if c.is_lowercase() {
        c as u64 - 96
    } else {
        c as u64 - 38
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_run() {
        let s: &'static str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(157, run(s));
    }
    #[test]
    fn test_duplicated() {
        let t = get_duplicate(("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
        assert_eq!('p', t);
    }
    #[test]
    fn test_priority() {
        assert_eq!(16, priority('p'));
        assert_eq!(38, priority('L'));
        assert_eq!(42, priority('P'));
        assert_eq!(22, priority('v'));
        assert_eq!(20, priority('t'));
    }
}
