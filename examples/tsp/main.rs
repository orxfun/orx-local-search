use crate::insert::{move_gen::duration::InsertForDuration, neighborhood::Insert};
use orx_local_search::LocalSearch;

mod criteria;
mod insert;
mod tour;
mod tour_cost;
mod tsp;

fn main() {
    let ls = LocalSearch
        .on::<Insert>()
        .for_criterion::<InsertForDuration>()
        .for_criterion::<InsertForDuration>()
        .for_criterion::<InsertForDuration>();
}
