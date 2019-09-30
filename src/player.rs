use crate::{
    tile::Tile,
    types::{Call, CallType},
};
use itertools::Itertools;

#[derive(Default)]
pub struct Player {
    hand: Vec<Tile>,
    player_wind: Tile,
    discards: Vec<Tile>,
    calls: Vec<Call>,
    total_score: u32,
}

impl Player {
    pub fn draw(&mut self, tile: Tile) { self.hand.push(tile); }

    pub fn discard(&mut self, idx: usize) -> Tile { self.hand.remove(idx) }

    pub fn call(&mut self, call_type: CallType, called_tile: Tile, hand_tile_indices: Vec<usize>) {
        let mut meld = vec![called_tile];
        for i in hand_tile_indices.into_iter().sorted().rev() {
            meld.push(self.hand.remove(i));
        }
        // We want the first tile (by ordering) in the meld for the `Call` struct
        meld.sort();
        let call = Call { ctype: call_type, tile: meld[0] };
        self.calls.push(call);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{tile_from_string, tiles_from_string};

    #[test]
    fn test_call_chi() {
        let mut player = Player { hand: tiles_from_string("12345m"), ..Default::default() };
        player.call(CallType::Chi, tile_from_string("3m"), vec![1, 3]);
        assert_eq!(player.hand, tiles_from_string("135m"));
        assert_eq!(player.calls, vec![Call { ctype: CallType::Chi, tile: tile_from_string("2m") }]);
    }

    #[test]
    fn test_call_pon() {
        let mut player = Player { hand: tiles_from_string("12334m"), ..Default::default() };
        player.call(CallType::Pon, tile_from_string("3m"), vec![2, 3]);
        assert_eq!(player.hand, tiles_from_string("124m"));
        assert_eq!(player.calls, vec![Call { ctype: CallType::Pon, tile: tile_from_string("3m") }]);
    }
}
