use bio::pattern_matching::shift_and;
use std::fs;

pub fn run(filename: &str) {
    let file = fs::read(filename).expect("Something went wrong reading the file");
    let mut lines = file.split(|&b| b == b'\n');
    let dna = lines.next().expect("Missing dna");
    let pattern = lines.next().expect("Missing pattern");
    let shiftand = shift_and::ShiftAnd::new(pattern);
    for occ in shiftand.find_all(dna) {
        print!("{} ", occ + 1);
    }
}
