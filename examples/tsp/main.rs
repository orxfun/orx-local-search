use orx_local_search::Ls;

use crate::{insert_neighborhood::InsertNeighborhood, tsp::Tsp};

#[cfg(test)]
mod tests;

mod criteria;
mod insert_neighborhood;
mod tour;
mod tour_cost;
mod tsp;

// pub use tour::Tour;
// pub use tour_cost::TourCost;

fn main() {
    let ls = Ls::<Tsp, InsertNeighborhood, _>::new();
}
