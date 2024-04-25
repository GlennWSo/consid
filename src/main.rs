use std::{
    collections::VecDeque,
    fmt::{write, Display},
};

use rand::Rng;

struct Scanner {
    source: Box<dyn Iterator<Item = i32>>,
    window: VecDeque<i32>,
    longest_streak: usize,
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo!()
        write!(f, "display not yet implmented for Scanner")
    }
}

impl Scanner {
    fn new(source: Box<dyn Iterator<Item = i32>>) -> Self {
        Self { source }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let temps: Vec<i32> = (0..10).map(|_| rng.gen_range(-25..50)).collect();
    println!("temps: {:#?}", temps);
    let source = temps.into_iter();
    let scanner = Scanner::new(Box::new(source));
    println!("{}", scanner);
}
