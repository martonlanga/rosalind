use bio::io::fasta;
use std::str;

use std::path::Path;

pub fn run(filename: &str) {
    let path = Path::new(filename);
    let reader = fasta::Reader::from_file(&path).unwrap();
    let mut arr = vec![];
    for record in reader.records() {
        let record = record.unwrap();
        arr.push(record.seq().to_owned());
    }

    let n = arr.len();
    let s = &arr[0];
    let len = s.len();
    let mut res = vec![];

    for i in 0..len {
        for j in i + 1..len {
            let stem = &s[i..j];
            let mut k = 1;
            while k < n {
                let long_string = String::from_utf8(arr[k].to_vec()).unwrap();
                let test_string = str::from_utf8(stem).unwrap();
                if !long_string.contains(test_string) {
                    break;
                }
                k += 1;
            }

            if k == n && res.len() < stem.len() {
                res = stem.to_vec();
            }
        }
    }

    let res = str::from_utf8(&res).unwrap();
    println!("{}", res);
}
