//! Hand scoring.

use crate::{
    calculation::{calculate, CalcResult},
    tile::{tile_from_string, Tile},
    types::{Call, FuReason, HanReason, HandContext, Limit, Points},
};

/// Represents the scoring results for a single hand configuration (division).
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ScoreResult {
    calc_result: CalcResult,
    context: HandContext,
}

impl ScoreResult {
    pub fn pair(&self) -> Vec<Tile> { self.calc_result.pair.clone() }

    pub fn melds(&self) -> Vec<Vec<Tile>> { self.calc_result.melds.clone() }

    pub fn remaining(&self) -> Vec<Tile> { self.calc_result.remaining.clone() }

    /// The hand's total fu.
    pub fn fu(&self) -> u8 { self.calc_result.fu_reasons.iter().map(|(_, fu)| fu).sum() }

    /// The reasons contributing to the hand's total fu.
    pub fn fu_reasons(&self) -> Vec<(FuReason, u8)> { self.calc_result.fu_reasons.clone() }

    /// The hand's total han.
    pub fn han(&self) -> u8 {
        let han = self.calc_result.han_reasons.iter().map(|(_, han)| han).sum();
        if han > 13 {
            13
        } else {
            han
        }
    }

    /// The yaku or yakuman contributing to hand's total han.
    pub fn han_reasons(&self) -> Vec<(HanReason, u8)> {
        let (han_reasons_yakuman, han_reasons_yaku): (Vec<(HanReason, u8)>, Vec<(HanReason, u8)>) =
                                                      self.calc_result.han_reasons.iter().partition(|(hr, _)| match hr {
                                                          HanReason::Yakuman(_) => true,
                                                          _ => false,
                                                      });
        if han_reasons_yakuman.len() > 0 {
            han_reasons_yakuman
        } else {
            han_reasons_yaku
        }
    }

    /// The hand's limit (or lack of).
    pub fn limit(&self) -> Limit {
        match self.han() {
            0 | 1 | 2 => Limit::NoLimit,
            3 => {
                if self.fu() >= 70 {
                    Limit::Mangan
                } else {
                    Limit::NoLimit
                }
            }
            4 => {
                if self.fu() >= 40 {
                    Limit::Mangan
                } else {
                    Limit::NoLimit
                }
            }
            5 => Limit::Mangan,
            6 | 7 => Limit::Haneman,
            8 | 9 | 10 => Limit::Baiman,
            11 | 12 => Limit::Sanbaiman,
            _ => Limit::Yakuman,
        }
    }

    /// The points based on dealer/nondealer and tsumo/ron.
    pub fn points(&self) -> Points {
        if self.han() == 0 {
            return Points::NoPoints;
        }
        let (tsumo_base, ron_dealer, ron_nondealer) = self.points_lookup();
        if self.is_dealer() {
            if self.is_tsumo() {
                Points::TsumoAll(tsumo_base)
            } else {
                Points::Ron(ron_dealer)
            }
        } else {
            if self.is_tsumo() {
                Points::Tsumo((tsumo_base / 2) + (tsumo_base / 2) % 100, tsumo_base)
            } else {
                Points::Ron(ron_nondealer)
            }
        }
    }

    /// The total points received.
    pub fn points_total(&self) -> u16 {
        match self.points() {
            Points::NoPoints => 0,
            Points::TsumoAll(a) => a * 3,
            Points::Tsumo(a, b) => a * 2 + b,
            Points::Ron(a) => a,
        }
    }

    fn is_tsumo(&self) -> bool { self.context.is_tsumo }

    fn is_dealer(&self) -> bool { self.context.player_wind == tile_from_string("1z") }

    // (tsumo, dealer ron, non-dealer ron)
    fn points_lookup(&self) -> (u16, u16, u16) {
        match self.limit() {
            Limit::Mangan => (4000, 12000, 8000),
            Limit::Haneman => (6000, 18000, 12000),
            Limit::Baiman => (8000, 24000, 16000),
            Limit::Sanbaiman => (12000, 36000, 24000),
            Limit::Yakuman => (16000, 48000, 32000),
            Limit::NoLimit => self.points_lookup_nolimit(),
        }
    }

