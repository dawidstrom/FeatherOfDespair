use crate::geo::*;

pub struct Entity {
    pub pos:        Position,
    pub size:       Size,
    pub max_hp:     i32,
    pub current_hp: i32,
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub trait Damagable {
    fn dmg(entity: &mut Entity, dmg: i32) -> ();
    fn heal(entity: &mut Entity, heal: i32) -> ();
}

pub trait Movable {
    fn move_dir(
        &mut self,
        steps:      i32,
        direction:  Direction,
        board:      &Board
    ) -> ();
}

impl Movable for Entity {
    fn move_dir(
        &mut self,
        steps:      i32,
        direction:  Direction,
        board:      &Board
    ) -> () {
        match direction {
            Direction::Up => {
                self.pos.y = std::cmp::max(0, self.pos.y-steps);
            },
            Direction::Right => {
                self.pos.x = std::cmp::min(
                    board.size.width-1, self.pos.x+steps
                );
            },
            Direction::Down => {
                self.pos.y = std::cmp::min(
                    board.size.height-1, self.pos.y+steps
                );
            },
                Direction::Left => {
                self.pos.x = std::cmp::max(0, self.pos.x-steps);
            },
        }
    }
}

impl Damagable for Entity {
    fn dmg(entity: &mut Entity, dmg: i32) -> () {
        entity.current_hp = std::cmp::max(
            0, entity.current_hp-dmg
        );
    }
    fn heal(entity: &mut Entity, heal: i32) -> () {
        entity.current_hp = std::cmp::min(
            entity.max_hp, entity.current_hp+heal
        );
    }
}
