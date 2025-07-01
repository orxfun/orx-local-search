use crate::{insert_move::InsertMove, tour::Tour, tour_after_move::TourAfterInsertIter};
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
}
