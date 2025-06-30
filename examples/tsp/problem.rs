use crate::{
    neighborhood::AllInsertMovesIter, insert_move::InsertMove, tour::Tour, tour_cost::TourCost,
};
use orx_iterable::Collection;
use orx_local_search::Problem;

pub struct Tsp;

impl Problem for Tsp {
    type Solution = Tour;

    type ObjectiveValue = TourCost;

    type Move = InsertMove;

    fn neighborhood(tour: &Self::Solution) -> impl Iterator<Item = Self::Move> {
        AllInsertMovesIter::new(tour.iter().len())
    }
}
