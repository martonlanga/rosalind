use bio::io::fasta;
use std::path::Path;

pub fn run(filename: &str) {
    let path = Path::new(filename);
    let reader = fasta::Reader::from_file(&path).unwrap();
    let mut records = vec![];
    for record in reader.records() {
        let record = record.unwrap();
        records.push(record);
    }

    for i in records.iter() {
        for j in records.iter() {
            if i.id() != j.id() && i.seq()[i.seq().len() - 3..] == j.seq()[..3] {
                println!("{} {}", i.id(), j.id());
            }
        }
    }
}
