use crate::{
    meld::{
        is_kanchan, is_partial_meld, is_penchan, is_ryanmen, is_shanpon, kanchan_waits,
        penchan_waits, possible_melds, ryanmen_waits, shanpon_waits, Meld,
    },
    tile::{has_tiles, with_tiles_removed, Tile},
};
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Division {
    pub(crate) pair: Vec<Tile>,
    pub(crate) melds: Vec<Meld>,
    pub(crate) remaining: Vec<Tile>,
}

impl Division {
    pub(crate) fn waits(&self) -> Vec<Tile> {
        let mut waits = vec![];

        if self.pair.len() == 0 && self.remaining.len() == 1 {
            // Tanki
            waits.append(&mut self.remaining.clone());
        } else if self.pair.len() == 2 && self.remaining.len() == 2 {
            let tiles = vec![self.remaining[0].clone(), self.remaining[1].clone()];
            if is_ryanmen(&self.remaining) {
                waits.append(&mut ryanmen_waits(&tiles));
            } else if is_penchan(&self.remaining) {
                waits.append(&mut penchan_waits(&tiles));
            } else if is_kanchan(&self.remaining) {
                waits.append(&mut kanchan_waits(&tiles));
            } else if is_shanpon(&self.remaining) {
                waits.append(&mut shanpon_waits(&tiles));
            }
        }

        // Reject impossible waits (not enough tiles)
        let tiles = self.tiles();
        waits.into_iter().filter(|w| tiles.iter().filter(|t| *t == w).count() < 4).collect()
    }

    pub(crate) fn is_tenpai(&self, n_calls: usize) -> bool {
        match self.pair.len() {
            // Tanki
            0 => (self.melds.len() + n_calls) == 4 && self.remaining.len() == 1,
            // Not tanki
            2 => (self.melds.len() + n_calls) == 3 && is_partial_meld(&self.remaining),
            _ => false,
        }
    }

    pub(crate) fn wins_on(&self, winning_tile: &Tile) -> bool {
        self.waits().contains(winning_tile)
    }

    fn tiles(&self) -> Vec<Tile> {
        let mut tiles = self.pair.clone();
        tiles.append(&mut self.melds.iter().flatten().cloned().collect());
        tiles.append(&mut self.remaining.clone());
        tiles
    }
}

pub(crate) fn divide(tiles: &Vec<Tile>) -> Vec<Division> {
    let mut results: Vec<Division> = Vec::new();

    // Get all candidate pairs (including no pair for tanki)
    let mut pairs: Vec<Vec<Tile>> = vec![vec![]];
    for tile in tiles.iter().sorted().dedup() {
        if tiles.iter().filter(|&t| t == tile).count() >= 2 {
            pairs.push(vec![tile.clone(), tile.clone()]);
        }
    }
    for pair in pairs {
        results.append(&mut divisions_for_pair(&pair, &tiles));
    }
    results
}

fn divisions_for_pair(pair: &Vec<Tile>, tiles: &[Tile]) -> Vec<Division> {
    let mut results = vec![];
    // Start by removing the pair
    let remaining = with_tiles_removed(&tiles, &pair.to_vec());

    // If there aren't enough tiles to form melds, just return immediately
    if remaining.len() < 3 {
        results.push(Division { pair: pair.to_vec(), melds: vec![], remaining });
        return results;
    }

    // Generate all possible combinations of melds that can be formed with the remaining tiles
    let n_melds = (remaining.len() / 3) as u8;
    // We want combinations with replacement since a meld may appear more than once
    let combinations = combis_with_rep(n_melds, &possible_melds(&remaining));

    for mut combi in combinations {
        let combi_tiles: Vec<Tile> = combi.iter().map(|m| m.to_vec()).flatten().collect();
        if !has_tiles(&remaining, &combi_tiles) {
            continue;
        }
        let combi_remaining = with_tiles_removed(&remaining, &combi_tiles);
        if pair.len() == 0 && combi_remaining.len() == 2 {
            continue;
        }
        combi.sort();
        results.push(Division { pair: pair.to_vec(), melds: combi, remaining: combi_remaining });
    }
    results
}

