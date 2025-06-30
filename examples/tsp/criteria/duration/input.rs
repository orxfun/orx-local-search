use crate::{insert_move::InsertMove, tour::Tour};
use orx_iterable::Collection;
use std::cmp::Ordering;

pub struct DurationMatrix(Vec<Vec<u64>>);

impl DurationMatrix {
    pub fn new(matrix: Vec<Vec<u64>>) -> Self {
        Self(matrix)
    }

    pub fn get(&self, from_city: usize, to_city: usize) -> u64 {
        self.0[from_city][to_city]
    }

    pub fn tour_cost(&self, tour: &Tour) -> u64 {
        let mut cost = 0;
        for p in 0..tour.iter().len().saturating_sub(1) {
            cost += self.get(tour[p], tour[p + 1]);
        }
        cost
    }

    pub fn tour_cost_after_move(&self, tour: &Tour, mv: &InsertMove) -> u64 {
        match mv.current_position.cmp(&mv.target_position) {
            Ordering::Equal => self.tour_cost(tour),
            Ordering::Less => {
                let mut cost = 0;
                for p in 0..(mv.current_position.saturating_sub(1)) {
                    cost += self.get(tour[p], tour[p + 1]);
                }
                if mv.current_position > 0 {
                    cost += self.get(tour[mv.current_position - 1], tour[mv.current_position + 1]);
                }
                for p in (mv.current_position + 1)..mv.target_position {
                    cost += self.get(tour[p], tour[p + 1]);
                }
                cost += self.get(tour[mv.target_position], tour[mv.current_position]);
                if mv.target_position + 1 < tour.iter().len() {
                    cost += self.get(tour[mv.current_position], tour[mv.target_position + 1]);
                }
                for p in (mv.target_position + 1)..tour.iter().len().saturating_sub(1) {
                    cost += self.get(tour[p], tour[p + 1]);
                }
                cost
            }
            Ordering::Greater => {
                let mut cost = 0;
                for p in 0..(mv.target_position.saturating_sub(1)) {
                    cost += self.get(tour[p], tour[p + 1]);
                }
                if mv.target_position > 0 {
                    cost += self.get(tour[mv.target_position - 1], tour[mv.current_position]);
                }
                cost += self.get(tour[mv.current_position], tour[mv.target_position + 1]);
                for p in (mv.target_position + 1)..(mv.current_position - 1) {
                    cost += self.get(tour[p], tour[p + 1]);
                }
                if mv.current_position + 1 < tour.iter().len() {
                    cost += self.get(tour[mv.current_position - 1], tour[mv.current_position + 1]);
                }
                for p in (mv.current_position + 1)..tour.iter().len().saturating_sub(1) {
                    cost += self.get(tour[p], tour[p + 1]);
                }
                cost
            }
        }
    }
}

pub fn test_duration_matrix() {
    let matrix = DurationMatrix::new(vec![
        vec![0, 3, 1, 7],
        vec![5, 0, 9, 6],
        vec![1, 2, 0, 8],
        vec![9, 2, 7, 0],
    ]);

    let tour = Tour::new(vec![3, 1, 0, 2]);
    assert_eq!(matrix.tour_cost(&tour), 2 + 5 + 1);

    let mv = InsertMove::new(2, 2);
    assert_eq!(
        matrix.tour_cost_after_move(&tour, &mv),
        matrix.tour_cost(&tour),
    );

    // let mv = InsertMove::new(1, 2);
    // assert_eq!(matrix.tour_cost_after_move(&tour, &mv), 9 + 3 + 9);

    // let mv = InsertMove::new(2, 1);
    // assert_eq!(matrix.tour_cost_after_move(&tour, &mv), 9 + 3 + 9);
}
