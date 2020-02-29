use std::fs;

pub fn run(filename: &str) {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let n: u32 = input
        .split(' ')
        .nth(0)
        .unwrap()
        .to_string()
        .trim()
        .parse::<u32>()
        .unwrap();
    let k: u32 = input
        .split(' ')
        .nth(1)
        .unwrap()
        .to_string()
        .trim()
        .parse::<u32>()
        .unwrap();

    let mut ages = vec![0; k as usize];
    ages[0] = 1;
    for _ in 0..n - 1 {
        let sum: u128 = ages[1..ages.len()].iter().sum();
        let clone = ages.clone();
        for i in 1..ages.len() {
            ages[i] = clone[i - 1];
        }
        ages[0] = sum;
    }

    let sum: u128 = ages.iter().sum();
    println!("{}", sum);
}
