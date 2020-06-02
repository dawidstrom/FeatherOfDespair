use crate::entity::*;
use crate::utils::*;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::File,
    io::{self, Read, Write},
};

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
