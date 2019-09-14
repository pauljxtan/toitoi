use crate::{
    division::Division,
    meld::{is_kanchan, is_penchan, is_shanpon, is_triplet, no_open_calls},
    tile::Tile,
    types::{Call, CallType, FuReason, HandContext},
};

pub fn fu_for_division(
    division: &Division, calls: &[Call], context: &HandContext,
) -> Vec<(FuReason, u8)> {
    let mut reasons: Vec<FuReason> = vec![FuReason::Base];

    // Score closed melds
    reasons.append(&mut score_melds(
        division,
        &context.winning_tile,
        context.is_tsumo,
        &context.round_wind,
        &context.player_wind,
    ));

    // Score called melds
    for call in calls.iter() {
        reasons.append(&mut score_call(&call));
    }

    // Score waits
    if division.remaining.len() == 1 {
        reasons.push(FuReason::Tanki);
        // Score yakuhai pairs too
        if &division.remaining[0] == &context.round_wind {
            reasons.push(FuReason::YakuhaiPairRoundWind);
        }
        if &division.remaining[0] == &context.player_wind {
            reasons.push(FuReason::YakuhaiPairPlayerWind);
        }
        if division.remaining[0].is_colour() {
            reasons.push(FuReason::YakuhaiPairColours);
        }
    }
    if is_kanchan(&division.remaining) {
        reasons.push(FuReason::Kanchan);
    } else if is_penchan(&division.remaining) {
        reasons.push(FuReason::Penchan);
    }

    let hand_is_closed = no_open_calls(&calls);

    // Score winning condition
    if !context.is_tsumo {
        if hand_is_closed {
            reasons.push(FuReason::ClosedRon);
        }
    } else {
        if reasons.len() > 1 || !hand_is_closed {
            reasons.push(FuReason::TsumoNoPinfu);
        }
    }
    if reasons.len() == 1 && !hand_is_closed {
        reasons.push(FuReason::OpenPinfu);
    }

    // Add up fu
    let mut reasons_with_values: Vec<(FuReason, u8)> =
        reasons.into_iter().map(|r| (r, score_reason(&r))).collect();
    let total_fu: u8 = reasons_with_values.iter().map(|(_, fu)| fu).sum();
    // Round up to nearest multiple of 10
    if total_fu % 10 != 0 {
        let round_up_fu = 10 - (total_fu % 10);
        reasons_with_values.push((FuReason::RoundUp, round_up_fu));
    }
    reasons_with_values
}

fn score_melds(
    division: &Division, winning_tile: &Tile, tsumo: bool, round_wind: &Tile, player_wind: &Tile,
) -> Vec<FuReason> {
    let mut reasons = vec![];
    // Score the completed meld
    reasons.append(&mut score_completed_meld(&division.remaining, &winning_tile, tsumo));
    // Score the pair
    reasons.append(&mut score_pair(&division.pair, &round_wind, &player_wind));
    // Score closed melds
    for meld in division.melds.iter() {
        reasons.append(&mut score_meld(&meld));
    }
    reasons
}

fn score_pair(pair: &[Tile], round_wind: &Tile, player_wind: &Tile) -> Vec<FuReason> {
    let mut reasons = vec![];
    if pair.len() != 2 {
        return reasons;
    }
    if pair[0].is_colour() {
        reasons.push(FuReason::YakuhaiPairColours);
    }
    if &pair[0] == round_wind {
        reasons.push(FuReason::YakuhaiPairRoundWind);
    }
    if &pair[0] == player_wind {
        reasons.push(FuReason::YakuhaiPairPlayerWind);
    }
    reasons
}

fn score_completed_meld(remaining: &[Tile], winning_tile: &Tile, tsumo: bool) -> Vec<FuReason> {
    match is_shanpon(remaining) {
        true if winning_tile.is_honour() => match tsumo {
            true => vec![FuReason::ClosedTripletHonours],
            false => vec![FuReason::OpenTripletHonours],
        },
        true if winning_tile.is_terminal() => match tsumo {
            true => vec![FuReason::ClosedTripletTerminals],
            false => vec![FuReason::OpenTripletTerminals],
        },
        true => match tsumo {
            true => vec![FuReason::ClosedTripletSimples],
            false => vec![FuReason::OpenTripletSimples],
        },
        false => vec![],
    }
}

fn score_meld(meld: &Vec<Tile>) -> Vec<FuReason> {
    match is_triplet(meld) {
        true if meld[0].is_honour() => vec![FuReason::ClosedTripletHonours],
        true if meld[0].is_terminal() => vec![FuReason::ClosedTripletTerminals],
        true => vec![FuReason::ClosedTripletSimples],
        _ => vec![],
    }
}

fn score_call(call: &Call) -> Vec<FuReason> {
    match call {
        Call { ctype: CallType::Pon, tile } if tile.is_honour() => {
            vec![FuReason::OpenTripletHonours]
        }
        Call { ctype: CallType::Pon, tile } if tile.is_terminal() => {
            vec![FuReason::OpenTripletTerminals]
        }
        Call { ctype: CallType::Pon, tile: _ } => vec![FuReason::OpenTripletSimples],
        Call { ctype: CallType::Minkan, tile } if tile.is_honour() => {
            vec![FuReason::OpenQuadHonours]
        }
        Call { ctype: CallType::Minkan, tile } if tile.is_terminal() => {
            vec![FuReason::OpenQuadTerminals]
        }
        Call { ctype: CallType::Minkan, tile: _ } => vec![FuReason::OpenQuadSimples],
        Call { ctype: CallType::Ankan, tile } if tile.is_honour() => {
            vec![FuReason::ClosedQuadHonours]
        }
        Call { ctype: CallType::Ankan, tile } if tile.is_terminal() => {
            vec![FuReason::ClosedQuadTerminals]
        }
        Call { ctype: CallType::Ankan, tile: _ } => vec![FuReason::ClosedQuadSimples],
        _ => vec![],
    }
}

fn score_reason(reason: &FuReason) -> u8 {
    match reason {
        FuReason::Base => 20,
        FuReason::Chiitoitsu => 25,
        // Winning condition
        FuReason::ClosedRon => 10,
        FuReason::TsumoNoPinfu | FuReason::OpenPinfu => 2,
        // Pair
        FuReason::YakuhaiPairColours
        | FuReason::YakuhaiPairRoundWind
        | FuReason::YakuhaiPairPlayerWind => 2,
        // Triplets
        FuReason::OpenTripletHonours | FuReason::OpenTripletTerminals => 4,
        FuReason::OpenTripletSimples => 2,
        FuReason::ClosedTripletHonours | FuReason::ClosedTripletTerminals => 8,
        FuReason::ClosedTripletSimples => 4,
        // Quads
        FuReason::OpenQuadHonours | FuReason::OpenQuadTerminals => 16,
        FuReason::OpenQuadSimples => 8,
        FuReason::ClosedQuadHonours | FuReason::ClosedQuadTerminals => 32,
        FuReason::ClosedQuadSimples => 16,
        // Wait
        FuReason::Kanchan | FuReason::Penchan | FuReason::Tanki => 2,
        // Misc
        FuReason::RoundUp | FuReason::NotWinning => 0,
    }
}

#[cfg(test)]
mod tests {
    // Tested at API level
}
