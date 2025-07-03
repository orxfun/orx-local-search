#[cfg(test)]
mod tests;

mod composing_criteria;
pub mod criteria;
mod insert_move;
mod neighborhood;
mod problem;
mod tour;
mod tour_after_move;
mod tour_cost;

pub use insert_move::InsertMove;
pub use neighborhood::AllInsertMovesIter;
pub use tour::Tour;
pub use tour_after_move::TourAfterInsertIter;
pub use tour_cost::TourCost;

use crate::composing_criteria::{composition3, composition5, explicit1, explicit3, explicit5};

fn main() {
    explicit1::run();
    explicit3::run();
    explicit5::run();
    composition3::run();
    composition5::run();
}
