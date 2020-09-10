use crate::entity::*;
use crate::utils::*;

use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    io::{self, Read},
};

pub trait Movable {
    fn move_dir(&mut self, steps: i32, direction: Direction, board: &Board);
}

impl Player {
    pub fn update(&mut self, board: &mut Board, elapsed: i64) {
        match self.entity.move_timer.as_mut() {
            Some(timer) => {
                if timer.remaining > 0 {
                    timer.remaining -= elapsed;
                }

                if let Some(dir) = self.entity.moving {
                    if timer.remaining <= 0 {
                        timer.remaining = timer.duration;
                        self.entity.move_dir(1, dir, board);
                    }
                }
            }
            None => {}
        }
    }
}

pub struct Board {
    pub size:           Size,
    pub scale:          i32,
    pub blocking_map:   Vec<Entity>,
}

impl Board {
    pub fn read_wall(reader: &mut impl Read) -> Option<Entity> {
        if let (
            Ok(x), 
            Ok(y), 
            Ok(blocking)
        ) = (
            reader.read_i32::<LittleEndian>(),
            reader.read_i32::<LittleEndian>(),
            reader.read_u8()
        )
        {
            return Some(Entity{
                pos: Position{ x,y },
                blocking: blocking != 0,
                moving: None,
                move_timer: None,
            })
        }
        None
    }

    pub fn load(reader: &mut impl Read) -> io::Result<Self> {
        // Board size.
        let width = reader.read_i32::<LittleEndian>().unwrap();
        let height = reader.read_i32::<LittleEndian>().unwrap();

        // Board scale factor.
        let scale = reader.read_i32::<LittleEndian>().unwrap();

        // Entities.
        let mut walls = Vec::<Entity>::new();
        while let Some(wall) = Board::read_wall(reader) {
            walls.push(wall);
        }

        Ok(Board {
            size: Size{ width, height },
            scale,
            blocking_map: walls,
        })
    }
}

impl Movable for Entity {
    fn move_dir(&mut self,
                steps: i32,
                direction: Direction,
                board: &Board) {

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
