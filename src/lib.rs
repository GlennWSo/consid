pub mod v2;

use std::time::{Duration, Instant};
use std::{collections::VecDeque, fmt::Display};

pub trait Scanner {
    fn new(source: Box<dyn Iterator<Item = i32>>, tolerance: i32) -> Self;
    fn best_range(&self) -> (usize, usize);
    fn step(&mut self) -> Option<i32>;
    fn seek(&mut self) {
        while let Some(_current_temp) = self.step() {}
    }
    fn time_me(&mut self) -> Duration {
        let start = Instant::now();
        self.seek();
        start.elapsed()
    }
}

pub struct OldScanner {
    source: Box<dyn Iterator<Item = i32>>,
    window: VecDeque<i32>,
    best_len: usize,
    best_final_day: usize,
    max: i32,
    min: i32,
    tolerance: i32,
    current_day: usize,
}

impl Display for OldScanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner: ")?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tlongest_streak: {:?}", self.best_len)?;
        write!(f, "\tbest_final_day: {:?}", self.best_final_day)
    }
}

impl OldScanner {
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
}

impl Scanner for OldScanner {
    fn new(mut source: Box<dyn Iterator<Item = i32>>, tolerance: i32) -> Self {
        let day0 = source
            .next()
            .expect("the source should have atleast one day");
        Self {
            source,
            tolerance,
            window: [day0].into(),
            best_len: 1,
            best_final_day: 0,
            max: day0,
            min: day0,
            current_day: 0,
        }
    }

    fn step(&mut self) -> Option<i32> {
        let current_temp = self.source.next()?;
        self.window.push_back(current_temp);
        self.current_day += 1;
        let mut outside = false;

        if current_temp > self.max {
            self.max = current_temp;
            outside = self.outside_tol();
        } else if current_temp < self.min {
            self.min = current_temp;
            outside = self.outside_tol();
        }
        if outside {
            self.shrink();
            return Some(current_temp);
        }
        if self.window.len() > self.best_len {
            self.best_final_day = self.current_day;
            self.best_len = self.window.len();
        }
        Some(current_temp)
    }

    fn best_range(&self) -> (usize, usize) {
        (
            1 + self.best_final_day - self.best_len,
            self.best_final_day + 1,
        )
    }
}
