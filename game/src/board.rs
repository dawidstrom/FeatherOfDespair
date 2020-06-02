use crate::entity::*;
use crate::utils::*;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::File,
    io::{self, Read, Write},
};

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

pub enum Tile {
    WALL = 1,
}

pub struct Board {
    pub size:           Size,
    pub scale:          i32,
    pub blocking_map:   Vec<Entity>,
}

impl Board {
    pub fn readWall(reader: &mut impl Read) -> Option<Entity> {
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
        while let Some(wall) = Board::readWall(reader) {
            walls.push(wall);
        }

        Ok(Board {
            size: Size{ width, height },
            scale,
            blocking_map: walls,
        })
    }

    pub fn write(&self, mut writer: impl byteorder::WriteBytesExt) {
        // Board size.
        writer.write_i32::<LittleEndian>(self.size.width);
        writer.write_i32::<LittleEndian>(self.size.height);

        // Board scale factor.
        writer.write_i32::<LittleEndian>(self.scale);

        // Entities.
        for wall in self.blocking_map.iter() {
            // Position.
            writer.write_i32::<LittleEndian>(wall.pos.x);
            writer.write_i32::<LittleEndian>(wall.pos.y);
            // Is blocking.
            writer.write_u8(wall.blocking as u8);
        }
    }
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
