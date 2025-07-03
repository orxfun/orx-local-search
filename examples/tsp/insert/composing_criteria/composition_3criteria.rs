use super::super::criteria::{
    capacity::{CapacityInsert, CapacityInput},
    duration::{Duration, DurationMatrix},
    time_windows::{TimeWindowInput, TimeWindows},
};
use crate::Tour;
use orx_local_search::{Criterion, LocalSearch};

fn print(
    tour: &Tour,
    ((input_duration, input_capacity), input_time_windows): &(
        (&DurationMatrix, &CapacityInput),
        &TimeWindowInput,
    ),
) {
    let cost_duration = Duration.evaluate(&tour, &input_duration).unwrap();
    let cost_capacity = CapacityInsert.evaluate(&tour, &input_capacity).unwrap();
    let cost_time_windows = TimeWindows.evaluate(&tour, &input_time_windows).unwrap();
    let cost = cost_duration + cost_capacity + cost_time_windows;

    println!("tour: {:?}", &tour);
    println!("cost - Duration: {:?}", cost_duration);
    println!("cost - Capacity: {:?}", cost_capacity);
    println!("cost - TimeWindows: {:?}", cost_time_windows);
    println!("cost: {:?}", cost);
}

pub fn run() {
    println!("\n\nRunning with composed criteria for (Duration, Capacity, TimeWindows).");

    let my_tsp = Duration.compose(CapacityInsert).compose(TimeWindows);

    let input_duration = DurationMatrix::example_input();
    let input_capacity = CapacityInput::example_input();
    let input_time_windows = TimeWindowInput::example_input(&input_duration);

    let input = my_tsp
        .input_builder()
        .add(&input_duration)
        .add(&input_capacity)
        .add(&input_time_windows)
        .value();

    let initial_tour = Tour::example_solution();
    println!("\nInitial Solution");
    print(&initial_tour, &input);

    let mut local_search = LocalSearch::new(my_tsp);
    let (tour, _) = local_search
        .optimize(initial_tour, &input, None)
        .into_local_optimum()
        .unwrap();
    println!("\nOptimized Solution");
    print(&tour, &input);
}
