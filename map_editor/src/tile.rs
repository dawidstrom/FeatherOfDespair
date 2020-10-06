extern crate num;
use num::FromPrimitive;

enum_from_primitive! {
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TileType {
    Wall = 0,
    Grass,
    Player,
}
}

impl TileType {
    pub fn from_u8(tile_type: u8) -> TileType {
        match FromPrimitive::from_u8(tile_type) {
            Some(next_tile) => next_tile,
            None => FromPrimitive::from_u8(0).unwrap(),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub blocking: bool,
}

impl Tile {
    pub fn next(self) -> Tile {
        let next_tile = (self.tile_type as usize + 1) % TILE_TYPES.len();
        TILE_TYPES[next_tile].clone()
    }
}

pub const WALL: Tile = Tile { tile_type: TileType::Wall, blocking: true, };
pub const GRASS: Tile = Tile { tile_type: TileType::Grass, blocking: false, };
pub const PLAYER: Tile = Tile { tile_type: TileType::Player, blocking: true, };

pub const TILE_TYPES: [&Tile; 3] = [&WALL, &GRASS, &PLAYER];
