use crate::board::{Board, point::Point};
use super::ArtificialChooser;

pub struct RecursiveChooser {
    target: Point,
}

impl RecursiveChooser {
    pub fn new(target: Point) -> RecursiveChooser {
        RecursiveChooser {
            target
        }
    }
}

impl ArtificialChooser for RecursiveChooser {
    fn choose(&self, _board: &Board) -> Option<Point> {
        None
    }
}

impl RecursiveChooser {
    fn get_best_distance(&self, board: &Board, point: &Point) -> i32 {
        3 
    }
}
