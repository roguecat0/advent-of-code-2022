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
        .map(transform_tupling)
        .filter(double_contains)
        .count() as u64
}
pub fn run2(text: &str) -> u64 {
    text.lines()
        .map(transform_tupling)
        .filter(or_contains)
        .count() as u64
}
fn double_contains((r1, r2): &((u64, u64), (u64, u64))) -> bool {
    ((r1.0..=r1.1).contains(&r2.0) && (r1.0..=r1.1).contains(&r2.1))
        || ((r2.0..=r2.1).contains(&r1.0) && (r2.0..=r2.1).contains(&r1.1))
}
fn or_contains((r1, r2): &((u64, u64), (u64, u64))) -> bool {
    ((r1.0..=r1.1).contains(&r2.0) || (r1.0..=r1.1).contains(&r2.1))
        || ((r2.0..=r2.1).contains(&r1.0) || (r2.0..=r2.1).contains(&r1.1))
}
fn transform_tupling(line: &str) -> ((u64, u64), (u64, u64)) {
    let vals = line
        .split(&['-', ','])
        .flat_map(|num_str| num_str.parse::<u64>())
        .collect::<Vec<_>>();
    ((vals[0], vals[1]), (vals[2], vals[3]))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_contains() {
        let val = ((2, 3), (1, 6));
        assert!(double_contains(&val));
        let val = ((2, 3), (1, 2));
        assert!(!double_contains(&val));
    }
    #[test]
    fn test_transform() {
        let val = "2-4,6-8";
        assert_eq!(((2, 4), (6, 8)), transform_tupling(val));
    }

    #[test]
    fn practice_run() {
        let s: &'static str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(2, run(s));
    }
    #[test]
    fn practice_run2() {
        let s: &'static str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(4, run2(s));
    }
}
