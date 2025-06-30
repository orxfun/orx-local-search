use crate::{
    all_moves::AllInsertMovesIter, insert_move::InsertMove, tour::Tour,
    tour_after_move::TourAfterInsertIter,
};
use orx_iterable::Collection;
use orx_local_search::Move;

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
        let mut cost = 0;
        let mut iter = TourAfterInsertIter::new(mv.clone(), tour);
        if let Some(mut a) = iter.next() {
            for b in iter {
                cost += self.get(a, b);
                a = b;
            }
        }
        cost
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

    for mv in AllInsertMovesIter::new(tour.iter().len()) {
        let cost = matrix.tour_cost_after_move(&tour, &mv);

        let tour = mv.apply(tour.clone());
        let expected = matrix.tour_cost(&tour);

        assert_eq!(cost, expected);
    }
}
