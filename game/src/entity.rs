use crate::utils::Position;

#[derive(PartialEq,Clone,Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Entity {
    pub pos: Position,
    pub blocking: bool,
    pub moving: Option<Direction>,
    pub move_timer: Option<Timer>,
}

pub struct Timer {
    pub remaining: i64,
    pub duration: i64,
    pub looping: bool,
}

pub struct Player {
    pub entity:     Entity,
    pub max_hp:     i32,
    pub current_hp: i32,
}
