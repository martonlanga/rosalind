use bio::io::fasta;
use bio::seq_analysis::gc;
use std::path::Path;

pub fn run(filename: &str) {
    let path = Path::new(filename);
    let reader = fasta::Reader::from_file(&path).unwrap();
    let mut max = (-1.0, String::new());
    for record in reader.records() {
        let record = record.unwrap();
        let gc_content = gc::gc_content(record.seq());
        if gc_content > max.0 {
            max.0 = gc_content;
            max.1 = record.id().to_string();
        }
    }
    println!("{}", max.1);
    println!("{}", max.0 * 100.0);
}
