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

mod cand_move;
mod crit;
mod crit_on;
mod local_search_on;
mod r#move;
mod move_generator;
mod neighborhood;
mod obj;
mod problem;
mod solution;

pub use obj::Objective;
pub use problem::Problem;
