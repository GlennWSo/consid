use std::{collections::VecDeque, fmt::Display};

use rand::Rng;

struct Scanner {
    source: Box<dyn Iterator<Item = i32>>,
    window: VecDeque<i32>,
    sort_inds: VecDeque<usize>,
    longest_streak: usize,
    best_final_day: usize,
    max: i32,
    min: i32,
    tolerance: i32,
    current_day: usize,
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo!()
        // write!(f, "display not yet implmented for Scanner")
        writeln!(f, "window: {:?}", self.window)?;
        writeln!(f, "sort_inds: {:?}", self.sort_inds)?;
        writeln!(f, "longest_streak: {:?}", self.longest_streak)?;
        write!(f, "best_final_day: {:?}", self.best_final_day)
    }
}

impl Scanner {
    fn new(mut source: Box<dyn Iterator<Item = i32>>) -> Self {
        let day0 = source
            .next()
            .expect("the source should have atleast one day");
        Self {
            source,
            window: [day0].into(),
            sort_inds: [0].into(),
            longest_streak: 1,
            best_final_day: 0,
            max: day0,
            min: day0,
            tolerance: 5,
            current_day: 0,
        }
    }
    fn step(&mut self) -> Option<i32> {
        let next_day = self.source.next()?;
        self.window.push_back(next_day);
        self.current_day += 1;
        let mut outside = false;

        if next_day > self.max {
            self.max = next_day;
            outside = self.outside_tol();
        } else if next_day < self.min {
            self.min = next_day;
            outside = self.outside_tol();
        }
        if outside {
            self.shrink();
            return Some(next_day);
        }
        if self.window.len() > self.longest_streak {
            self.best_final_day = self.current_day;
            self.longest_streak = self.window.len();
        }
        Some(next_day)
    }

    fn outside_tol(&mut self) -> bool {
        (self.max - self.min) > self.tolerance
    }

    fn shrink(&mut self) {
        loop {
            let dropped = self
                .window
                .pop_front()
                .expect("shrink should stop before window is empty");
            if dropped == self.min {
                self.min = *self.window.iter().min().expect("not empty");
                if !self.outside_tol() {
                    return;
                }
            } else if dropped == self.max {
                self.max = *self.window.iter().max().expect("not empty");
                if !self.outside_tol() {
                    return;
                }
            }
        }
    }
    fn best_range(&self) -> (usize, usize) {
        (
            1 + self.best_final_day - self.longest_streak,
            self.best_final_day + 1,
        )
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let temps: Vec<i32> = (0..50).map(|_| rng.gen_range(0..12)).collect();
    println!("temps: {:#?}", temps);
    let source = temps.clone().into_iter();
    let mut scanner = Scanner::new(Box::new(source));
    println!("{}\n", scanner);
    while let Some(_) = scanner.step() {
        println!("{}\n", scanner);
    }
    let (start, end) = scanner.best_range();
    println!("best range: {}..{}", start, end);
    println!("best temps: {:?}", &temps[start..end])
}
