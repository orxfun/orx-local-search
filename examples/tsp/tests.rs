use crate::{
    criteria::duration, insert_move::test_insert_move, neighborhood::test_all_insert_moves_iter,
    tour_after_move::test_tour_after_move,
};

pub fn run() {
    test_insert_move();
    test_all_insert_moves_iter();
    test_tour_after_move();

    duration::tests::run();
}
