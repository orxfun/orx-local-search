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

// mod composition;
mod criterion;
mod eval_move;
mod eval_soln;
// mod ls;
mod moves;
mod neighborhood;
mod objective;
mod problem;
mod solution;

pub use criterion::Criterion;
pub use eval_move::EvalMove;
pub use eval_soln::EvalSoln;
// pub use ls::LocalSearch;
pub use moves::Moves;
pub use neighborhood::Neighborhood;
pub use objective::Objective;
pub use problem::Problem;
