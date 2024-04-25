use rand::prelude::*;

#[allow(unused)]
use rand_chacha::ChaCha8Rng;

use consid::{v2::Scanner2, OldScanner, Scanner};

fn main() {
    // let mut rng = ChaCha8Rng::seed_from_u64(1);
    let mut rng = thread_rng();
    let temps: Vec<i32> = (0..10000000).map(|_| rng.gen_range(-20..20)).collect();
    // println!("temps: {:#?}", temps);
    let source = temps.clone().into_iter();
    let tolerance = 30;
    let mut scanner = OldScanner::new(Box::new(source.clone()), tolerance);
    let time = scanner.time_me();
    let (start, end) = scanner.best_range();
    if start > 0 {
        println!("pre temp: {}", temps[start - 1]);
    }
    println!("{}", scanner);
    if end < (temps.len() - 1) {
        println!("post temp: {}", temps[end + 1]);
    }
    println!("old time: {:.3?}", time);
    {
        let mut new_scanner = Scanner2::new(Box::new(source), tolerance);
        let time = new_scanner.time_me();
        println!("new time: {:.3?}", time);
        assert_eq!((start, end), new_scanner.best_range());
    }
}
