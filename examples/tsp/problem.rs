use crate::{tour::Tour, tour_cost::TourCost};
use orx_local_search::Problem;

pub struct Tsp;

// impl Problem for Tsp {
//     type Solution = Tour;

//     type ObjectiveValue = TourCost;

//     type Move;

//     fn neighborhood(object: &Self::Solution) -> impl Iterator<Item = Self::Move> {
//         todo!()
//     }
// }
