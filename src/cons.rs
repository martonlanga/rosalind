use bio::io::fasta;
use std::path::Path;

pub fn run(filename: &str) {
    let path = Path::new(filename);
    let reader = fasta::Reader::from_file(&path).unwrap();
    let length = reader.records().next().unwrap().expect("Failed");
    let length = length.seq().len();
    let reader = fasta::Reader::from_file(&path).unwrap();
    let mut matrix = vec![vec![0; length]; 4];
    for record in reader.records() {
        let record = record.unwrap();
        for (index, bp) in record.seq().iter().enumerate() {
            match bp {
                b'A' => matrix[0][index] += 1,
                b'C' => matrix[1][index] += 1,
                b'G' => matrix[2][index] += 1,
                b'T' => matrix[3][index] += 1,
                _ => panic!("Whoops"),
            }
        }
    }

    for i in 0..length {
        let mut max = -1;
        let mut max_type: usize = 4;
        for j in 0..4 {
            if matrix[j][i] > max {
                max = matrix[j][i];
                max_type = j;
            }
        }
        print!(
            "{}",
            match max_type {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => panic!("Impossible"),
            }
        );
    }

    println!();

    for (i, cur) in vec!['A', 'C', 'G', 'T'].iter().enumerate() {
        print!("{}:", cur);
        for bp in matrix[i].iter() {
            print!(" {}", bp);
        }
        println!();
    }
}
