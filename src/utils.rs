use crate::{division::Division, tile::Tile, types::Call};

pub(crate) fn combine_melds(division: &Division, calls: &Vec<Call>) -> Vec<Vec<Tile>> {
    let mut melds = division.melds.clone();
    melds.append(&mut calls.iter().map(|call| call.meld()).collect());
    melds
}

pub(crate) fn combine_melds_with_pair(division: &Division, calls: &Vec<Call>) -> Vec<Vec<Tile>> {
    let mut melds = combine_melds(division, calls);
    melds.push(division.pair.clone());
    melds
}

pub(crate) fn flatten_tiles(division: &Division, calls: &Vec<Call>) -> Vec<Tile> {
    let mut tiles = division.pair.clone();
    tiles.append(&mut division.melds.iter().flatten().cloned().collect());
    tiles.append(&mut division.remaining.clone());
    tiles.append(&mut calls.iter().flat_map(|call| call.meld()).collect());
    tiles
}
