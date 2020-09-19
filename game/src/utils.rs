#[derive(PartialEq,Clone,Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance(pos1: Position, pos2: Position) -> f64 {
        (((pos1.x - pos2.x) as f64).powi(2) + 
         ((pos1.y - pos2.y) as f64).powi(2)).sqrt()
    }
}

pub struct Rect {
    pub width:  i32,
    pub height: i32,
}

