use std::thread::sleep;
use std::time::Duration;

use rand::distributions::{DistIter, Uniform};
use rand::{distributions::Standard, prelude::*};

use rand_chacha::ChaCha8Rng;

use consid::{v4::Scanner4 as NewScanner, OldScanner, Scanner};

fn main() {
    // data spec
    let seed: u64 = 11;
    let low = -20;
    let high = 30;
    let count = 10_usize.pow(6);
    let rng = ChaCha8Rng::seed_from_u64(seed);

    let distro = Uniform::new_inclusive(low, high);
    let rng_gen = distro.sample_iter(rng).take(count);

    // task
    let tolerance = 5;

    // solve
    let mut scanner = NewScanner::new(Box::new(rng_gen), tolerance);
    let time = scanner.time_me();

    // // report
    let (start, end) = scanner.best_range();
    println!("best window found {start}..={end}");
    println! {"time taken: {}ms", time.as_millis()};
}
