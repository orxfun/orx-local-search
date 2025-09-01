use crate::{
    criteria::{duration::DurationMatrix, duration2::DurationMatrix2, duration3::DurationMatrix3},
    insert::{
        move_gen::{
            duration::InsertForDuration, duration2::InsertForDuration2,
            duration3::InsertForDuration3,
        },
        neighborhood::Insert,
    },
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
        .for_criterion::<InsertForDuration2>()
        .for_criterion::<InsertForDuration3>();

    let initial_tour = Tour::example_solution();

    let input = ls
        .input_buidler()
        .push_back(DurationMatrix::example_input())
        .push_back(DurationMatrix2::example_input())
        .push_back(DurationMatrix3::example_input())
        .finish();

    let initial = ls.evaluate(&input, &initial_tour);
    dbg!(initial);

    let optimal = ls.run(&input, initial_tour, None);

    dbg!(&optimal);

    let tour = optimal.into_local_optimum().unwrap().0;

    let r#final = ls.evaluate(&input, &tour);
    dbg!(r#final);
}
