use std::collections::HashMap;
use std::fs;

pub fn run(filename: &str) {
    let protein = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let protein = protein.trim();
    let table: HashMap<char, &str> = [
        ('A', "71.03711"),
        ('C', "103.00919"),
        ('D', "115.02694"),
        ('E', "129.04259"),
        ('F', "147.06841"),
        ('G', "57.02146"),
        ('H', "137.05891"),
        ('I', "113.08406"),
        ('K', "128.09496"),
        ('L', "113.08406"),
        ('M', "131.04049"),
        ('N', "114.04293"),
        ('P', "97.05276"),
        ('Q', "128.05858"),
        ('R', "156.10111"),
        ('S', "87.03203"),
        ('T', "101.04768"),
        ('V', "99.06841"),
        ('W', "186.07931"),
        ('Y', "163.06333"),
    ]
    .iter()
    .cloned()
    .collect();
    let mut weight = 0.0;
    for p in protein.chars() {
        match table.get(&p) {
            Some(val) => {
                let val: f64 = val.parse().unwrap();
                weight += val;
            }
            None => println!("Invalid codon: {}", p),
        }
    }
    println!("{}", weight);
}