// Combinations with replacement
fn combis_with_rep(n: u8, melds: &[Meld]) -> Vec<Vec<Meld>> {
    if melds.len() == 0 {
        return vec![];
    }
    if n == 0 {
        return vec![vec![]];
    }
    let mut result: Vec<Vec<Meld>> = combis_with_rep(n, &melds[1..]);
    for mut subresult in combis_with_rep(n - 1, &melds) {
        subresult.push(melds[0].clone());
        result.push(subresult);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::tiles_from_string;

    #[test]
    fn test_divide_nocalls() {
        assert_eq!(
            divide(&tiles_from_string("4555m123789s333z")),
            vec![
                d("", &["555m", "123s", "789s", "333z"], "4m"),
                d("55m", &["123s", "789s", "333z"], "45m"),
                d("33z", &["555m", "123s", "789s"], "4m3z"),
            ]
        );
    }

    #[test]
    fn test_divide_nocalls_5sidedwait() {
        assert_eq!(
            divide(&tiles_from_string("1114444555566m")),
            vec![
                d("", &["111m", "444m", "456m", "555m"], "6m"),
                d("11m", &["444m", "456m", "555m"], "16m"),
                d("44m", &["111m", "456m", "555m"], "46m"),
                d("44m", &["111m", "456m", "456m"], "55m"),
                d("55m", &["111m", "444m", "456m"], "56m"),
                d("55m", &["111m", "456m", "456m"], "44m"),
                d("66m", &["111m", "444m", "555m"], "45m")
            ]
        );
    }

    #[test]
    fn test_waits_1sided() {
        // Kanchan
        _test_waits("234m13999p567s11z", "2p");
    }

    #[test]
    fn test_waits_2sided() {
        // Nobetan
        _test_waits("1234m123789s333z", "14m");

        _test_waits("3555m123456789s", "34m");
        _test_waits("222p123s111555z4p", "34p");
    }

    #[test]
    fn test_waits_3sided() {
        _test_waits("4555m123789s333z", "346m");
        _test_waits("4555m123456789s", "346m");

        // Sanmentan
        _test_waits("23456m11p123456s", "147m");
        _test_waits("1234567m123456s", "147m");

        // Kantankan
        _test_waits("3335777s111222z", "456s");

        // A weird one
        _test_waits("22223333444s55z", "14s5z");
    }

    #[test]
    fn test_waits_4sided() {
        _test_waits("1222234456678s", "1347s");
        _test_waits("1123333445566p", "1247p");
        _test_waits("2333445556777m", "1345m");
        _test_waits("44456s66678p111z", "69p47s");
        _test_waits("7788999s123456p", "6789s");
    }

    #[test]
    fn test_waits_5sided() {
        _test_waits("4445678m456789s", "35689m");

        // Tatsumaki
        _test_waits("3334555s111222z", "23456s");
    }

    #[test]
    fn test_waits_6sided() {
        // 3-sou is invalid since there are already 4 of them in the hand
        _test_waits("1233334555678s", "124569s");
    }

    #[test]
    fn test_waits_8sided() {
        // Happoubijin
        _test_waits("1113334567888m", "23456789m");
        _test_waits("2223456777s111z", "12345678s");

        // 6-man is invalid since there are already 4 of them in the hand
        _test_waits("1112345666678m", "12345789m");
        // 4-man is invalid since there are already 4 of them in the hand
        _test_waits("2344445678999m", "12356789m");
    }

    #[test]
    fn test_waits_iipeikou() {
        _test_waits("22334455s11122z", "25s2z");
        _test_waits("2233445566s123p", "2356s");
    }

    #[test]
    fn test_combis_with_rep() {
        let melds = vec![
            tiles_from_string("123m"),
            tiles_from_string("456m"),
            tiles_from_string("789m"),
            tiles_from_string("123p"),
            tiles_from_string("456p"),
            tiles_from_string("789p"),
            tiles_from_string("123s"),
            tiles_from_string("456s"),
            tiles_from_string("789s"),
        ];
        assert_eq!(combis_with_rep(3, &melds).len(), 165);
    }

    fn _test_waits(tiles_str: &str, expected_waits_str: &str) {
        let mut waits: Vec<Tile> =
            divide(&tiles_from_string(tiles_str)).iter().flat_map(|d| d.waits()).collect();
        waits.sort();
        waits.dedup();

        assert_eq!(waits, tiles_from_string(expected_waits_str));
    }

    fn d(pair: &str, melds: &[&str], remaining: &str) -> Division {
        Division {
            pair: tiles_from_string(pair),
            melds: melds.into_iter().map(|m| tiles_from_string(m)).collect(),
            remaining: tiles_from_string(remaining),
        }
    }
}
