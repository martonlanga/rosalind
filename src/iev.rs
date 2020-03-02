use std::fs;

struct Input {
    raw: String,
}

impl Input {
    fn num(&self, index: usize) -> f32 {
        self.raw
            .split(' ')
            .nth(index - 1)
            .unwrap()
            .to_string()
            .trim()
            .parse::<f32>()
            .unwrap()
            * 2.0
    }
}

pub fn run(filename: &str) {
    let input = Input {
        raw: fs::read_to_string(filename).expect("Something went wrong reading the file"),
    };

    println!(
        "{}",
        input.num(1) + input.num(2) + input.num(3) + input.num(4) * 3.0 / 4.0 + input.num(5) / 2.0
    );
}
