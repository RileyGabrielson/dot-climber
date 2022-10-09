use crate::board::{Board, point::Point};
use super::ArtificialChooser;
use rand::distributions::{Uniform, Distribution};

pub struct RandomChooser {}

impl ArtificialChooser for RandomChooser {
    fn choose(&self, board: &Board) -> Option<Point> {
        let options = board.get_move_options();
        if options.len() > 0 {
            let mut rng = rand::thread_rng();
            let index = Uniform::from(0..options.len()).sample(&mut rng);
            Some(options[index])
        } else {
            None
        }
    }
}
