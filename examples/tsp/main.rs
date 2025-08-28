use crate::{
    insert::{move_gen::duration::InsertForDuration, neighborhood::Insert},
    tour::Tour,
};
use orx_local_search::LocalSearch;

mod criteria;
mod insert;
mod tour;
mod tour_cost;
mod tsp;

fn main() {
    let mut ls = LocalSearch
        .on::<Insert>()
        .for_criterion::<InsertForDuration>()
        .for_criterion::<InsertForDuration>()
        .for_criterion::<InsertForDuration>();

    let initial_tour = Tour::example_solution();

    // let solution = ls.run(12, initial_tour, None);
}

// Pair<Pair<Single<&DurationMatrix>, Single<&DurationMatrix>>, Single<&DurationMatrix>>
