use crate::{tour::Tour, tour_cost::TourCost};
use orx_local_search::Problem;

#[derive(Default, Clone, Copy)]
pub struct Tsp;

impl Problem for Tsp {
    type Objective = TourCost;

    type Solution = Tour;
}
