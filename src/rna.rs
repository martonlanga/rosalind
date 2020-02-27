use std::fs;

pub fn transcribe(filename: &str) {
    let mut dna = fs::read_to_string(filename).expect("Something went wrong reading the file");
    dna = dna.replace("T", "U");
    println!("{}", dna)
}
