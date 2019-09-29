use crate::{
    tile::{all_tiles, tile_from_string, Tile},
    types::{Call, CallType},
};

pub type Meld = Vec<Tile>;
impl Call {
    pub fn new(ctype: CallType, tile: &str) -> Call { Call { ctype, tile: tile_from_string(tile) } }

    pub fn chi(tile: Tile) -> Call { Call { ctype: CallType::Chi, tile } }

    pub fn pon(tile: Tile) -> Call { Call { ctype: CallType::Pon, tile } }

    pub fn minkan(tile: Tile) -> Call { Call { ctype: CallType::Minkan, tile } }

    pub fn ankan(tile: Tile) -> Call { Call { ctype: CallType::Ankan, tile } }

    pub fn meld(&self) -> Meld {
        match self.ctype {
            CallType::Chi => vec![self.tile, self.tile.next(), self.tile.next().next()],
            CallType::Pon => vec![self.tile, self.tile, self.tile],
            CallType::Minkan | CallType::Ankan => vec![self.tile, self.tile, self.tile, self.tile],
            _ => vec![],
        }
    }
}

// TODO: Could optimize by precomputing/hardcoding these (21 and 34)

fn all_sequences() -> Vec<Meld> {
    all_tiles()
        .into_iter()
        .filter(|t| t.is_number() && (t.number() != 8) && (t.number() != 9))
        .map(|t| vec![t, t.next(), t.next().next()])
        .collect()
}

fn all_triplets() -> Vec<Meld> { all_tiles().into_iter().map(|t| vec![t, t, t]).collect() }

pub fn possible_melds(tiles: &[Tile]) -> Vec<Meld> {
    let mut possible_melds = possible_sequences(tiles);
    possible_melds.append(&mut possible_triplets(tiles));
    possible_melds
}

fn possible_sequences(tiles: &[Tile]) -> Vec<Meld> {
    all_sequences().into_iter().filter(|ts| ts.iter().all(|t| tiles.contains(t))).collect()
}

fn possible_triplets(tiles: &[Tile]) -> Vec<Meld> {
    all_triplets()
        .into_iter()
        .filter(|triplet| tiles.iter().filter(|&tile| tile == &triplet[0]).count() >= 3)
        .collect()
}

pub fn is_triplet(tiles: &[Tile]) -> bool {
    tiles.len() == 3 && tiles.iter().all(|t| *t == tiles[0])
}

pub fn is_quadruplet(tiles: &[Tile]) -> bool {
    tiles.len() == 4 && tiles.iter().all(|t| *t == tiles[0])
}

pub fn is_sequence(tiles: &[Tile]) -> bool {
    tiles.len() == 3
        && tiles[0].is_number()
        && tiles[0].number() <= 7
        && tiles[0].next() == tiles[1]
        && tiles[1].next() == tiles[2]
}

pub fn is_partial_meld(tiles: &[Tile]) -> bool {
    is_ryanmen(&tiles) || is_penchan(&tiles) || is_kanchan(&tiles) || is_shanpon(&tiles)
}

pub fn is_ryanmen(tiles: &[Tile]) -> bool {
    tiles.len() == 2
        && tiles[0].is_number()
        && [1, 8, 9].iter().all(|&n| tiles[0].number() != n)
        && tiles[1] == tiles[0].next()
}

pub fn is_penchan(tiles: &[Tile]) -> bool {
    tiles.len() == 2
        && tiles[0].is_number()
        && [1, 8].iter().any(|&n| tiles[0].number() == n)
        && tiles[1] == tiles[0].next()
}

pub fn is_kanchan(tiles: &[Tile]) -> bool {
    tiles.len() == 2
        && tiles[0].is_number()
        && [8, 9].iter().all(|&n| tiles[0].number() != n)
        && tiles[1] == tiles[0].next().next()
}

pub fn is_shanpon(tiles: &[Tile]) -> bool { tiles.len() == 2 && tiles[0] == tiles[1] }

pub fn ryanmen_waits(tiles: &Meld) -> Vec<Tile> { vec![tiles[0].prev(), tiles[1].next()] }

pub fn penchan_waits(tiles: &Meld) -> Vec<Tile> {
    match tiles[0].number() {
        1 => vec![tiles[1].next()],
        8 => vec![tiles[0].prev()],
        _ => panic!("Not penchan"),
    }
}

pub fn kanchan_waits(tiles: &Meld) -> Vec<Tile> { vec![tiles[0].next()] }

pub fn shanpon_waits(tiles: &Meld) -> Vec<Tile> { vec![tiles[0]] }

pub fn no_open_calls(calls: &[Call]) -> bool { calls.iter().all(|c| c.ctype == CallType::Ankan) }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{tile_from_string, tiles_from_string};

    #[test]
    fn test_all_sequences() {
        assert_eq!(all_sequences().len(), 21);
    }
    #[test]
    fn test_all_triplets() {
        assert_eq!(all_triplets().len(), 34);
    }

    #[test]
    fn test_possible_melds() {
        let tiles = tiles_from_string("123466777999m");
        assert_eq!(
            possible_sequences(&tiles),
            [tiles_from_string("123m"), tiles_from_string("234m")]
        );
        assert_eq!(
            possible_triplets(&tiles),
            [tiles_from_string("777m"), tiles_from_string("999m")]
        );
    }

    #[test]
    fn test_no_open_calls() {
        assert!(!no_open_calls(&vec![
            Call { ctype: CallType::Minkan, tile: tile_from_string("1m") },
            Call { ctype: CallType::Ankan, tile: tile_from_string("2p") }
        ]));
        assert!(no_open_calls(&vec![Call {
            ctype: CallType::Ankan,
            tile: tile_from_string("3s")
        }]));
    }
}
