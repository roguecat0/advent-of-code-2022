use day_06 as lib;

fn main() {
    let s = lib::read_file_to_string("input.txt").unwrap();
    println!("result: {}", lib::run(&s));
}
