use std::fs;

pub fn run(filename: &str) {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let k: f32 = parse_number(&input, 0);
    let m: f32 = parse_number(&input, 1);
    let n: f32 = parse_number(&input, 2);
    let x = k + m + n;
    let p_dominant = 1.0
        - ((n / x) * ((n - 1.0) / (x - 1.0) + (m / (x - 1.0) / 2.0))
            + (m / x / 2.0) * (n / (x - 1.0) + (m - 1.0) / (x - 1.0) / 2.0));
    println!("{}", p_dominant);
}

fn parse_number(input: &str, index: usize) -> f32 {
    input
        .split(' ')
        .nth(index)
        .unwrap()
        .to_string()
        .trim()
        .parse::<f32>()
        .unwrap()
}
