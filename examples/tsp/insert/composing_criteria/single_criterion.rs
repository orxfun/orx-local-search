use crate::{
    Tour,
    insert::criteria::duration::{Duration, DurationMatrix},
};
use orx_local_search::{Criterion, LocalSearch};

fn print(tour: &Tour, input_duration: &DurationMatrix) {
    let cost_duration = Duration.evaluate(&tour, &input_duration).unwrap();

    println!("tour: {:?}", &tour);
    println!("cost - Duration: {:?}", cost_duration);
}

pub fn run() {
    println!("\n\nRunning with single explicit criterion for (Duration).");

    let my_tsp = Duration::new();

    let input_duration = DurationMatrix::example_input();

    let initial_tour = Tour::example_solution();

    println!("\nInitial Solution");
    print(&initial_tour, &input_duration);

    let mut local_search = LocalSearch::new(my_tsp);
    let (tour, _) = local_search
        .optimize(initial_tour, &input_duration, None)
        .into_local_optimum()
        .unwrap();

    println!("\nOptimized Solution");
    print(&tour, &input_duration);
}
