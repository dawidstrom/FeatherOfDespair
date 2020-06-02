use crate::utils::Position;

pub struct Entity {
    pub pos: Position,
    pub blocking: bool,
}

pub struct Player {
    pub entity:     Entity,
    pub max_hp:     i32,
    pub current_hp: i32,
}
