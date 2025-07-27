#[test]
fn test_duration_matrix() {
    use crate::criteria::duration::*;
    use crate::*;
    use orx_iterable::Collection;
    use orx_local_search::*;

    let matrix = DurationMatrix::new(vec![
        vec![0, 3, 1, 7],
        vec![5, 0, 9, 6],
        vec![1, 2, 0, 8],
        vec![9, 2, 7, 0],
    ]);

    let tour = Tour::new(vec![3, 1, 0, 2]);
    assert_eq!(matrix.tour_cost(&tour), 2 + 5 + 1);

    for mv in AllInsertMovesIter::new(tour.iter().len()) {
        let cost = matrix.tour_cost_after_move(&tour, &mv);

        let tour = mv.apply(tour.clone());
        let expected = matrix.tour_cost(&tour);

        assert_eq!(cost, expected);
    }
}
