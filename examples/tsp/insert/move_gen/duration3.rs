use crate::{
    criteria::{
        duration::{Duration, DurationMatrix},
        duration3::{Duration3, DurationMatrix3},
    },
    insert::{
        neighborhood::{AllInsertMovesIter, Insert, InsertMove},
        tour_after_move::TourAfterInsertIter,
    },
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::{Criterion, EvalMove, Moves, Problem};

#[derive(Default)]
pub struct InsertForDuration3;

impl<'i> Moves<'i> for InsertForDuration3 {
    type Neighborhood = Insert;

    type X = Duration3;

    fn moves<'a>(
        &'a mut self,
        duration_matrix: &'i <Self::X as Criterion>::Input<'i>,
        tour: &'a <<Self::X as Criterion>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        DurationMoves {
            iter: AllInsertMovesIter::new(tour.iter().len()),
            duration_matrix,
            tour,
        }
    }
}

// moves iter

pub struct DurationMoves<'a> {
    tour: &'a Tour,
    duration_matrix: &'a DurationMatrix3,
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
