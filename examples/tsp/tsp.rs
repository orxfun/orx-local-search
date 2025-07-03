use crate::{Tour, tour_cost::TourCost};
use orx_local_search::Problem;

#[derive(Default, Clone, Copy)]
pub struct Tsp;

impl Problem for Tsp {
    type ObjectiveValue = TourCost;

    type Solution = Tour;
}
