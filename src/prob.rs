use std::fs;

pub fn run(filename: &str) {
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = file.split('\n');
    let dna = lines.next().expect("Missing dna");
    let numbers = lines.next().expect("Missing pattern");
    let numbers: Vec<f64> = numbers
        .to_string()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for number in numbers.iter() {
        let mut chance = 1.0;
        let gc_prob = number / 2.0;
        let at_prob = (1.0 - number) / 2.0;
        for c in dna.chars() {
            match c {
                'G' | 'C' => chance *= gc_prob,
                'A' | 'T' => chance *= at_prob,
                _ => println!("Invalid"),
            }
        }

        print!("{} ", chance.log10())
    }
    println!();
}
