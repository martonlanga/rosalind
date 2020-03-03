use bio::io::fasta;
use std::path::Path;

fn is_revp(v: &[u8]) -> bool {
    let mut reversed = v.to_vec();
    let mut revc = v.to_vec();
    reversed.reverse();
    for (i, r) in reversed.iter().enumerate() {
        match r {
            b'A' => revc[i] = b'T',
            b'T' => revc[i] = b'A',
            b'C' => revc[i] = b'G',
            b'G' => revc[i] = b'C',
            _ => panic!("F"),
        }
    }
    revc == v
}

pub fn run(filename: &str) {
    let path = Path::new(filename);
    let reader = fasta::Reader::from_file(&path).unwrap();
    for record in reader.records() {
        let record = record.unwrap();
        let record = record.seq();

        for (i, _) in record.iter().enumerate() {
            let max = if record.len() - i < 12 {
                record.len() - i
            } else {
                12
            };

            if max > 3 {
                for j in 4..=max {
                    if is_revp(&record[i..i + j]) {
                        println!("{} {}", i + 1, j);
                    }
                }
            }
        }
    }
}
