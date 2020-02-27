use std::fs;

pub fn count(filename: &str) {
    let dna = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut a = 0;
    let mut c = 0;
    let mut g = 0;
    let mut t = 0;
    for n in dna.as_bytes() {
        match n {
            65 => a += 1,
            67 => c += 1,
            71 => g += 1,
            84 => t += 1,
            _ => println!("Invalid nucleotide"),
        }
    }
    println!("A C G T");
    println!("{} {} {} {}", a, c, g, t)
}
