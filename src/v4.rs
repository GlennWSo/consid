use std::{collections::VecDeque, fmt::Display};

use crate::Scanner;

/// this Scanner tries to remeber where 2 extrema are in the moving window
/// To point of this is to speed up shrinking
/// but for most cenerios the cost of finding extra does not payoff, unless
/// tolerance and distrubtions of days are set up so that very long streaks/windows sizes occur
pub struct Scanner4 {
    source: Box<dyn Iterator<Item = i32>>,
    window: VecDeque<i32>,
    best_final_day: usize,
    best_len: usize,
    current_day: usize,
    max: i32,
    max_ind: usize,
    min: i32,
    min_ind: usize,
    tolerance: i32,
}

impl Display for Scanner4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner: ")?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tmax: {:?}", self.max)?;
        writeln!(f, "\twindow: {:?}", self.window)?;
        writeln!(f, "\tlongest_streak: {:?}", self.best_len)?;
        write!(f, "\tbest_final_day: {:?}", self.best_final_day)
    }
}

impl Scanner4 {
    /// is the max - min of current window ok?
    fn check(&self) -> bool {
        (self.max - self.min) <= self.tolerance
    }
    fn find_max(&mut self) {
        self.max = i32::MIN;
        for (i, &value) in self.window.iter().enumerate() {
            if value > self.max {
                self.max = value;
                self.max_ind = i;
            }
        }
    }
    fn find_min(&mut self) {
        self.min = i32::MAX;
        for (i, &value) in self.window.iter().enumerate() {
            if value < self.min {
                self.min = value;
                self.min_ind = i;
            }
        }
    }
    fn remove_max(&mut self) {
        self.window.drain(0..=self.max_ind);
        self.min_ind = self.window.len() - 1;
        self.find_max();
    }
    fn remove_min(&mut self) {
        self.window.drain(0..=self.min_ind);
        self.max_ind = self.window.len() - 1;
        self.find_min();
    }
    fn reduce_max(&mut self) {
        while !self.check() {
            self.remove_max()
        }
    }
    fn reduce_min(&mut self) {
        while !self.check() {
            self.remove_min()
        }
    }
}

impl Scanner for Scanner4 {
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
            self.max_ind = self.window.len() - 1;
            self.reduce_min();
        } else if current_temp < self.min {
            self.min = current_temp;
            self.min_ind = self.window.len() - 1;
            self.reduce_max();
        }

        if self.window.len() > self.best_len {
            self.best_final_day = self.current_day;
            self.best_len = self.window.len();
        }
        Some(current_temp)
    }
}
