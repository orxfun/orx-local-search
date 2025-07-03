use super::{AllInsertMovesIter, InsertMove};
use crate::{Tour, tour_cost::TourCost};
use orx_iterable::Collection;
use orx_local_search::{Move, Problem};

#[derive(Default, Clone, Copy)]
pub struct Tsp;

impl Problem for Tsp {
    type ObjectiveValue = TourCost;

    type Solution = Tour;

    type Move = InsertMove;

    fn neighborhood(tour: &<Self::Move as Move>::Solution) -> impl Iterator<Item = Self::Move> {
        AllInsertMovesIter::new(tour.iter().len())
    }
}
