//! Provides functions for scoring yaku and yakuman.

use crate::{
    division::Division,
    meld::{is_quadruplet, is_sequence, is_triplet, no_open_calls, Meld},
    tile::{tile_from_string, tiles_from_string, Tile},
    types::{Call, CallType, HandContext, Yaku, Yakuman},
    utils::{combine_melds, combine_melds_with_pair, flatten_tiles},
};
use itertools::Itertools;
use std::{collections::HashSet, iter::FromIterator};

type CheckFunc = fn(&Division, &Vec<Call>, &HandContext) -> bool;

pub(crate) struct YakuInfo<T> {
    han_closed: u8,
    han_open: u8,
    check_func: CheckFunc,
    supercedes: Vec<T>,
}

// Excluding pinfu and doras
const YAKU_TO_CHECK: [Yaku; 31] = [
    Yaku::MenzenTsumo,
    Yaku::Riichi,
    Yaku::Ippatsu,
    Yaku::Iipeikou,
    Yaku::HaiteiRaoyue,
    Yaku::HouteiRaoyui,
    Yaku::RinshanKaihou,
    Yaku::Chankan,
    Yaku::Tanyao,
    Yaku::Ton,
    Yaku::Nan,
    Yaku::Sha,
    Yaku::Pei,
    Yaku::Haku,
    Yaku::Hatsu,
    Yaku::Chun,
    Yaku::DoubleRiichi,
    Yaku::Chantaiyao,
    Yaku::SanshokuDoujun,
    Yaku::Sankantsu,
    Yaku::Ittsu,
    Yaku::Toitoi,
    Yaku::Sanankou,
    Yaku::SanshokuDoukou,
    Yaku::Chiitoitsu,
    Yaku::Honroutou,
    Yaku::Shousangen,
    Yaku::Ryanpeikou,
    Yaku::Honitsu,
    Yaku::JunchanTaiyao,
    Yaku::Chinitsu,
];

// Excluding kazoe yakuman, nagashi mangan
const YAKUMAN_TO_CHECK: [Yakuman; 12] = [
    Yakuman::KokushiMusou,
    Yakuman::Suuankou,
    Yakuman::Daisangen,
    Yakuman::Shousuushii,
    Yakuman::Daisuushii,
    Yakuman::Tsuuiisou,
    Yakuman::Chinroutou,
    Yakuman::Ryuuiisou,
    Yakuman::ChuurenPoutou,
    Yakuman::Suukantsu,
    Yakuman::Tenhou,
    Yakuman::Chiihou,
];

impl Yaku {
    pub(crate) fn han_closed(&self) -> u8 { self.info().han_closed }

    pub(crate) fn han_open(&self) -> u8 { self.info().han_open }

    pub(crate) fn is_wind(&self) -> bool {
        *self == Yaku::Ton || *self == Yaku::Nan || *self == Yaku::Sha || *self == Yaku::Pei
    }

    fn make_info(
        han_closed: u8, han_open: u8, check_func: CheckFunc, supercedes: Vec<Yaku>,
    ) -> YakuInfo<Yaku> {
        YakuInfo::<Yaku> { han_closed, han_open, check_func, supercedes }
    }
}

impl Yakuman {
    pub(crate) fn han_closed(&self) -> u8 { self.info().han_closed }

    pub(crate) fn han_open(&self) -> u8 { self.info().han_open }

    fn make_info(
        han_closed: u8, han_open: u8, check_func: CheckFunc, supercedes: Vec<Yakuman>,
    ) -> YakuInfo<Yakuman> {
        YakuInfo::<Yakuman> { han_closed, han_open, check_func, supercedes }
    }
}

