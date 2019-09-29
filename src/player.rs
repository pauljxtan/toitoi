use crate::{
    tile::Tile,
    types::{Call, HandContext},
};

pub struct Player {
    hand: Vec<Tile>,
    hand_context: HandContext,
    discards: Vec<Tile>,
    calls: Vec<Call>,
    score: u32,
}

impl Player {
    pub fn draw(&mut self, tile: Tile) {
        self.hand.push(tile);
    }

    pub fn discard(&mut self, idx: usize) -> Tile {
        self.hand.remove(idx)
    }

    pub fn call(&mut self) {}
}
