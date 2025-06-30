use crate::{insert_move::InsertMove, tour::Tour};

pub struct DurationMatrix(Vec<Vec<u64>>);

impl DurationMatrix {
    pub fn get(&self, from_city: usize, to_city: usize) -> u64 {
        self.0[from_city][to_city]
    }

    pub fn tour_cost(&self, tour: &Tour, mv: &InsertMove) -> u64 {
        3
    }
}
