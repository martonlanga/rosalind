use bio::io::fasta;
use std::collections::HashMap;
use std::collections::HashSet;

use std::path::Path;

pub fn run(filename: &str) {
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
    let path = Path::new(filename);
    let reader = fasta::Reader::from_file(&path).unwrap();
    for record in reader.records() {
        let record = record.unwrap();
        let record = String::from_utf8(record.seq().to_vec()).unwrap();
        let record = record.replace("T", "U");
        let mut res = HashSet::new();
        for i in 0..record.len() - 3 {
            match table.get(&record[i..i + 3]) {
                Some(val) if val as &str == "M" => {
                    let mut current = String::from("");
                    for j in (i..record.len() - 3).step_by(3) {
                        match table.get(&record[j..j + 3]) {
                            Some(val) if val as &str == "Stop" => {
                                res.insert(current);
                                break;
                            }
                            Some(val) => current.push_str(val),
                            _ => println!("Invalid."),
                        }
                    }
                }
                _ => (),
            }
        }
        let record = record.replace("A", "0").replace("U", "A").replace("0", "U");
        let record = record.replace("C", "0").replace("G", "C").replace("0", "G");
        let record = record.chars().rev().collect::<String>();

        for i in 0..record.len() - 3 {
            match table.get(&record[i..i + 3]) {
                Some(val) if val as &str == "M" => {
                    let mut current = String::from("");
                    for j in (i..record.len() - 3).step_by(3) {
                        match table.get(&record[j..j + 3]) {
                            Some(val) if val as &str == "Stop" => {
                                res.insert(current);
                                break;
                            }
                            Some(val) => current.push_str(val),
                            _ => println!("Invalid."),
                        }
                    }
                }
                _ => (),
            }
        }

        for r in &res {
            println!("{}", r);
        }
    }
}
