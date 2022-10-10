use crate::board::{Board, point::Point};
use super::ArtificialChooser;

pub struct NaiveChooser {
    target: Point,
}

impl NaiveChooser {
    pub fn new(target: Point) -> NaiveChooser {
        NaiveChooser {
            target
        }
    }
}

impl ArtificialChooser for NaiveChooser {
    fn choose(&self, board: &Board) -> Option<Point> {
        let options = board.get_move_options();

        if options.len() > 0 {
            let mut best_index = 0;
            let mut best_distance = 0;
            for index in 0..options.len() {
                let option = options[index];
                let distance = self.target.distance_squared_from(&option);
                if distance < best_distance {
                    best_index = index;
                    best_distance = distance;
                }
            }
            Some(options[best_index])
        } else {
            None
        }
    }
}
