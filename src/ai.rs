use crate::board::{point::Point, Board};

pub mod random_chooser;

pub trait ArtificialChooser {
    fn choose(&self, board: &Board) -> Option<Point>;
}
