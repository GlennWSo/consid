use rand::prelude::*;

#[allow(unused)]
use rand_chacha::ChaCha8Rng;

use consid::{v3::Scanner3 as NewScanner, OldScanner, Scanner};

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(11);
    // let mut rng = thread_rng();
    let temps: Vec<i32> = (0..40).map(|_| rng.gen_range(10..20)).collect();
    let source = temps.clone().into_iter();
    let tolerance = 5;
    let mut new_scanner = NewScanner::new(Box::new(source.clone()), tolerance);
    let time = new_scanner.time_me();
    let (start, end) = new_scanner.best_range();
    if start > 0 {
        println!("pre temp: {}", temps[start - 1]);
    }
    println!("best temps: {:?}", &temps[start..end]);
    if end < (temps.len() - 1) {
        println!("post temp: {}", temps[end]);
    }
    println!("{}", new_scanner);
    println!("old time: {:.3?}", time);
    {
        let mut old_scanner = OldScanner::new(Box::new(source), tolerance);
        let time = old_scanner.time_me();
        println!("old time: {:.3?}", time);
        let old_range = old_scanner.best_range();
        println!("old");
        let (ostart, oend) = old_range;
        if ostart > 0 {
            println!("pre temp: {}", temps[ostart - 1]);
        }
        println!("best temps: {:?}", &temps[ostart..oend]);
        if oend < (temps.len() - 1) {
            println!("post temp: {}", temps[oend]);
        }
        assert_eq!((start, end), old_range, "\nleft: new");
    }
}
