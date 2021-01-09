use crate::utils::Position;
use crate::tile::Tile;

#[derive(PartialEq,Clone,Copy)]
pub struct Direction {
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
}

impl Direction {
    pub fn is_moving(self) -> bool {
        self.up || self.right || self.down || self.left
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction{
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
}

pub struct Entity {
    pub pos:                    Position,
    pub tile_type:              Tile,
    pub is_movement_blocking:   bool,
    pub is_vision_blocking:     bool,
    pub moving:                 Direction,
    pub move_timer:             Option<Timer>,
}

pub struct Timer {
    pub remaining:  i64,
    pub duration:   i64,
    pub looping:    bool,
}

pub struct Player {
    pub entity:     Entity,
    pub max_hp:     i32,
    pub current_hp: i32,
}
