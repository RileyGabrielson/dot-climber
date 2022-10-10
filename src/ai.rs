use crate::board::{point::Point, Board};

pub mod naive_chooser;
pub mod random_chooser;
pub mod recursive_chooser;

pub trait ArtificialChooser {
    fn choose(&self, board: &Board) -> Option<Point>;
}
