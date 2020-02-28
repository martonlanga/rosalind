use std::fs;

pub fn run(filename: &str) {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let n: u64 = input
        .split(' ')
        .nth(0)
        .unwrap()
        .to_string()
        .trim()
        .parse::<u64>()
        .unwrap();
    let k: u64 = input
        .split(' ')
        .nth(1)
        .unwrap()
        .to_string()
        .trim()
        .parse::<u64>()
        .unwrap();
    let mut previous1: u64 = 1;
    let mut previous2: u64 = 1;
    for _ in 2..n {
        let temp = previous1;
        previous1 = previous2;
        previous2 = temp * k + previous2;
    }
    println!("{}", previous2);
}
