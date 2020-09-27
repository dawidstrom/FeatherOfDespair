use crate::utils::Position;
use crate::tile::Tile;

pub struct Entity {
    pub pos: Position,
    pub blocking: bool,
    pub tile: Tile,
}