pub(crate) trait Checkable<T> {
    fn info(&self) -> YakuInfo<T>;
    fn check(&self, division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool;
}

impl Checkable<Yaku> for Yaku {
    fn info(&self) -> YakuInfo<Yaku> {
        match self {
            Yaku::MenzenTsumo => Yaku::make_info(1, 0, has_menzen_tsumo, vec![]),
            Yaku::Riichi => Yaku::make_info(1, 0, has_riichi, vec![]),
            Yaku::Ippatsu => Yaku::make_info(1, 0, has_ippatsu, vec![]),
            Yaku::Pinfu => Yaku::make_info(1, 0, placeholder, vec![]),
            Yaku::Iipeikou => Yaku::make_info(1, 0, has_iipeikou, vec![]),
            Yaku::HaiteiRaoyue => Yaku::make_info(1, 1, has_haitei, vec![]),
            Yaku::HouteiRaoyui => Yaku::make_info(1, 1, has_houtei, vec![]),
            Yaku::RinshanKaihou => Yaku::make_info(1, 1, has_rinshan, vec![]),
            Yaku::Chankan => Yaku::make_info(1, 1, has_chankan, vec![]),
            Yaku::Tanyao => Yaku::make_info(1, 1, has_tanyao, vec![]),
            Yaku::Ton => Yaku::make_info(1, 1, has_ton, vec![]),
            Yaku::Nan => Yaku::make_info(1, 1, has_nan, vec![]),
            Yaku::Sha => Yaku::make_info(1, 1, has_sha, vec![]),
            Yaku::Pei => Yaku::make_info(1, 1, has_pei, vec![]),
            Yaku::Haku => Yaku::make_info(1, 1, has_haku, vec![]),
            Yaku::Hatsu => Yaku::make_info(1, 1, has_hatsu, vec![]),
            Yaku::Chun => Yaku::make_info(1, 1, has_chun, vec![]),
            Yaku::DoubleRiichi => Yaku::make_info(2, 0, has_double_riichi, vec![]),
            Yaku::Chantaiyao => Yaku::make_info(2, 1, has_chanta, vec![]),
            Yaku::SanshokuDoujun => Yaku::make_info(2, 1, has_sanshoku_doujun, vec![]),
            Yaku::Ittsu => Yaku::make_info(2, 1, has_ittsu, vec![]),
            Yaku::Toitoi => Yaku::make_info(2, 2, has_toitoi, vec![]),
            Yaku::Sanankou => Yaku::make_info(2, 2, has_sanankou, vec![]),
            Yaku::SanshokuDoukou => Yaku::make_info(2, 2, has_sanshoku_doukou, vec![]),
            Yaku::Chiitoitsu => Yaku::make_info(2, 0, has_chiitoi, vec![Yaku::Toitoi]),
            Yaku::Sankantsu => Yaku::make_info(2, 2, has_sankantsu, vec![]),
            Yaku::Honroutou => Yaku::make_info(2, 2, has_honroutou, vec![Yaku::Chantaiyao]),
            Yaku::Shousangen => Yaku::make_info(2, 2, has_shousangen, vec![]),
            Yaku::Ryanpeikou => Yaku::make_info(3, 0, has_ryanpeikou, vec![Yaku::Iipeikou]),
            Yaku::Honitsu => Yaku::make_info(3, 2, has_honitsu, vec![]),
            Yaku::JunchanTaiyao => Yaku::make_info(
                3,
                2,
                has_junchan,
                vec![Yaku::Honroutou, Yaku::Chantaiyao, Yaku::Toitoi],
            ),
            Yaku::Chinitsu => {
                Yaku::make_info(6, 5, has_chinitsu, vec![Yaku::Honitsu, Yaku::JunchanTaiyao])
            }
            _ => panic!("Should get yaku info from elsewhere"),
        }
    }

    fn check(&self, division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
        (self.info().check_func)(division, calls, context)
    }
}

impl Checkable<Yakuman> for Yakuman {
    fn info(&self) -> YakuInfo<Yakuman> {
        match self {
            Yakuman::KokushiMusou => Yakuman::make_info(13, 13, has_kokushi, vec![]),
            Yakuman::Suuankou => Yakuman::make_info(13, 13, has_suuankou, vec![]),
            Yakuman::Daisangen => Yakuman::make_info(13, 13, has_daisangen, vec![]),
            Yakuman::Shousuushii => Yakuman::make_info(13, 13, has_shousuushii, vec![]),
            Yakuman::Daisuushii => {
                Yakuman::make_info(13, 13, has_daisuushii, vec![Yakuman::Shousuushii])
            }
            Yakuman::Tsuuiisou => Yakuman::make_info(13, 13, has_tsuuiisou, vec![]),
            Yakuman::Chinroutou => Yakuman::make_info(13, 13, has_chinroutou, vec![]),
            Yakuman::Ryuuiisou => Yakuman::make_info(13, 13, has_ryuuiisou, vec![]),
            Yakuman::ChuurenPoutou => Yakuman::make_info(13, 13, has_chuuren, vec![]),
            Yakuman::Suukantsu => Yakuman::make_info(13, 13, has_suukantsu, vec![]),
            Yakuman::Tenhou => Yakuman::make_info(13, 13, has_tenhou, vec![]),
            Yakuman::Chiihou => Yakuman::make_info(13, 13, has_chiihou, vec![]),
            _ => panic!("Should get yakuman info from elsewhere"),
        }
    }

