use crate::entity::*;
use crate::utils::*;
use crate::tile::*;

use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    io::{self, Read},
};

pub trait Movable {
    fn move_dir(&mut self, board: &Board);
}

impl Player {
    pub fn update(&mut self, board: &mut Board, elapsed: i64) {
        match self.entity.move_timer.as_mut() {
            Some(timer) => {
                if timer.remaining > 0 {
                    timer.remaining -= elapsed;
                }

                if self.entity.moving.is_moving() {
                    if timer.remaining <= 0 {
                        timer.remaining = timer.duration;
                        self.entity.move_dir(board);
                    }
                }
            }
            None => {}
        }
    }
}

pub struct Board {
    pub size:           Rect,
    pub scale:          i32,
    pub blocking_map:   Vec<Entity>,
}

impl Board {
    pub fn read_wall(reader: &mut impl Read) -> Option<Entity> {
        if let (
            Ok(x), 
            Ok(y), 
            Ok(blocking),
            Ok(tile_type),
        ) = (
            reader.read_i32::<LittleEndian>(),
            reader.read_i32::<LittleEndian>(),
            reader.read_u8(),
            reader.read_u8(),
        )
        {
            return Some(Entity{
                pos: Position{ x,y },
                tile_type: Tile::from_u8(tile_type),
                blocking: blocking != 0,
                moving: Direction::default(),
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
        let mut tiles = Vec::<Entity>::new();
        while let Some(tile) = Board::read_wall(reader) {
            tiles.push(tile);
        }

        Ok(Board {
            size: Rect{ width, height },
            scale,
            blocking_map: tiles,
        })
    }
}

impl Movable for Entity {
    fn move_dir(&mut self,
                board: &Board) {

        let &dir = &self.moving;
        let x = dir.right as i32 - dir.left as i32;
        let y = dir.down as i32 - dir.up as i32;

        let mut new_pos: Position;

        use std::cmp::{min,max};
        new_pos = Position {
            x: max(0,min(self.pos.x + x, board.size.width-1)),
            y: max(0,min(self.pos.y + y, board.size.height-1)),
        };

        for entity in board.blocking_map.iter() {
            if entity.blocking && entity.pos == new_pos {
                new_pos = self.pos;
            }
        }

        self.pos = new_pos;
    }
}
