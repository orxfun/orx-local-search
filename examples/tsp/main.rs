use crate::insert::{move_gen::duration::InsertMovesForDuration, neighborhood::Insert};
use orx_local_search::LocalSearch;

mod criteria;
mod insert;
mod tour;
mod tour_cost;
mod tsp;

fn main() {
    let ls = LocalSearch::<Insert, _>::new()
        .compose::<InsertMovesForDuration>()
        .compose::<InsertMovesForDuration>()
        .compose::<InsertMovesForDuration>();
}
