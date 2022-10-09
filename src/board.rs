pub mod border;
pub mod point;

use std::collections::HashMap;
pub use self::border::Border;
use self::point::{Point, PointRenderData, DiagonalConnection};

// test

#[derive(Debug)]
pub struct Board {
    pub border: Border,
    pub connections: HashMap<String, Vec<Point>>,
    pub cur_position: Point,
}

impl Board {
    pub fn new(border: Border) -> Board {
        Board {
            connections: HashMap::new(),
            cur_position: Point {
                x: &border.left + (&border.right - &border.left) / 2,
                y: &border.bottom + (&border.top - &border.bottom) / 2,
            },
            border,
        }
    }


    pub fn make_move(&mut self, new_point: Point) {
        let cur_position_key = Board::get_point_key(&self.cur_position);
        let new_point_key = Board::get_point_key(&new_point);

        match self.connections.get_mut(&cur_position_key) {
            Some(connections) => {
                connections.push(Point { ..new_point });

                match self.connections.get_mut(&new_point_key) {
                    Some(another_connection) => {another_connection.push(Point {..self.cur_position});},
                    None => {self.connections.insert(new_point_key, Vec::from([Point {..self.cur_position}]));},
                }
            },
            None => {
                self.connections.insert(cur_position_key, Vec::from([Point {..new_point}]));
                self.connections.insert(new_point_key, Vec::from([Point {..self.cur_position}]));
            }
        }

        self.cur_position = new_point;
    }

    pub fn get_move_options(&self) -> Vec<Point> {
        let possible = self.get_possible_moves();
        self.filter_valid_moves(possible)
    }

    pub fn get_point_key(point: &Point) -> String {
        let mut key = point.x.to_string();
        key.push_str("-");
        key.push_str(&point.y.to_string());
        key
    }

    pub fn to_string(&self) -> String {
        let mut board_characters: Vec<Vec<String>> = Vec::from([]);
        for _ in 0..2*self.border.top {
            board_characters.push(Vec::from([]));
        }

        for y in self.border.bottom .. self.border.top {
            for x in self.border.left .. self.border.right {
                let point = Point { x, y: self.border.top - y };
                let point_chars = point::get_point_characters(
                    PointRenderData { 
                        right: self.has_right_connection(&point), 
                        bottom: self.has_bottom_connection(&point), 
                        bottom_right: if self.has_bottom_right_connection(&point) {DiagonalConnection::SingleLeft} else {DiagonalConnection::None},
                        bottom_left: if self.has_bottom_left_connection(&point) {DiagonalConnection::SingleRight} else {DiagonalConnection::None},
                        is_current_position: self.cur_position.x == point.x && self.cur_position.y == point.y,
                    }
                );

                if self.cur_position.x == point.x && self.cur_position.y == point.y {
                    println!("{:?}", point_chars);
                    
                }

                let row_1 = board_characters.get_mut((y*2) as usize).expect("Board should be initialized with the correct size");
                row_1.push(point_chars[0][1].clone());
                row_1.push(point_chars[0][2].clone());

                let row_2 = board_characters.get_mut(((y*2)+1) as usize).expect("Board should be initialized with the correct size");
                if point_chars[1][0] != "  " {
                    row_2.pop();
                    row_2.push(point_chars[1][0].clone());
                }
                row_2.push(point_chars[1][1].clone());
                row_2.push(point_chars[1][2].clone());

            }
        }

        let mut board_string = String::from("");
        for row in board_characters {
            for character in row {
                board_string.push_str(&character);
            }
            board_string.push_str("\n");
        }
        board_string
    }

    fn has_right_connection(&self, point: &Point) -> bool {
        match self.connections.get(&Board::get_point_key(&point)) {
            Some(connections) => connections.contains(&Point { x: point.x + 1, y: point.y }),
            None => false,
        }
    }

    fn has_bottom_connection(&self, point: &Point) -> bool {
        match self.connections.get(&Board::get_point_key(&point)) {
            Some(connections) => connections.contains(&Point { x: point.x, y: point.y - 1, }),
            None => false,
        }
    }

    fn has_bottom_right_connection(&self, point: &Point) -> bool {
        match self.connections.get(&Board::get_point_key(&point)) {
            Some(connections) => connections.contains(&Point { y: point.y - 1, x: point.x + 1 }),
            None => false,
        }
    }

    fn has_bottom_left_connection(&self, point: &Point) -> bool {
        match self.connections.get(&Board::get_point_key(&point)) {
            Some(connections) => connections.contains(&Point { y: point.y - 1, x: point.x - 1 }),
            None => false,
        }
    }


    fn filter_valid_moves(&self, points: [Point; 8]) -> Vec<Point> {
        let mut valid_moves: Vec<Point> = Vec::from([]); 
        for point in points {
            if self.border.is_in_border(&point) && !self.is_connected_to_current_point(&point) {
                valid_moves.push(point);
            }
        }
        valid_moves
    }

    fn is_connected_to_current_point(&self, point: &Point) -> bool {
        let result = self.connections.get(&Board::get_point_key(&self.cur_position));
        match result {
            Some(connections) => connections.contains(point),
            None => false,
        }
    }

    fn get_possible_moves(&self) -> [Point; 8] {
        let x = self.cur_position.x;
        let y = self.cur_position.y;

        [
            Point { x: x-1, y: y-1},
            Point { x: x-1, y},
            Point { x: x-1, y: y+1},
            Point { x, y: y-1},
            Point { x, y: y+1},
            Point { x: x+1, y: y-1},
            Point { x: x+1, y},
            Point { x: x+1, y: y+1},
        ]
    }
}

