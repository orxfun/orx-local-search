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

use crate::composing_criteria::{
    composition_3criteria, composition_5criteria, explicit_3criteria, explicit_5criteria,
    single_criterion,
};

fn main() {
    single_criterion::run();
    explicit_3criteria::run();
    explicit_5criteria::run();
    composition_3criteria::run();
    composition_5criteria::run();
}
