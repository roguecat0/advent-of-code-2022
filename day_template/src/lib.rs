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


pub fn run(text: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_run() {
        let s: &'static str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46, run(s));
    }
}
