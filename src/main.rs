use rand::prelude::*;

#[allow(unused)]
use rand_chacha::ChaCha8Rng;
use std::{collections::VecDeque, fmt::Display};

use consid::{OldScanner, Scanner};

struct NewScanner {
    source: Box<dyn Iterator<Item = i32>>,
    window: VecDeque<i32>,
    sort_inds: Vec<usize>,
    best_final_day: usize,
    best_len: usize,
    current_day: usize,
    max: i32,
    max_ind: usize,
    min: i32,
    min_ind: usize,
    tolerance: i32,
}

impl Display for NewScanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner: ")?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tmax: {:?}", self.max)?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tlongest_streak: {:?}", self.best_len)?;
        write!(f, "\tbest_final_day: {:?}", self.best_final_day)
    }
}

impl NewScanner {
    /// is the max - min of current window ok?
    fn check(&self) -> bool {
        (self.max - self.min) <= self.tolerance
    }
    /// drain from 0 until the current highest value has been dropped
    fn remove_max(&mut self) {
        self.window.drain(0..=self.max_ind);
        self.sort_inds.clear();
        self.sort_inds.extend(0..self.window.len());
        self.max_ind = *match self.sort_inds.iter().max_by_key(|&&i| self.window[i]) {
            Some(v) => v,
            None => {
                println!("{}", self);
                panic!("how did we get here")
            }
        };
        self.max = self.window[self.max_ind];
    }
    fn remove_min(&mut self) {
        self.window.drain(0..=self.min_ind);
        self.sort_inds.clear();
        self.sort_inds.extend(0..self.window.len());

        self.min_ind = *self
            .sort_inds
            .iter()
            .min_by_key(|&&i| self.window[i])
            .expect("should not be empty");
        self.min = self.window[self.min_ind];
    }
}

impl Scanner for NewScanner {
    fn new(mut source: Box<dyn Iterator<Item = i32>>) -> Self {
        let day0 = source
            .next()
            .expect("the source should have atleast one day");
        Self {
            source,
            window: [day0].into(),
            sort_inds: [0].into(),
            best_len: 1,
            best_final_day: 0,
            max: day0,
            min: day0,
            max_ind: 0,
            min_ind: 0,
            tolerance: 5,
            current_day: 0,
        }
    }

    fn best_range(&self) -> (usize, usize) {
        (
            1 + self.best_final_day - self.best_len,
            self.best_final_day + 1,
        )
    }

    fn step(&mut self) -> Option<i32> {
        let current_temp = self.source.next()?;
        self.window.push_back(current_temp);
        self.current_day += 1;

        if current_temp > self.max {
            self.max = current_temp;
            while !self.check() {
                self.remove_min();
            }
            self.max_ind = self.window.len() - 1;
        } else if current_temp < self.min {
            self.min = current_temp;
            while !self.check() {
                self.remove_max();
            }
            self.min_ind = self.window.len() - 1;
        }
        if self.window.len() > self.best_len {
            self.best_final_day = self.current_day;
            self.best_len = self.window.len();
        }
        Some(current_temp)
    }
}

fn main() {
    // let mut rng = ChaCha8Rng::seed_from_u64(1);
    let mut rng = thread_rng();
    let temps: Vec<i32> = (0..1000000).map(|_| rng.gen_range(0..15)).collect();
    // println!("temps: {:#?}", temps);
    let source = temps.clone().into_iter();
    let mut scanner = OldScanner::new(Box::new(source.clone()));
    let time = scanner.time_me();
    println!("old time: {:.3?}", time);
    let (start, end) = scanner.best_range();
    {
        let mut new_scanner = NewScanner::new(Box::new(source));
        let time = new_scanner.time_me();
        println!("new time: {:.3?}", time);
        assert_eq!((start, end), new_scanner.best_range());
    }
    println!("{}", scanner);
    println!("longest range: {}..{}", start, end);

    if start > 0 {
        println!("pre temp: {}", temps[start - 1]);
    }
    println!("temps: {:?}", &temps[start..end]);
    if end < (temps.len() - 1) {
        println!("post temp: {}", temps[end + 1]);
    }
}
