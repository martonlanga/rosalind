use std::collections::HashMap;
use std::fs;

pub fn run(filename: &str) {
    let rna = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let table: HashMap<&str, &str> = [
        ("UUU", "F"),
        ("CUU", "L"),
        ("AUU", "I"),
        ("GUU", "V"),
        ("UUC", "F"),
        ("CUC", "L"),
        ("AUC", "I"),
        ("GUC", "V"),
        ("UUA", "L"),
        ("CUA", "L"),
        ("AUA", "I"),
        ("GUA", "V"),
        ("UUG", "L"),
        ("CUG", "L"),
        ("AUG", "M"),
        ("GUG", "V"),
        ("UCU", "S"),
        ("CCU", "P"),
        ("ACU", "T"),
        ("GCU", "A"),
        ("UCC", "S"),
        ("CCC", "P"),
        ("ACC", "T"),
        ("GCC", "A"),
        ("UCA", "S"),
        ("CCA", "P"),
        ("ACA", "T"),
        ("GCA", "A"),
        ("UCG", "S"),
        ("CCG", "P"),
        ("ACG", "T"),
        ("GCG", "A"),
        ("UAU", "Y"),
        ("CAU", "H"),
        ("AAU", "N"),
        ("GAU", "D"),
        ("UAC", "Y"),
        ("CAC", "H"),
        ("AAC", "N"),
        ("GAC", "D"),
        ("UAA", "Stop"),
        ("CAA", "Q"),
        ("AAA", "K"),
        ("GAA", "E"),
        ("UAG", "Stop"),
        ("CAG", "Q"),
        ("AAG", "K"),
        ("GAG", "E"),
        ("UGU", "C"),
        ("CGU", "R"),
        ("AGU", "S"),
        ("GGU", "G"),
        ("UGC", "C"),
        ("CGC", "R"),
        ("AGC", "S"),
        ("GGC", "G"),
        ("UGA", "Stop"),
        ("CGA", "R"),
        ("AGA", "R"),
        ("GGA", "G"),
        ("UGG", "W"),
        ("CGG", "R"),
        ("AGG", "R"),
        ("GGG", "G"),
    ]
    .iter()
    .cloned()
    .collect();
    let mut protein = String::new();
    for i in (0..rna.len() - 3).step_by(3) {
        match table.get(&rna[i..i + 3]) {
            Some(val) if val as &str == "Stop" => println!("End."),
            Some(val) => protein.push_str(val),
            None => println!("Invalid codon."),
        }
    }
    println!("{}", protein)
}
