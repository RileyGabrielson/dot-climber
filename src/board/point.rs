#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}

impl Copy for Point {}

pub struct PointConnections {
    pub right: bool,
    pub bottom: bool,
    pub bottom_right: bool,
    pub is_current_position: bool,
}

pub fn get_point_characters(connections: PointConnections) -> [[String; 2]; 2]  {
    let point = if connections.is_current_position { "! " } else { "* " };
    let right = if connections.right { "--" } else { "  " };
    let bottom = if connections.bottom { "| " } else { "  " };
    let bottom_right = if connections.bottom_right { "\\ " } else { "  " };
    [
        [point.to_string(), right.to_string()],
        [bottom.to_string(), bottom_right.to_string()],
    ]
}
