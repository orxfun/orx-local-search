#[test]
fn test_tour_after_move() {
    use crate::*;
    use orx_iterable::Collection;
    use orx_local_search::*;

    let tour = Tour::new(vec![1, 3, 5, 2, 0, 4]);

    for mv in AllInsertMovesIter::new(tour.iter().len()) {
        let new_tour = Tour::new(TourAfterInsertIter::new(mv, &tour).collect());
        let expected_tour = mv.apply(tour.clone());
        assert_eq!(new_tour, expected_tour);
    }
}
