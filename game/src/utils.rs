#[derive(PartialEq,Clone,Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn square_distance(pos1: Position, pos2: Position) -> i32 {
        (pos1.x-pos2.x) * (pos1.x-pos2.x) + (pos1.y-pos2.y) * (pos1.y-pos2.y)
    }
}

pub struct Size {
    pub width:  i32,
    pub height: i32,
}

