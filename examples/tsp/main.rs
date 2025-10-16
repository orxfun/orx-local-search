use crate::{
    criteria::{capacity::CapacityInput, duration::DurationMatrix, time_windows::TimeWindowsInput},
    insert::{
        move_gen::{
            capacity::InsertForCapacity, duration::InsertForDuration,
            time_windows::InsertForTimeWindows,
        },
        neighborhood::Insert,
    },
    tour::Tour,
    tsp::Tsp,
};
// use orx_local_search::LocalSearch;

mod criteria;
mod insert;
mod tour;
mod tour_cost;
mod tsp;

fn main() {
    // let mut ls = LocalSearch::<Tsp, Insert, _>::new()
    //     .with::<InsertForDuration>()
    //     .with::<InsertForCapacity>()
    //     .with::<InsertForTimeWindows>();

    // let initial_tour = Tour::example_solution();

    // let duration_matrix = DurationMatrix::example_input();
    // let capacity_input = CapacityInput::example_input();
    // let time_window_input = TimeWindowsInput::example_input(&duration_matrix);

    // let input = ls
    //     .input_builder()
    //     .push_back(&duration_matrix)
    //     .push_back(&capacity_input)
    //     .push_back(&time_window_input)
    //     .finish();

    // let initial = ls.evaluate(&input, &initial_tour);
    // dbg!(initial);

    // let optimal = ls.run(&input, initial_tour.clone(), None);

    // dbg!(&optimal);

    // let tour = optimal.into_local_optimum().unwrap().0;

    // let r#final = ls.evaluate(&input, &tour);
    // dbg!(r#final);

    // // alternatively

    // let input = (&duration_matrix, &capacity_input, &time_window_input).into();

    // let initial = ls.evaluate(&input, &initial_tour);
    // dbg!(initial);

    // let optimal = ls.run(&input, initial_tour, None);

    // dbg!(&optimal);

    // let tour = optimal.into_local_optimum().unwrap().0;

    // let r#final = ls.evaluate(&input, &tour);
    // dbg!(r#final);
}
