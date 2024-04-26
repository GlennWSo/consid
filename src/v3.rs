use std::time::{Duration, Instant};
use std::{collections::VecDeque, fmt::Display};

use crate::Scanner;

/// when shrinking is needed instead of checking how many too drop
/// check how many can be kept.
/// inital permance testing shows the wether this implementation is more performent then OldScanner
/// depends on the distrubion range and tolerance within a streak
pub struct Scanner3 {
    source: Box<dyn Iterator<Item = i32>>,
    window: VecDeque<i32>,
    best_len: usize,
    best_final_day: usize,
    max: i32,
    min: i32,
    tolerance: i32,
    current_day: usize,
}

impl Display for Scanner3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner: ")?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tlongest_streak: {:?}", self.best_len)?;
        write!(f, "\tbest_final_day: {:?}", self.best_final_day)
    }
}

impl Scanner3 {
    fn outside_tol(&mut self) -> bool {
        (self.max - self.min) > self.tolerance
    }
    fn remove_min(&mut self) {
        let perfect_min = self.max - self.tolerance;
        let mut min_iter = self.window.iter().copied().enumerate().rev();

        let mut index: usize;
        self.min = i32::MAX;
        (index, self.min) = min_iter
            .next()
            .expect("should have atleast 2 values if we need to shrink");
        for (i, v) in min_iter {
            index = i;
            if v < perfect_min {
                break;
            }
            if v < self.min {
                self.min = v;
            }
        }
        self.window.drain(0..=index);
    }
    fn remove_max(&mut self) {
        let perfect_max = self.tolerance + self.min;
        let max_iter = self.window.iter().copied().enumerate().rev();
        let mut index: usize = 0;

        self.max = i32::MIN;
        for (i, v) in max_iter {
            index = i;
            if v > perfect_max {
                break;
            }
            if v > self.max {
                self.max = v;
            }
        }
        self.window.drain(0..=index);
    }
}

impl Scanner for Scanner3 {
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

        // println!("{}", self);
        if current_temp > self.max {
            self.max = current_temp;
            if self.outside_tol() {
                // println!("pre remove_min min:{} w:{:?}", self.min, self.window);
                self.remove_min();
                // println!("warmer min: {} w: {:?}", self.min, self.window);
                return Some(current_temp);
            };
        } else if current_temp < self.min {
            self.min = current_temp;
            if self.outside_tol() {
                // println!("pre max: {} remove_max: {:?}", self.max, self.window);
                self.remove_max();
                // println!("max: {}, colder: {:?}", self.max, self.window);
                return Some(current_temp);
            };
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
