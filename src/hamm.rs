use std::fs;

pub fn run(filename: &str) {
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let first = file.split("\n").nth(0).unwrap();
    let second = file.split("\n").nth(1).unwrap();

    let mut count = 0;
    for i in 0..first.len() {
        if first.chars().nth(i) != second.chars().nth(i) {
            count += 1;
        }
    }

    println!("{}", count);
}
