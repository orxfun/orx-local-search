#![doc = include_str!("../README.md")]
#![warn(
    // missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![cfg_attr(not(test), no_std)]

mod crit;
mod crit_on_neighborhood;
mod eval_move;
mod ls;
mod r#move;
mod move_gen;
mod neighborhood;
mod objective;
mod problem;
mod solution;
mod symbolic;

pub use crit::Criterion;
pub use crit_on_neighborhood::CriterionOnNeighborhood;
pub use r#move::Move;
pub use move_gen::MoveGenerator;
pub use neighborhood::Neighborhood;
pub use objective::Objective;
pub use problem::Problem;
pub use solution::Solution;
