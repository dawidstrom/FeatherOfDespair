use crate::entity::*;
use crate::utils::*;

pub trait Movable {
    fn move_dir(
        &mut self,
        steps:      i32,
        direction:  Direction,
        board:      &Board
    ) -> ();
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Board {
    pub size:       Size,
    pub scale:      i32,
    pub blocking_map: Vec<Entity>,
}

impl Movable for Entity {
    fn move_dir(
        &mut self,
        steps:      i32,
        direction:  Direction,
        board:      &Board
    ) -> () {
        let mut new_pos: Position;

        match direction {
            Direction::Up => {
                new_pos = Position {
                    x: self.pos.x,
                    y: std::cmp::max(0, self.pos.y-steps),
                };
            },
            Direction::Right => {
                new_pos = Position {
                    x: std::cmp::min(board.size.width-1, self.pos.x+steps),
                    y: self.pos.y,
                };

            },
            Direction::Down => {
                new_pos = Position {
                    x: self.pos.x,
                    y: std::cmp::min(board.size.height-1, self.pos.y+steps),
                };
            },
            Direction::Left => {
                new_pos = Position {
                    x: std::cmp::max(0, self.pos.x-steps),
                    y: self.pos.y,
                };
            },
        }

        for entity in board.blocking_map.iter() {
            if entity.blocking && entity.pos == new_pos {
                new_pos = self.pos;
            }
        }

        self.pos = new_pos;
    }
}
