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

pub enum DiagonalConnection {
    Crossed,
    SingleLeft,
    SingleRight,
    None,
}

impl DiagonalConnection {
    fn to_string(&self) -> &str {
       match self {
            DiagonalConnection::None => "  ",
            DiagonalConnection::Crossed => "X ",
            DiagonalConnection::SingleLeft => "\\ ",
            DiagonalConnection::SingleRight => "/ ",
       }
    }
}

pub struct PointRenderData {
    pub right: bool,
    pub bottom: bool,
    pub bottom_right: DiagonalConnection,
    pub bottom_left: DiagonalConnection,
    pub is_current_position: bool,
}

pub fn get_point_characters(connections: PointRenderData) -> [[String; 3]; 2]  {
    let point = if connections.is_current_position { "! " } else { "* " };
    let right = if connections.right { "- " } else { "  " };
    let bottom = if connections.bottom { "| " } else { "  " };
    let bottom_right = connections.bottom_right.to_string();
    let bottom_left = connections.bottom_left.to_string();
    [
        [String::from("  "), point.to_string(), right.to_string()],
        [bottom_left.to_string(), bottom.to_string(), bottom_right.to_string()],
    ]
}

