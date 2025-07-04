use crate::{
    criteria::DurationMatrix,
    insert::{
        AllInsertMovesIter, InsertMove, TourAfterInsertIter, neighborhood::InsertNeighborhood,
    },
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::{CandidateMove, CandidateMoveOf};

pub struct DurationMoves<'a> {
    tour: &'a Tour,
    input: &'a DurationMatrix,
    iter: AllInsertMovesIter,
}

impl<'a> DurationMoves<'a> {
    pub fn new(tour: &'a Tour, input: &'a DurationMatrix) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self { tour, input, iter }
    }

    fn tour_cost_after_move(&self, mv: &InsertMove) -> u64 {
        let mut cost = 0;
        let mut new_tour = TourAfterInsertIter::new(mv.clone(), self.tour);
        if let Some(mut a) = new_tour.next() {
            for b in new_tour {
                cost += self.input.get(a, b);
                a = b;
            }
        }
        cost
    }
}

impl<'a> Iterator for DurationMoves<'a> {
    type Item = CandidateMoveOf<InsertNeighborhood>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.tour_cost_after_move(&mv);
            CandidateMove::new(mv, cost)
        })
    }
}
