use std::fs;

pub fn complement(filename: &str) {
    let mut dna = fs::read_to_string(filename).expect("Something went wrong reading the file");
    dna = dna.replace("A", "0").replace("T", "A").replace("0", "T");
    dna = dna.replace("C", "0").replace("G", "C").replace("0", "G");
    println!("{}", dna.chars().rev().collect::<String>())
}
