use crate::{
    criteria::TimeWindowsInput,
    insert_neighborhood::{
        r#move::Insert,
        neighborhood::{AllInsertMovesIter, InsertNeighborhood},
        tour_after_insert::TourAfterInsertIter,
    },
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::EvalMove;

pub struct TimeWindowsMoves<'a> {
    input: &'a TimeWindowsInput<'a>,
    tour: &'a Tour,
    iter: AllInsertMovesIter,
}

impl<'a> TimeWindowsMoves<'a> {
    pub fn new(input: &'a TimeWindowsInput, tour: &'a Tour) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self { tour, input, iter }
    }

    fn tour_cost_after_move(&self, mv: &Insert) -> Option<u64> {
        let new_tour = TourAfterInsertIter::new(mv.clone(), self.tour);
        let first_city = new_tour.peek();
        self.input
            .tour_cost_for_cities_sequence(first_city, new_tour)
    }
}

impl<'a> Iterator for TimeWindowsMoves<'a> {
    type Item = EvalMove<InsertNeighborhood>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(mv) => {
                    match self.tour_cost_after_move(&mv) {
                        None => { /* infeasible move, continue to the next */ }
                        Some(cost) => return Some(EvalMove::new(mv, cost)),
                    }
                }
            }
        }
    }
}
