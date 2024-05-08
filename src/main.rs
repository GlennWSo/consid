use rand::distributions::Uniform;
use rand::prelude::*;

use consid::{v4::Scanner4 as NewScanner, Scanner};
use rand_chacha::ChaCha8Rng;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "finds largest sequence of numbers within a tolerance from a random source",
    long_about
)]
struct Args {
    #[arg(short, long)]
    seed: Option<u64>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "prints debug information"
    )]
    print_rng: bool,

    #[arg(long, help = "low bound for rng numbers")]
    lb: i32,

    #[arg(short, long, help = "rng span(high bound - low bound)")]
    width: u32,

    #[arg(
        short,
        long,
        default_value_t = 1000,
        help = "number values to generate/scan for"
    )]
    count: usize,

    #[arg(
        short,
        default_value_t = 5,
        help = "tolerance of minmax differance in window scan"
    )]
    tol: i32,
}

impl Args {
    fn hb(&self) -> i32 {
        self.lb + self.width as i32
    }
    fn run(&self) {
        // data spec
        let seed = match self.seed {
            Some(seed) => seed,
            None => {
                let seed = thread_rng().gen();
                println!("using random seed: {seed}");
                seed
            }
        };
        let rng = ChaCha8Rng::seed_from_u64(seed);
        if self.print_rng {
            let rng = rng.clone();
            let distro = Uniform::new_inclusive(self.lb, self.hb());
            let rng_gen = distro.sample_iter(rng).take(self.count);
            println!("random values:");
            for (i, v) in rng_gen.enumerate() {
                println!("{i}: {v}");
            }
        }
        let distro = Uniform::new_inclusive(self.lb, self.hb());
        let rng_gen = distro.sample_iter(rng).take(self.count);

        // solve: find largest window with diff tolerance
        let mut scanner = NewScanner::new(Box::new(rng_gen), self.tol);
        let time = scanner.time_me();

        // // report
        let (start, end) = scanner.best_range();
        println!("best window found {}..={}", start, end - 1);
        println! {"time taken: {}ms", time.as_millis()};
    }
}

fn main() {
    let args = Args::parse();
    args.run();
}
