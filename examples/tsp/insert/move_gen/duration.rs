use crate::{
    criteria::duration::{Duration, DurationMatrix},
    insert::{
        neighborhood::{AllInsertMovesIter, Insert, InsertMove},
        tour_after_move::TourAfterInsertIter,
    },
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::{Criterion, EvalMove, MoveGenerator, Problem};
use orx_meta::queue::NonEmptyQueue;

#[derive(Default)]
pub struct InsertForDuration;

impl<'i> MoveGenerator<'i> for InsertForDuration {
    type Neighborhood = Insert;

    type X = Duration;

    fn moves<'a>(
        &'a mut self,
        input: <Self::X as Criterion>::Input<'i>,
        tour: &'a <<Self::X as Criterion>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        DurationMoves {
            iter: AllInsertMovesIter::new(tour.iter().len()),
            duration_matrix: input.front(),
            tour,
        }
    }
}

// moves iter

pub struct DurationMoves<'a> {
    tour: &'a Tour,
    duration_matrix: &'a DurationMatrix,
    iter: AllInsertMovesIter,
}

impl<'a> DurationMoves<'a> {
    fn tour_cost_after_move(&self, mv: &InsertMove) -> u64 {
        let mut cost = 0;
        let mut new_tour = TourAfterInsertIter::new(mv.clone(), self.tour);
        if let Some(mut a) = new_tour.next() {
            for b in new_tour {
                cost += self.duration_matrix.get(a, b);
                a = b;
            }
        }
        cost
    }
}

impl<'a> Iterator for DurationMoves<'a> {
    type Item = EvalMove<Insert>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.tour_cost_after_move(&mv);
            EvalMove::new(mv, cost)
        })
    }
}
