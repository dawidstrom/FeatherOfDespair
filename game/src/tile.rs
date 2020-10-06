extern crate num;
use num::FromPrimitive;

enum_from_primitive! {
#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Wall = 0,
    Grass,
    Player,
}
}

impl Tile {
    pub fn from_u8(tile_type: u8) -> Tile {
        match FromPrimitive::from_u8(tile_type) {
            Some(next_tile) => next_tile,
            None => FromPrimitive::from_u8(0).unwrap(),
        }
    }
}
