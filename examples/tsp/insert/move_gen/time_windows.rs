use crate::{
    criteria::time_windows::{TimeWindows, TimeWindowsInput},
    insert::{
        neighborhood::{AllInsertMovesIter, Insert, InsertMove},
        tour_after_move::TourAfterInsertIter,
    },
    tour::Tour,
    tsp::Tsp,
};
use orx_iterable::Collection;
use orx_local_search::{Criteria1, Criterion, EvalMove, Moves, Problem};

#[derive(Default)]
pub struct InsertForTimeWindows;

impl<'i> Moves<'i, Tsp, Insert> for InsertForTimeWindows {
    type X = Criteria1<Tsp, TimeWindows>;

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as Criterion<Tsp>>::Input<'i>,
        tour: &'a <Tsp as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Tsp, Insert>> + 'a
    where
        'i: 'a,
    {
        let input = *input.value().value();
        TimeWindowsMoves {
            iter: AllInsertMovesIter::new(tour.iter().len()),
            input,
            tour,
        }
    }
}

// moves iter

pub struct TimeWindowsMoves<'a> {
    input: &'a TimeWindowsInput<'a>,
    tour: &'a Tour,
    iter: AllInsertMovesIter,
}

impl<'a> TimeWindowsMoves<'a> {
    fn tour_cost_after_move(&self, mv: &InsertMove) -> Option<u64> {
        let new_tour = TourAfterInsertIter::new(mv.clone(), self.tour);
        let first_city = new_tour.peek();
        self.input
            .tour_cost_for_cities_sequence(first_city, new_tour)
    }
}

impl<'a> Iterator for TimeWindowsMoves<'a> {
    type Item = EvalMove<Tsp, Insert>;

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
