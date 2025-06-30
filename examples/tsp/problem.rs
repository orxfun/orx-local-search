use crate::{
    insert_move::InsertMove, neighborhood::AllInsertMovesIter, tour::Tour, tour_cost::TourCost,
};
use orx_iterable::Collection;
use orx_local_search::{Move, Problem};

pub struct Tsp;

impl Problem for Tsp {
    type ObjectiveValue = TourCost;

    type Move = InsertMove;

    fn neighborhood<'a, 'b, 'c>(
        tour: &'b <Self::Move as Move>::Solution,
    ) -> impl Iterator<Item = Self::Move> + 'a + 'b + 'c {
        AllInsertMovesIter::new(tour.iter().len())
    }
}
