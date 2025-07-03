use crate::{
    Tour,
    insert::{InsertMove, TourAfterInsertIter},
};
use orx_iterable::Collection;

pub struct DurationMatrix(Vec<Vec<u64>>);

impl DurationMatrix {
    pub fn new(matrix: Vec<Vec<u64>>) -> Self {
        Self(matrix)
    }

    pub fn get(&self, from_city: usize, to_city: usize) -> u64 {
        self.0[from_city][to_city]
    }

    pub fn tour_len(&self) -> usize {
        self.0.len()
    }

    pub fn tour_cost(&self, tour: &Tour) -> u64 {
        let mut cost = 0;
        for p in 0..tour.iter().len().saturating_sub(1) {
            cost += self.get(tour[p], tour[p + 1]);
        }
        cost
    }

    pub fn tour_cost_after_move(&self, tour: &Tour, mv: &InsertMove) -> u64 {
        let mut cost = 0;
        let mut new_tour = TourAfterInsertIter::new(mv.clone(), tour);
        if let Some(mut a) = new_tour.next() {
            for b in new_tour {
                cost += self.get(a, b);
                a = b;
            }
        }
        cost
    }

    pub fn example_input() -> Self {
        Self(vec![
            vec![0, 30, 60, 20, 5, 120],
            vec![20, 0, 10, 50, 80, 70],
            vec![10, 50, 0, 30, 40, 10],
            vec![50, 70, 60, 0, 140, 70],
            vec![10, 80, 30, 50, 0, 20],
            vec![100, 20, 50, 50, 80, 0],
        ])
    }
}
