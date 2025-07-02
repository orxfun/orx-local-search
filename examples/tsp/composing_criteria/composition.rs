use crate::{
    Tour,
    criteria::{
        capacity::{Capacity, CapacityInput},
        duration::{Duration, DurationMatrix},
        time_windows::{TimeWindowInput, TimeWindows},
    },
};
use orx_local_search::{Criterion, LocalSearch};

fn print(
    tour: &Tour,
    ((input_duration, input_capacity), input_time_windows): &(
        (DurationMatrix, CapacityInput),
        TimeWindowInput,
    ),
) {
    let cost_duration = Duration.evaluate(&tour, &input_duration).unwrap();
    let cost_capacity = Capacity.evaluate(&tour, &input_capacity).unwrap();
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

    let my_tsp = Duration.compose(Capacity).compose(TimeWindows);

    let input = my_tsp
        .input_builder()
        .add(DurationMatrix::example_input())
        .add(CapacityInput::example_input())
        .add(TimeWindowInput::example_input())
        .value();

    let initial_tour = Tour::example_solution();

    print(&initial_tour, &input);

    let mut local_search = LocalSearch::new(my_tsp);
    let (tour, _) = local_search
        .optimize(initial_tour, &input, None)
        .into_local_optimum()
        .unwrap();

    print(&tour, &input);
}
