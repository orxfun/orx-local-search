use crate::{
    Tour,
    criteria::TimeWindowInput,
    insert::{
        AllInsertMovesIter, InsertMove, TourAfterInsertIter, neighborhood::InsertNeighborhood,
    },
};
use orx_iterable::Collection;
use orx_local_search::{CandidateMove, CandidateMoveOf};

pub struct TimeWindowMoves<'a> {
    tour: &'a Tour,
    input: &'a TimeWindowInput<'a>,
    iter: AllInsertMovesIter,
}

impl<'a> TimeWindowMoves<'a> {
    pub fn new(tour: &'a Tour, input: &'a TimeWindowInput) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self { tour, input, iter }
    }

    fn tour_cost_after_move(&self, mv: &InsertMove) -> Option<u64> {
        let new_tour = TourAfterInsertIter::new(mv.clone(), self.tour);
        let first_city = new_tour.peek();
        self.input
            .tour_cost_for_cities_sequence(first_city, new_tour)
    }
}

impl<'a> Iterator for TimeWindowMoves<'a> {
    type Item = CandidateMoveOf<InsertNeighborhood>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(mv) => {
                    match self.tour_cost_after_move(&mv) {
                        None => { /* infeasible move, continue to the next */ }
                        Some(cost) => return Some(CandidateMove::new(mv, cost)),
                    }
                }
            }
        }
    }
}
