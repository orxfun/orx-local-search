use crate::{InsertMove, Tour};
use std::{collections::HashMap, ops::Range};

pub struct TimeWindow {
    window: Range<u32>,
}

impl TimeWindow {
    pub fn new(begin: u32, end: u32) -> Self {
        assert!(end >= begin);
        Self { window: begin..end }
    }
}

pub struct TimeWindowInput {
    penalty_per_late_minutes: u32,
    max_allowed_lateness: u32,
    windows: Vec<Option<TimeWindow>>,
    start_time: u32,
}

impl TimeWindowInput {
    pub fn new(
        n: u32,
        penalty_per_late_minutes: u32,
        max_allowed_lateness: u32,
        start_time: u32,
        time_windows: HashMap<usize, TimeWindow>,
    ) -> Self {
        let mut windows: Vec<_> = (0..n).map(|_| None).collect();
        for (city, window) in time_windows {
            windows[city] = Some(window);
        }

        Self {
            penalty_per_late_minutes,
            max_allowed_lateness,
            windows,
            start_time,
        }
    }

    pub fn tour_cost(&self, tour: &Tour) -> Option<u64> {
        None
    }

    pub fn tour_cost_after_move(&self, tour: &Tour, mv: &InsertMove) -> Option<u64> {
        None
    }
}
