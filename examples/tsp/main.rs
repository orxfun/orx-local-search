use crate::criteria::{
    capacity::{Capacity, CapacityInput},
    duration::{Duration, DurationMatrix},
    time_windows::{TimeWindows, TimeWindowsInput},
};
use crate::tour::Tour;
use orx_local_search::LocalSearch;

mod criteria;
mod insert;
mod tour;
mod tour_cost;
mod tsp;

fn main() {
    let initial_tour = Tour::example_solution();
    println!("initial-tour = {initial_tour:?}");

    let duration_matrix = DurationMatrix::example();
    let capacity_input = CapacityInput::example();
    let time_window_input = TimeWindowsInput::example(&duration_matrix);

    let mut alg = LocalSearch::new(Duration.insert())
        .and_with(Capacity.insert())
        .and_with(TimeWindows.insert());

    // using input builder

    let input = alg
        .input_builder()
        .push(&duration_matrix)
        .push(&capacity_input)
        .push(&time_window_input)
        .finish();

    let initial = alg.evaluate(&input, &initial_tour);
    println!("initial-cost = {initial:?}");

    let solution = alg.run(&input, initial_tour.clone(), None);
    println!("solution = {solution:?}");

    // alternatively using tuple

    let input = (&duration_matrix, &capacity_input, &time_window_input).into();

    let initial = alg.evaluate(&input, &initial_tour);
    println!("initial-cost = {initial:?}");

    let solution = alg.run(&input, initial_tour, None);
    println!("solution = {solution:?}");
}
