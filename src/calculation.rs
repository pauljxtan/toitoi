//! Hand calculator.

use crate::{
    division::{divide, Division},
    fu::fu_for_division,
    meld::no_open_calls,
    tile::Tile,
    types::{Call, FuReason, HanReason, HandContext, Yaku, Yakuman},
    yaku::{yaku_in_hand, yakuman_in_hand},
};
use itertools::{repeat_n, Itertools};

/// Stores calculation results.
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct CalcResult {
    pub winning_tile: Tile,
    pub pair: Vec<Tile>,
    pub melds: Vec<Vec<Tile>>,
    pub remaining: Vec<Tile>,
    pub fu_reasons: Vec<(FuReason, u8)>,
    pub han_reasons: Vec<(HanReason, u8)>,
    //pub calls: Vec<Call>,
}

/// Calculates fu and han for all winning hand configurations.
pub fn calculate(tiles: &Vec<Tile>, calls: &Vec<Call>, context: &HandContext) -> Vec<CalcResult> {
    let mut results: Vec<CalcResult> = divide(tiles)
        .into_iter()
        .filter(|d| d.is_tenpai(calls.len()))
        .filter(|d| d.wins_on(&context.winning_tile))
        .map(|d| calculate_division(&d, calls, context))
        .sorted()
        .collect();
    // Separate checks for chiitoi and kokushi
    match check_chiitoi(tiles, calls, context) {
        Some(result) => results.push(result),
        None => (),
    }
    match check_kokushi(tiles, calls, context) {
        Some(result) => results.push(result),
        None => (),
    }
    results
}

fn calculate_division(division: &Division, calls: &Vec<Call>, context: &HandContext) -> CalcResult {
    let fu_reasons = fu_for_division(&division, calls, context);

    // Add in winning tile for yaku calculation
    let mut con = (*context).clone();
    let mut div = (*division).clone();

    // Assume tanki if a pair is not formed
    let mut winning_meld =
        if division.pair.len() != 2 { division.pair.clone() } else { division.remaining.clone() };
    winning_meld.push(context.winning_tile.clone());
    winning_meld.sort();
    con.winning_meld = winning_meld.clone();

    if division.pair.len() != 2 {
        div.pair = winning_meld;
    } else {
        div.melds.push(winning_meld);
    }
    div.remaining = vec![];

    let mut han_reasons = han_for_division(&div, calls, &con);

    // Check for pinfu
    if is_pinfu(&fu_reasons) {
        han_reasons.push((HanReason::Yaku(Yaku::Pinfu), Yaku::Pinfu.han_closed()));
    }

    CalcResult {
        winning_tile: context.winning_tile.clone(),
        pair: div.pair.clone(),
        melds: div.melds.clone(),
        remaining: div.remaining.clone(),
        fu_reasons,
        han_reasons,
    }
}

fn is_pinfu(fu_reasons: &Vec<(FuReason, u8)>) -> bool {
    fu_reasons
        .iter()
        .all(|(r, _)| *r == FuReason::Base || *r == FuReason::ClosedRon || *r == FuReason::RoundUp)
}

fn han_for_division(
    division: &Division, calls: &Vec<Call>, context: &HandContext,
) -> Vec<(HanReason, u8)> {
    let hand_is_closed = no_open_calls(calls);
    let mut reasons = vec![];

    for yakuman in yakuman_in_hand(division, calls, context) {
        reasons.push((
            HanReason::Yakuman(yakuman.clone()),
            if hand_is_closed { yakuman.han_closed() } else { yakuman.han_open() },
        ))
    }
    for yaku in yaku_in_hand(division, calls, context) {
        reasons.push((
            HanReason::Yaku(yaku.clone()),
            if hand_is_closed { yaku.han_closed() } else { yaku.han_open() },
        ));
        // Check for double winds
        if context.round_wind == context.player_wind && yaku.is_wind() {
            reasons.push((HanReason::Yaku(yaku), 1));
        }
    }
    // Take dora counts directly from hand context
    reasons
        .append(&mut repeat_n((HanReason::Yaku(Yaku::Dora), 1), context.n_dora as usize).collect());
    reasons.append(
        &mut repeat_n((HanReason::Yaku(Yaku::Akadora), 1), context.n_akadora as usize).collect(),
    );
    reasons.append(
        &mut repeat_n((HanReason::Yaku(Yaku::Uradora), 1), context.n_uradora as usize).collect(),
    );
    reasons
}

// TODO: Optimization - only need to check contextual yaku with chiitoi and kokushi

fn check_chiitoi(
    tiles: &Vec<Tile>, calls: &Vec<Call>, context: &HandContext,
) -> Option<CalcResult> {
    // Make a dummy division
    let mut remaining = tiles.clone();
    remaining.push(context.winning_tile.clone());
    let division = Division { pair: vec![], melds: vec![], remaining };

    if Yaku::Chiitoitsu.check(&division, calls, context) {
        let han_reasons = han_for_division(&division, calls, context);
        Some(CalcResult {
            winning_tile: context.winning_tile.clone(),
            pair: division.pair,
            melds: division.melds,
            remaining: division.remaining,
            fu_reasons: vec![(FuReason::Chiitoitsu, 25)],
            han_reasons,
        })
    } else {
        None
    }
}

fn check_kokushi(
    tiles: &Vec<Tile>, calls: &Vec<Call>, context: &HandContext,
) -> Option<CalcResult> {
    // Make a dummy division
    let mut remaining = tiles.clone();
    remaining.push(context.winning_tile.clone());
    let division = Division { pair: vec![], melds: vec![], remaining };

    if Yakuman::KokushiMusou.check(&division, calls, context) {
        Some(CalcResult {
            winning_tile: context.winning_tile.clone(),
            pair: division.pair,
            melds: division.melds,
            remaining: division.remaining,
            fu_reasons: vec![],
            han_reasons: vec![(HanReason::Yakuman(Yakuman::KokushiMusou), 13)],
        })
    } else {
        None
    }
}
