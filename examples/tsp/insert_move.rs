use crate::tour::Tour;
use orx_local_search::Move;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InsertMove {
    current_position: usize,
    target_position: usize,
}

impl InsertMove {
    pub fn new(current_position: usize, target_position: usize) -> Self {
        Self {
            current_position,
            target_position,
        }
    }
}

impl Move for InsertMove {
    type On = Tour;

    fn apply(&self, mut tour: Self::On) -> Self::On {
        match self.current_position.cmp(&self.target_position) {
            Ordering::Equal => {}
            Ordering::Less => {
                let current = tour[self.current_position];
                for p in self.current_position..self.target_position {
                    tour[p] = tour[p + 1];
                }
                tour[self.target_position] = current;
            }
            Ordering::Greater => {
                let current = tour[self.current_position];
                for p in (self.target_position..self.current_position).rev() {
                    tour[p + 1] = tour[p];
                }
                tour[self.target_position] = current;
            }
        }
        tour
    }
}

pub fn test_insert_move() {
    let tour = Tour::from((0..7).collect::<Vec<_>>());
    let mv = InsertMove {
        current_position: 2,
        target_position: 2,
    };
    let tour = mv.apply(tour);
    assert_eq!(tour, Tour::from((0..7).collect::<Vec<_>>()));

    let tour = Tour::from((0..7).collect::<Vec<_>>());
    let mv = InsertMove {
        current_position: 1,
        target_position: 4,
    };
    let tour = mv.apply(tour);
    assert_eq!(tour, Tour::from(vec![0, 2, 3, 4, 1, 5, 6]));

    let tour = Tour::from((0..7).collect::<Vec<_>>());
    let mv = InsertMove {
        current_position: 4,
        target_position: 1,
    };
    let tour = mv.apply(tour);
    assert_eq!(tour, Tour::from(vec![0, 4, 1, 2, 3, 5, 6]));
}
