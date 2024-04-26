use std::{collections::VecDeque, fmt::Display};

use crate::Scanner;

/// this Scanner tries to remeber where 2 extrema are in the moving window
/// To point of this is to speed up shrinking
/// but for most cenerios the cost of finding extra does not payoff, unless
/// tolerance and distrubtions of days are set up so that very long streaks/windows sizes occur
pub struct Scanner2 {
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

impl Display for Scanner2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner: ")?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tmax: {:?}", self.max)?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tlongest_streak: {:?}", self.best_len)?;
        write!(f, "\tbest_final_day: {:?}", self.best_final_day)
    }
}

impl Scanner2 {
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

impl Scanner for Scanner2 {
    fn new(mut source: Box<dyn Iterator<Item = i32>>, tolerance: i32) -> Self {
        let day0 = source
            .next()
            .expect("the source should have atleast one day");
        Self {
            source,
            tolerance,
            window: [day0].into(),
            sort_inds: [0].into(),
            best_len: 1,
            best_final_day: 0,
            max: day0,
            min: day0,
            max_ind: 0,
            min_ind: 0,
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
