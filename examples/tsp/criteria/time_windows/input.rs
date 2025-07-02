use crate::TourAfterInsertIter;
use crate::{InsertMove, Tour, criteria::duration::DurationMatrix};
use orx_iterable::Collection;
use std::cmp::max;
use std::collections::HashMap;

pub struct TimeWindow {
    begin: u64,
    end: u64,
}

impl TimeWindow {
    pub fn new(begin: u64, end: u64) -> Self {
        assert!(end >= begin);
        Self { begin, end }
    }
}

pub struct TimeWindowInput<'i> {
    duration_matrix: &'i DurationMatrix,
    penalty_per_late_minutes: u64,
    max_allowed_lateness: u64,
    windows: Vec<Option<TimeWindow>>,
    start_time: u64,
}

impl<'i> TimeWindowInput<'i> {
    pub fn new(
        duration_matrix: &'i DurationMatrix,
        penalty_per_late_minutes: u64,
        max_allowed_lateness: u64,
        start_time: u64,
        time_windows: HashMap<usize, TimeWindow>,
    ) -> Self {
        let n = duration_matrix.tour_len();
        let mut windows: Vec<_> = (0..n).map(|_| None).collect();
        for (city, window) in time_windows {
            windows[city] = Some(window);
        }

        Self {
            duration_matrix,
            penalty_per_late_minutes,
            max_allowed_lateness,
            windows,
            start_time,
        }
    }

    pub fn tour_cost(&self, tour: &Tour) -> Option<u64> {
        let first_city = tour.get(0);
        self.tour_cost_for_cities_sequence(first_city, tour.iter().copied())
    }

    pub fn tour_cost_after_move(&self, tour: &Tour, mv: &InsertMove) -> Option<u64> {
        let new_tour = TourAfterInsertIter::new(mv.clone(), tour);
        let first_city = new_tour.peek();
        self.tour_cost_for_cities_sequence(first_city, new_tour)
    }

    fn tour_cost_for_cities_sequence(
        &self,
        first_city: Option<usize>,
        cities: impl Iterator<Item = usize>,
    ) -> Option<u64> {
        match first_city {
            None => Some(0),
            Some(mut prev_city) => {
                let mut total_lateness = 0;
                let mut arrival_time = self.start_time;

                for city in cities {
                    let window = &self.windows[city];
                    let travel_time = self.duration_matrix.get(prev_city, city);

                    match window {
                        None => {
                            arrival_time += travel_time;
                        }
                        Some(w) => {
                            arrival_time = max(arrival_time + travel_time, w.begin);
                            let lateness = arrival_time.saturating_sub(w.end);
                            if lateness > self.max_allowed_lateness {
                                return None;
                            }
                            total_lateness += lateness;
                        }
                    }

                    prev_city = city;
                }

                let lateness_penalty = self.penalty_per_late_minutes * total_lateness;
                Some(lateness_penalty)
            }
        }
    }

    pub fn example_input(duration_matrix: &'i DurationMatrix) -> Self {
        let windows = HashMap::<usize, TimeWindow>::from_iter([
            (1, TimeWindow::new(400, 500)),
            (3, TimeWindow::new(550, 850)),
        ]);

        Self::new(duration_matrix, 2, 120, 480, windows)
    }
}