    fn points_lookup_nolimit(&self) -> (u16, u16, u16) {
        match self.han() {
            1 => match self.fu() {
                30 => (500, 1500, 1000),
                40 => (700, 2000, 1300),
                50 => (800, 2400, 1600),
                60 => (1000, 2900, 2000),
                70 => (1200, 3400, 2300),
                80 => (1300, 3900, 2600),
                90 => (1500, 4400, 2900),
                100 => (1600, 4800, 3200),
                110 => (1800, 5300, 3600),
                _ => panic!("Invalid fu"),
            },
            2 => match self.fu() {
                20 => (700, 2000, 1300),
                25 => (0, 2400, 1600),
                30 => (1000, 2900, 2000),
                40 => (1300, 3900, 2600),
                50 => (1600, 4800, 3200),
                60 => (2000, 5800, 3900),
                70 => (2300, 6800, 4500),
                80 => (2600, 7700, 5200),
                90 => (2900, 8700, 5800),
                100 => (3200, 9600, 6400),
                110 => (3600, 10600, 7100),
                _ => panic!("Invalid fu"),
            },
            3 => match self.fu() {
                20 => (1300, 3900, 2600),
                25 => (1600, 4800, 3200),
                30 => (2000, 5800, 3900),
                40 => (2600, 7700, 5200),
                50 => (3200, 9600, 6400),
                60 => (3900, 11600, 7700),
                _ => panic!("Invalid fu"),
            },
            4 => match self.fu() {
                20 => (2600, 7700, 5200),
                25 => (3200, 9600, 6400),
                30 => (3900, 11600, 7700),
                _ => panic!("Invalid fu"),
            },
            _ => panic!("Invalid han"),
        }
    }
}

/// Scores all winning hand combinations.
///
/// # Example
///
/// ```rust
/// use toitoi::score::score;
/// use toitoi::tile::{tile_from_string, tiles_from_string};
/// use toitoi::types::{Call, FuReason, HandContext, HanReason, Limit, Points, Yaku};
///
/// let tiles = tiles_from_string("456m1122z");
/// let calls = vec![
///     Call::ankan(tile_from_string("1s")),
///     Call::ankan(tile_from_string("7z")),
/// ];
/// let context = HandContext {
///     winning_tile: tile_from_string("1z"),
///     is_tsumo: false,
///     round_wind: tile_from_string("2z"),
///     player_wind: tile_from_string("2z"),
///     ..Default::default()
/// };
///
/// let results = score(&tiles, &calls, &context);
///
/// assert_eq!(results.len(), 1);
///
/// assert_eq!(results[0].fu(), 110);
/// assert_eq!(results[0].han(), 1);
/// assert_eq!(results[0].limit(), Limit::NoLimit);
/// assert_eq!(results[0].points(), Points::Ron(3600));
///
/// assert_eq!(results[0].fu_reasons(), vec![
///     (FuReason::Base, 20),
///     (FuReason::OpenTripletHonours, 4),
///     (FuReason::YakuhaiPairRoundWind, 2),
///     (FuReason::YakuhaiPairPlayerWind, 2),
///     (FuReason::ClosedQuadTerminals, 32),
///     (FuReason::ClosedQuadHonours, 32),
///     (FuReason::ClosedRon, 10),
///     (FuReason::RoundUp, 8),
/// ]);
/// assert_eq!(results[0].han_reasons(), vec![(HanReason::Yaku(Yaku::Chun), 1)]);
/// ```
pub fn score(tiles: &Vec<Tile>, calls: &Vec<Call>, context: &HandContext) -> Vec<ScoreResult> {
    // TODO: no need to clone context?
    calculate(tiles, calls, context)
        .into_iter()
        .map(|calc_result| ScoreResult { calc_result, context: context.clone() })
        .collect()
}