    fn check(&self, division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
        (self.info().check_func)(division, calls, context)
    }
}

/// Finds all yaku in the given hand.
pub(crate) fn yaku_in_hand(
    division: &Division, calls: &Vec<Call>, context: &HandContext,
) -> Vec<Yaku> {
    _find_in_hand(&YAKU_TO_CHECK, division, calls, context)
}

/// Finds all yakuman in the given hand.
pub(crate) fn yakuman_in_hand(
    division: &Division, calls: &Vec<Call>, context: &HandContext,
) -> Vec<Yakuman> {
    _find_in_hand(&YAKUMAN_TO_CHECK, division, calls, context)
}

fn _find_in_hand<T: Checkable<T> + Clone + PartialEq>(
    to_check: &[T], division: &Division, calls: &Vec<Call>, context: &HandContext,
) -> Vec<T> {
    let found: Vec<T> =
        to_check.iter().filter(|&y| y.check(division, calls, context)).cloned().collect();
    let superceded: Vec<T> = found.iter().flat_map(|y| y.info().supercedes).collect();
    found.into_iter().filter(|y| !superceded.contains(y)).collect()
}

// Context-dependent (composition-independent) yaku

fn has_menzen_tsumo(_division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    no_open_calls(calls) && context.is_tsumo
}

fn has_riichi(_division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    no_open_calls(calls) && context.is_riichi
}

fn has_ippatsu(_division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    no_open_calls(calls) && context.is_ippatsu
}

fn has_haitei(_division: &Division, _calls: &Vec<Call>, context: &HandContext) -> bool {
    context.is_haitei
}

fn has_houtei(_division: &Division, _calls: &Vec<Call>, context: &HandContext) -> bool {
    context.is_houtei
}

fn has_rinshan(_division: &Division, _calls: &Vec<Call>, context: &HandContext) -> bool {
    context.is_rinshan
}

fn has_chankan(_division: &Division, _calls: &Vec<Call>, context: &HandContext) -> bool {
    context.is_chankan
}

fn has_double_riichi(_division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    no_open_calls(calls) && context.is_double_riichi
}

fn has_tenhou(_division: &Division, _calls: &Vec<Call>, context: &HandContext) -> bool {
    context.is_tenhou
}

fn has_chiihou(_division: &Division, _calls: &Vec<Call>, context: &HandContext) -> bool {
    context.is_chiihou
}

// Context-independent (composition-dependent) yaku

fn has_iipeikou(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    let melds = combine_melds(division, calls);
    no_open_calls(calls)
        && melds
            .iter()
            .dedup()
            .any(|m| is_sequence(m) && melds.iter().filter(|&mm| mm == m).count() >= 2)
}

fn has_tanyao(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    // TODO: Check if open tanyao allowed
    flatten_tiles(division, calls).iter().all(|t| t.is_simple())
}

fn has_ton(division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    has_wind(tile_from_string("1z"), division, calls, context)
}

fn has_nan(division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    has_wind(tile_from_string("2z"), division, calls, context)
}

fn has_sha(division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    has_wind(tile_from_string("3z"), division, calls, context)
}

fn has_pei(division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    has_wind(tile_from_string("4z"), division, calls, context)
}

fn has_wind(wind: Tile, division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    (wind == context.round_wind || wind == context.player_wind)
        && combine_melds(division, calls).iter().any(|m| m[0] == wind)
}

fn has_haku(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    has_colour(tile_from_string("5z"), division, calls)
}

fn has_hatsu(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    has_colour(tile_from_string("6z"), division, calls)
}

fn has_chun(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    has_colour(tile_from_string("7z"), division, calls)
}

fn has_colour(colour: Tile, division: &Division, calls: &Vec<Call>) -> bool {
    combine_melds(division, calls).iter().any(|m| m[0] == colour)
}

fn has_chanta(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    combine_melds_with_pair(division, calls)
        .iter()
        .all(|m| m.iter().any(|t| t.is_terminal() || t.is_honour()))
}

fn has_sanshoku_doujun(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    (1..8).any(|n| {
        ["m", "p", "s"].iter().all(|s| {
            combine_melds(division, calls)
                .iter()
                .filter(|&m| *m == tiles_from_string(&format!("{}{}{}{}", n, n + 1, n + 2, s)))
                .count()
                >= 1
        })
    })
}

fn has_sankantsu(_division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    kan_count(calls) == 3
}

fn has_ittsu(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    ["m", "p", "s"].iter().any(|s| {
        ["123", "456", "789"].iter().all(|n| {
            combine_melds(division, calls)
                .iter()
                .any(|m| *m == tiles_from_string(&format!("{}{}", n, s)))
        })
    })
}

fn has_toitoi(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    combine_melds(division, calls).iter().all(|meld| !is_sequence(meld))
}

fn has_sanankou(division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    let closed_quads: Vec<Meld> =
        calls.iter().filter(|c| c.ctype == CallType::Ankan).map(|c| c.meld()).collect();
    let closed_triplets: Vec<Meld> =
        division.melds.iter().filter(|m| is_triplet(m)).cloned().collect();
    let count = closed_quads.len() + closed_triplets.len();
    match count {
        0 | 1 | 2 => false,
        // Reject if final triplet/quad completed by ron
        3 => context.is_tsumo || !closed_triplets.iter().any(|m| *m == context.winning_meld),
        // Returns true for suuankou, which is not a problem
        _ => true,
    }
}

fn has_sanshoku_doukou(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    (1..10).any(|n| {
        ["m", "p", "s"].iter().all(|s| {
            combine_melds(division, calls)
                .iter()
                .filter(|&m| {
                    *m == tiles_from_string(&format!("{}{}{}{}", n, n, n, s))
                        || *m == tiles_from_string(&format!("{}{}{}{}{}", n, n, n, n, s))
                })
                .count()
                >= 1
        })
    })
}

fn has_chiitoi(division: &Division, _calls: &Vec<Call>, _context: &HandContext) -> bool {
    let tiles = &division.remaining;
    tiles.len() == 14 && tiles.iter().all(|t| tiles.iter().filter(|&tt| tt == t).count() == 2)
}

fn has_honroutou(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    flatten_tiles(division, calls).iter().all(|t| t.is_terminal() || t.is_honour())
}

fn has_shousangen(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    combine_melds(division, calls)
        .iter()
        .filter(|m| (is_triplet(m) || is_quadruplet(m)) && m[0].is_colour())
        .count()
        == 2
        && division.pair.len() >= 1
        && division.pair[0].is_colour()
}

fn has_ryanpeikou(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    let melds = combine_melds(division, calls);
    melds
        .iter()
        .filter(|m| is_sequence(m) && [2, 4].contains(&melds.iter().filter(|mm| mm == m).count()))
        .count()
        == 4
}

fn has_honitsu(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    let tiles = flatten_tiles(division, calls);
    tiles.iter().all(|t| t.is_honour() || t.is_man())
        || tiles.iter().all(|t| t.is_honour() || t.is_pin())
        || tiles.iter().all(|t| t.is_honour() || t.is_sou())
}

fn has_junchan(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    combine_melds_with_pair(division, calls).iter().all(|m| m.iter().any(|t| t.is_terminal()))
}

fn has_chinitsu(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    let tiles = flatten_tiles(division, calls);
    tiles.iter().all(|t| t.is_man())
        || tiles.iter().all(|t| t.is_pin())
        || tiles.iter().all(|t| t.is_sou())
}

fn has_kokushi(division: &Division, _calls: &Vec<Call>, _context: &HandContext) -> bool {
    // TODO: Check winning tile for 13-way wait
    let tiles = division.remaining.clone();
    let pair_candidates: Vec<Tile> = tiles
        .iter()
        .sorted()
        .dedup()
        .filter(|t| tiles.iter().filter(|tt| tt == t).count() == 2)
        .cloned()
        .collect();
    if pair_candidates.len() != 1 {
        return false;
    }
    // TODO: Precompute for efficiency
    let terminals_honours: HashSet<Tile> =
        HashSet::from_iter(tiles_from_string("19m19p19s1234567z"));
    HashSet::from_iter(tiles) == terminals_honours
        && terminals_honours.iter().any(|t| *t == pair_candidates[0])
}

fn has_suuankou(division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    let closed_quads: Vec<Meld> =
        calls.iter().filter(|c| c.ctype == CallType::Ankan).map(|c| c.meld()).collect();
    let closed_triplets: Vec<Meld> =
        division.melds.iter().filter(|m| is_triplet(m)).cloned().collect();
    let count = closed_quads.len() + closed_triplets.len();
    match count {
        // Reject if final triplet/quad completed by ron
        4 => context.is_tsumo || !closed_triplets.iter().any(|m| *m == context.winning_meld),
        _ => false,
    }
}

fn has_daisangen(division: &Division, calls: &Vec<Call>, context: &HandContext) -> bool {
    has_haku(division, calls, context)
        && has_hatsu(division, calls, context)
        && has_chun(division, calls, context)
}

fn has_shousuushii(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    combine_melds(division, calls).iter().filter(|m| m[0].is_wind()).count() == 3
        && division.pair[0].is_wind()
}

fn has_daisuushii(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    combine_melds(division, calls).iter().filter(|m| m[0].is_wind()).count() == 4
}

fn has_tsuuiisou(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    flatten_tiles(division, calls).iter().all(|t| t.is_honour())
}

fn has_chinroutou(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    flatten_tiles(division, calls).iter().all(|t| t.is_terminal())
}

fn has_ryuuiisou(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    // TODO: Pre-compute
    let green_tiles = tiles_from_string("23468s6z");
    flatten_tiles(division, calls).iter().all(|t| green_tiles.contains(t))
}

fn has_chuuren(division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    if !no_open_calls(calls) {
        return false;
    }
    let tiles: Vec<Tile> = flatten_tiles(division, calls).into_iter().sorted().collect();
    for suit in ["m", "p", "s"].iter() {
        let chuuren_base = tiles_from_string(&format!("1112345678999{}", suit));
        // Add any one tile of the same suit and compare
        for num in 1..10 {
            let mut candidate = chuuren_base.clone();
            candidate.push(tile_from_string(&format!("{}{}", num, suit)));
            candidate.sort();
            if tiles == candidate {
                return true;
            }
        }
    }
    false
}

fn has_suukantsu(_division: &Division, calls: &Vec<Call>, _context: &HandContext) -> bool {
    kan_count(calls) == 4
}

// Helpers

fn placeholder(_division: &Division, _calls: &Vec<Call>, _context: &HandContext) -> bool {
    panic!("This placeholder function should never be called");
}

fn kan_count(calls: &Vec<Call>) -> usize {
    calls.iter().filter(|c| [CallType::Minkan, CallType::Ankan].contains(&c.ctype)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::division::divide;

    #[test]
    fn menzen_tsumo() {
        let tiles = tiles_from_string("234456m66p12344s");
        let calls = vec![];

        // Closed, tsumo
        assert_any_division(has_menzen_tsumo, &tiles, &calls, &ctw("4s", true, "1z", "1z"));
        // Closed, ron
        refute_all_divisions(has_menzen_tsumo, &tiles, &calls, &ctw("4s", false, "1z", "1z"));

        let tiles_open = tiles_from_string("234m66p12344s");
        let calls_open = vec![c("c", "4m")];

        // Open, tsumo
        refute_all_divisions(
            has_menzen_tsumo,
            &tiles_open,
            &calls_open,
            &ctw("4s", true, "1z", "1z"),
        );
        // Open, false
        refute_all_divisions(
            has_menzen_tsumo,
            &tiles_open,
            &calls_open,
            &ctw("4s", false, "1z", "1z"),
        );
    }

    #[test]
    fn iipeikou() {
        let tiles = tiles_from_string("45566p22789s111z");
        let calls = vec![];

        assert_any_division(has_iipeikou, &tiles, &calls, &ctw("4p", true, "1z", "1z"));
        refute_all_divisions(has_iipeikou, &tiles, &calls, &ctw("7p", true, "1z", "1z"));
    }

    #[test]
    fn tanyao() {
        let tiles = tiles_from_string("23456m22p234567s");
        let calls = vec![];

        assert_any_division(has_tanyao, &tiles, &calls, &ctw("7m", true, "1z", "1z"));
        refute_all_divisions(has_tanyao, &tiles, &calls, &ctw("1m", true, "1z", "1z"));

        // TODO: Test on/off open tanyao when implemented
    }

    #[test]
    fn ton() {
        let tiles = tiles_from_string("56m22p234567s111z");
        let calls = vec![];

        // Round wind
        assert_any_division(has_ton, &tiles, &calls, &ctw("7m", true, "1z", "2z"));
        // Player wind
        assert_any_division(has_ton, &tiles, &calls, &ctw("7m", true, "2z", "1z"));
        // Guest wind
        refute_all_divisions(has_ton, &tiles, &calls, &ctw("7m", true, "2z", "2z"));
    }

    #[test]
    fn nan() {
        let tiles = tiles_from_string("56m22p234567s");
        let calls = vec![c("p", "2z")];

        // Round wind
        assert_any_division(has_nan, &tiles, &calls, &ctw("7m", true, "2z", "1z"));
        // Player wind
        assert_any_division(has_nan, &tiles, &calls, &ctw("7m", true, "1z", "2z"));
        // Guest wind
        refute_all_divisions(has_nan, &tiles, &calls, &ctw("7m", true, "1z", "1z"));
    }

    #[test]
    fn sha() {
        let tiles = tiles_from_string("56m22p234567s333z");
        let calls = vec![];

        // Round wind
        assert_any_division(has_sha, &tiles, &calls, &ctw("7m", true, "3z", "1z"));
        // Player wind
        assert_any_division(has_sha, &tiles, &calls, &ctw("7m", true, "1z", "3z"));
        // Guest wind
        refute_all_divisions(has_sha, &tiles, &calls, &ctw("7m", true, "1z", "2z"));
    }

    #[test]
    fn pei() {
        let tiles = tiles_from_string("56m22p234567s");
        let calls = vec![c("ck", "4z")];

        // Round wind
        assert_any_division(has_pei, &tiles, &calls, &ctw("7m", true, "4z", "1z"));
        // Player wind
        assert_any_division(has_pei, &tiles, &calls, &ctw("7m", true, "1z", "4z"));
        // Guest wind
        refute_all_divisions(has_pei, &tiles, &calls, &ctw("7m", true, "2z", "3z"));
    }

    #[test]
    fn haku() {
        let tiles = tiles_from_string("56m22p234567s55z");
        let calls = vec![];

        assert_any_division(has_haku, &tiles, &calls, &ct("5z", true));
        refute_all_divisions(has_haku, &tiles, &calls, &ct("2p", true));
    }

    #[test]
    fn hatsu() {
        let tiles = tiles_from_string("56m22p234567s66z");
        let calls = vec![];

        assert_any_division(has_hatsu, &tiles, &calls, &ct("6z", true));
        refute_all_divisions(has_hatsu, &tiles, &calls, &ct("2p", true));
    }

    #[test]
    fn chun() {
        let tiles = tiles_from_string("56m22p234567s77z");
        let calls = vec![];

        assert_any_division(has_chun, &tiles, &calls, &ct("7z", true));
        refute_all_divisions(has_chun, &tiles, &calls, &ct("2p", true));
    }

    #[test]
    fn chanta() {
        let tiles = tiles_from_string("12378m123s22333z");
        let calls = vec![];

        assert_any_division(has_chanta, &tiles, &calls, &ct("9m", true));
        refute_all_divisions(has_chanta, &tiles, &calls, &ct("6m", true));

        // TODO: Negative case w/ noncompliant call
    }

    #[test]
    fn sanshoku_doujun() {
        let tiles = tiles_from_string("123m2345677p123s");
        let calls = vec![];

        assert_any_division(has_sanshoku_doujun, &tiles, &calls, &ct("1p", true));
        refute_all_divisions(has_sanshoku_doujun, &tiles, &calls, &ct("4p", true));
    }

    #[test]
    fn sanshoku_doukou() {
        let tiles = tiles_from_string("111m1145677p111s");
        let calls = vec![];

        assert_any_division(has_sanshoku_doukou, &tiles, &calls, &ct("1p", true));
        refute_all_divisions(has_sanshoku_doukou, &tiles, &calls, &ct("7p", true));
    }

    #[test]
    fn ittsu() {
        let tiles = tiles_from_string("12345678m123s22z");
        let calls = vec![];

        assert_any_division(has_ittsu, &tiles, &calls, &ct("9m", true));
        refute_all_divisions(has_ittsu, &tiles, &calls, &ct("6m", true));
    }

    #[test]
    fn toitoi() {
        assert_any_division(
            has_toitoi,
            &tiles_from_string("44555p11s"),
            &vec![c("p", "3m"), c("p", "3s")],
            &ct("1s", true),
        );
        refute_all_divisions(
            has_toitoi,
            &tiles_from_string("44567p11s"),
            &vec![c("p", "3m"), c("p", "3s")],
            &ct("1s", true),
        );

        // Toitoi + sanankou
        assert_any_division(
            has_toitoi,
            &tiles_from_string("44555p11333s"),
            &vec![c("p", "3m")],
            &ct("1s", true),
        );
    }

    #[test]
    fn sanankou() {
        assert_any_division(
            has_sanankou,
            &tiles_from_string("333m4455p111s"),
            &vec![c("p", "5s")],
            &ct("5p", true),
        );
        refute_all_divisions(
            has_sanankou,
            &tiles_from_string("333m4455p"),
            &vec![c("p", "1s"), c("p", "5s")],
            &ct("5p", true),
        );

        // TODO: Negative case w/ 3rd triplet ronned
    }

    #[test]
    fn sankantsu() {
        assert_any_division(
            has_sankantsu,
            &tiles_from_string("12m44p"),
            &vec![c("ok", "6p"), c("ck", "1s"), c("ok", "3s")],
            &ct("3m", true),
        );
        refute_all_divisions(
            has_sankantsu,
            &tiles_from_string("12m44p"),
            &vec![c("p", "6p"), c("ck", "1s"), c("ok", "3s")],
            &ct("3m", true),
        );
    }

    #[test]
    fn chiitoi() {
        // Special case: check division directly
        assert!(has_chiitoi(
            &Division {
                pair: vec![],
                melds: vec![],
                remaining: tiles_from_string("113344m55p22s3355z")
            },
            &vec![],
            &ct("4m", true)
        ));
    }

    #[test]
    fn honroutou() {
        assert_any_division(
            has_honroutou,
            &tiles_from_string("11199m111s22333z"),
            &vec![],
            &ct("9m", true),
        );
    }

    #[test]
    fn shousangen() {
        assert_any_division(
            has_shousangen,
            &tiles_from_string("345mm33s55666777z"),
            &vec![],
            &ct("3s", true),
        );
    }

    #[test]
    fn honitsu() {
        assert_any_division(
            has_honitsu,
            &tiles_from_string("123456789m1122z"),
            &vec![],
            &ct("1z", true),
        );
        refute_all_divisions(
            has_honitsu,
            &tiles_from_string("456789m123p1122z"),
            &vec![],
            &ct("1z", true),
        );
    }

    #[test]
    fn junchan() {
        let tiles = tiles_from_string("123789m789p2399s");
        let calls = vec![];

        assert_any_division(has_junchan, &tiles, &calls, &ct("1s", true));
        refute_all_divisions(has_junchan, &tiles, &calls, &ct("4s", true));

        refute_all_divisions(
            has_junchan,
            &tiles_from_string("111999m111s2233z"),
            &calls,
            &ct("3z", true),
        );
    }

    #[test]
    fn ryanpeikou() {
        assert_any_division(
            has_ryanpeikou,
            &tiles_from_string("22m223344p11233s"),
            &vec![],
            &ct("2s", true),
        );
        assert_any_division(
            has_ryanpeikou,
            &tiles_from_string("22m11122223333s"),
            &vec![],
            &ct("1s", true),
        );
        refute_all_divisions(
            has_ryanpeikou,
            &tiles_from_string("123m2344p112233s"),
            &vec![],
            &ct("4p", true),
        );
    }

    #[test]
    fn chinitsu() {
        assert_any_division(
            has_chinitsu,
            &tiles_from_string("1234566677889m"),
            &vec![],
            &ct("9m", true),
        );
    }

    #[test]
    fn tenhou() {
        let tiles = tiles_from_string("234456m66p12344s");
        let calls = vec![];
        let mut context = ctw("4s", true, "1z", "1z");
        context.is_tenhou = true;

        assert_any_division(has_tenhou, &tiles, &calls, &context);
    }

    #[test]
    fn chiihou() {
        let tiles = tiles_from_string("234456m66p12344s");
        let calls = vec![];
        let mut context = ctw("4s", true, "1z", "1z");
        context.is_chiihou = true;

        assert_any_division(has_chiihou, &tiles, &calls, &context);
    }

    #[test]
    fn kokushi() {
        // Special case: check division directly
        assert!(has_kokushi(
            &Division {
                pair: vec![],
                melds: vec![],
                remaining: tiles_from_string("19m19p19s12345667z")
            },
            &vec![],
            &ct("9m", true)
        ));
        assert!(!has_kokushi(
            &Division {
                pair: vec![],
                melds: vec![],
                remaining: tiles_from_string("1m199p19s12345667z")
            },
            &vec![],
            &ct("9p", true)
        ));
    }

    #[test]
    fn suuankou() {
        assert_any_division(
            has_suuankou,
            &tiles_from_string("333m4455p111555s"),
            &vec![],
            &ct("5p", true),
        );
        // TODO: Negative cases
    }

    #[test]
    fn daisangen() {
        assert_any_division(
            has_daisangen,
            &tiles_from_string("345m2s555666777z"),
            &vec![],
            &ct("2s", true),
        );
        refute_all_divisions(
            has_daisangen,
            &tiles_from_string("345m2s444666777z"),
            &vec![],
            &ct("2s", true),
        );
        refute_all_divisions(
            has_daisangen,
            &tiles_from_string("345m22s55566677z"),
            &vec![],
            &ct("2s", true),
        );
    }

    #[test]
    fn shousuushii() {
        assert_any_division(
            has_shousuushii,
            &tiles_from_string("888m1222z"),
            &vec![c("p", "3z"), c("p", "4z")],
            &ct("1z", true),
        );
    }

    #[test]
    fn daisuushii() {
        assert_any_division(
            has_daisuushii,
            &tiles_from_string("5p111222444z"),
            &vec![c("p", "3z")],
            &ct("5p", true),
        );
    }

    #[test]
    fn tsuuiisou() {
        assert_any_division(
            has_tsuuiisou,
            &tiles_from_string("1113344z"),
            &vec![c("p", "2z"), c("p", "6z")],
            &ct("3z", true),
        );
    }

    #[test]
    fn chinroutou() {
        assert_any_division(
            has_chinroutou,
            &tiles_from_string("11199m111999p11s"),
            &vec![],
            &ct("9m", true),
        );
    }

    #[test]
    fn ryuuiisou() {
        assert_any_division(
            has_ryuuiisou,
            &tiles_from_string("22334466688s66z"),
            &vec![],
            &ct("6z", true),
        );
    }

    #[test]
    fn chuuren() {
        let tiles = tiles_from_string("1112345678999m");
        assert_any_division(has_chuuren, &tiles, &vec![], &ct("1m", true));
        assert_any_division(has_chuuren, &tiles, &vec![], &ct("5m", true));
        assert_any_division(has_chuuren, &tiles, &vec![], &ct("9m", true));
    }

    #[test]
    fn suukantsu() {
        assert_any_division(
            has_suukantsu,
            &tiles_from_string("4z"),
            &vec![c("ok", "9p"), c("ck", "2m"), c("ck", "7z"), c("ok", "4s")],
            &ct("4z", true),
        );
    }

    fn assert_any_division(
        func: CheckFunc, tiles: &Vec<Tile>, calls: &Vec<Call>, context: &HandContext,
    ) {
        let divisions = make_divisions(tiles, context);
        // Make sure we have a division at all to avoid false positives
        assert!(divisions.len() >= 1);
        assert!(divisions.iter().any(|d| func(d, calls, context)))
    }

    fn refute_all_divisions(
        func: CheckFunc, tiles: &Vec<Tile>, calls: &Vec<Call>, context: &HandContext,
    ) {
        let divisions = make_divisions(tiles, context);
        // Make sure we have a division at all to avoid false negatives
        assert!(divisions.len() >= 1);
        assert!(divisions.iter().all(|d| !func(d, calls, context)))
    }

    fn make_divisions(tiles: &Vec<Tile>, context: &HandContext) -> Vec<Division> {
        let mut all_tiles = tiles.clone();
        all_tiles.push(context.winning_tile.clone());
        divide(&all_tiles)
    }

    fn c(call_type: &str, tile: &str) -> Call {
        Call {
            ctype: match call_type {
                "p" => CallType::Pon,
                "c" => CallType::Chi,
                "ok" => CallType::Minkan,
                "ck" => CallType::Ankan,
                _ => panic!("Invalid call type"),
            },
            tile: tile_from_string(tile),
        }
    }
    fn ct(winning_tile: &str, is_tsumo: bool) -> HandContext {
        HandContext { winning_tile: tile_from_string(winning_tile), is_tsumo, ..Default::default() }
    }

    fn ctw(winning_tile: &str, is_tsumo: bool, round_wind: &str, player_wind: &str) -> HandContext {
        HandContext {
            winning_tile: tile_from_string(winning_tile),
            is_tsumo,
            round_wind: tile_from_string(round_wind),
            player_wind: tile_from_string(player_wind),
            ..Default::default()
        }
    }
}
