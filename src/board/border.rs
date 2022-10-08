use super::point::Point;

#[derive(Debug)]
pub struct Border {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Border {
    pub fn is_in_border(&self, point: &Point) -> bool {
        point.x <= self.right && point.x >= self.left && point.y <= self.top && point.y >= self.bottom
    }
}
