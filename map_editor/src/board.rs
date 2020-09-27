use crate::entity::*;
use crate::utils::*;
use crate::tile::*;

use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    io::{self, Read},
};

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
                blocking: blocking != 0,
                tile: Tile::from_u8(tile_type),
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

    pub fn write(&self, mut writer: impl byteorder::WriteBytesExt) {
        // Board size.
        if let Err(e) = writer.write_i32::<LittleEndian>(self.size.width) {
            println!("Failed to write width, {:?}.", e);
            return;
        }
        if let Err(e) = writer.write_i32::<LittleEndian>(self.size.height) {
            println!("Failed to write height, {:?}.", e);
            return;
        }
        
        // Board scale factor.
        if let Err(e) = writer.write_i32::<LittleEndian>(self.scale) {
            println!("Failed to write scale, {:?}.", e);
            return;
        }

        // Entities.
        for entity in self.blocking_map.iter() {
            // Position.
            if let Err(e) = writer.write_i32::<LittleEndian>(entity.pos.x) {
                println!("Failed to write entity x-position. {:?}.", e);
                return;
            }
            if let Err(e) = writer.write_i32::<LittleEndian>(entity.pos.y) {
                println!("Failed to write entity y-position. {:?}.", e);
                return;
            }
            // Is blocking.
            if let Err(e) = writer.write_u8(entity.blocking as u8) {
                println!("Failed to write if entity is blocking. {:?}.", e);
                return;
            }
            // Tile type.
            if let Err(e) = writer.write_u8(entity.tile as u8) {
                println!("Failed to write entity tile type. {:?}.", e);
                return;
            }
        }
    }
}
