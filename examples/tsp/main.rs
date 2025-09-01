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
    // let mut ls = LocalSearch
    //     .on::<Insert>()
    //     .for_criterion::<InsertForDuration>()
    //     .for_criterion::<InsertForDuration2>()
    //     .for_criterion::<InsertForDuration3>();

    // let initial_tour = Tour::example_solution();

    // let a = DurationMatrix::example_input();
    // let b = DurationMatrix2::example_input();
    // let c = DurationMatrix3::example_input();

    // let input = ls
    //     .input_buidler()
    //     .push_back(&a)
    //     .push_back(&b)
    //     .push_back(&c)
    //     .finish();

    // let initial = ls.evaluate(input, &initial_tour);
    // dbg!(initial);

    // let optimal = ls.run(input, initial_tour, None);

    // dbg!(&optimal);

    // let tour = optimal.into_local_optimum().unwrap().0;

    // let r#final = ls.evaluate(input, &tour);
    // dbg!(r#final);
}

// Pair<Pair<Single<&DurationMatrix>, Single<&DurationMatrix>>, Single<&DurationMatrix>>

// PairOfInputs<'_, &DurationMatrix, PairOfInputs<'_, &DurationMatrix2, SingleInput<'_, &DurationMatrix3>>>
