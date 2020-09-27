//! Integration tests.

extern crate toitoi;
use itertools::Itertools;
use toitoi::{
    score::{score, ScoreResult},
    tile::{tile_from_string, tiles_from_string},
    types::{Call, FuReason, HanReason, HandContext, Limit, Points, Yaku, Yakuman},
};

// ---- 0 han (for testing fu)

#[test]
fn score_0han_30fu() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::pon(tile_from_string("2s"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::OpenTripletSimples],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_kanchan() {
    let results = score(&tiles_from_string("123456m55p12357s"), &vec![], &ct("6s", false));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::Kanchan, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_penchan() {
    let results = score(&tiles_from_string("123456m55p12456s"), &vec![], &ct("3s", false));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::Penchan, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_tanki() {
    let results = score(&tiles_from_string("123456m1p123678s"), &vec![], &ct("1p", false));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::Tanki, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_yakuhai_pair() {
    let results =
        score(&tiles_from_string("123456m12378s11z"), &vec![], &ctw("6s", false, "1z", "3z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::YakuhaiPairRoundWind, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_closed_triplet_simples() {
    let results = score(&tiles_from_string("123456m11p22278s"), &vec![], &ct("6s", false));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::ClosedTripletSimples, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_ronned_triplet_simples() {
    let results = score(&tiles_from_string("123456m11p22678s"), &vec![], &ct("2s", false));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::OpenTripletSimples, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_closed_triplet_terminals() {
    let results = score(&tiles_from_string("123456m11p11178s"), &vec![], &ct("6s", false));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::ClosedTripletTerminals, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_ronned_triplet_terminals() {
    let results = score(&tiles_from_string("123456m11p11678s"), &vec![], &ct("1s", false));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::OpenTripletTerminals, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_closed_triplet_honours() {
    let results =
        score(&tiles_from_string("123456m1178s111z"), &vec![], &ctw("6s", false, "2z", "2z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::ClosedTripletHonours, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_ronned_triplet_honours() {
    let results =
        score(&tiles_from_string("123456m11678s11z"), &vec![], &ctw("1z", false, "2z", "2z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::OpenTripletHonours, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_open_triplet_simples() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::pon(tile_from_string("2s"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::OpenTripletSimples],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_open_triplet_terminals() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::pon(tile_from_string("1s"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::OpenTripletTerminals],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_open_triplet_honours() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::pon(tile_from_string("1z"))],
        &ctw("6s", false, "2z", "2z"),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::OpenTripletHonours],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_closed_quad_simples() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::ankan(tile_from_string("2s"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        50,
        vec![FuReason::Base, FuReason::ClosedQuadSimples, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_open_quad_simples() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::minkan(tile_from_string("2s"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::OpenQuadSimples],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_closed_quad_terminals() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::ankan(tile_from_string("1s"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        70,
        vec![FuReason::Base, FuReason::ClosedQuadTerminals, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_open_quad_terminals() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::minkan(tile_from_string("1s"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::OpenQuadTerminals],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_closed_quad_honours() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::ankan(tile_from_string("3z"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        70,
        vec![FuReason::Base, FuReason::ClosedQuadHonours, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_0han_40fu_open_quad_honours() {
    let results = score(
        &tiles_from_string("123456m11p78s"),
        &vec![Call::minkan(tile_from_string("3z"))],
        &ct("6s", false),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::OpenQuadHonours],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

// ---- 1 han

#[test]
fn score_1han_30fu_nopinfu() {
    let results = score(&tiles_from_string("123456m111p2278s"), &vec![], &ct("6s", true));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::ClosedTripletTerminals, FuReason::TsumoNoPinfu],
        1,
        vec![HanReason::Yaku(Yaku::MenzenTsumo)],
        Limit::NoLimit,
        Points::TsumoAll(500),
    );
}

#[test]
fn score_1han_30fu_pinfu_or_40fu_nopinfu() {
    let results = score(&tiles_from_string("234789m1234566p"), &vec![], &ct("6p", false));

    assert_eq!(results.len(), 2);
    check(
        &results[1],
        30,
        vec![FuReason::Base, FuReason::ClosedRon],
        1,
        vec![HanReason::Yaku(Yaku::Pinfu)],
        Limit::NoLimit,
        Points::Ron(1500),
    );
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::Tanki, FuReason::ClosedRon],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_1han_110fu() {
    let results = score(
        &tiles_from_string("456m1122z"),
        &vec![Call::ankan(tile_from_string("1s")), Call::ankan(tile_from_string("7z"))],
        &ctw("1z", false, "2z", "2z"),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        110,
        vec![
            FuReason::Base,
            FuReason::OpenTripletHonours,
            FuReason::YakuhaiPairRoundWind,
            FuReason::YakuhaiPairPlayerWind,
            FuReason::ClosedQuadTerminals,
            FuReason::ClosedQuadHonours,
            FuReason::ClosedRon,
        ],
        1,
        vec![HanReason::Yaku(Yaku::Chun)],
        Limit::NoLimit,
        Points::Ron(3600),
    );
}

// ---- 2 han

#[test]
fn score_2han_20fu_pinfu() {
    let results =
        score(&tiles_from_string("123456m123p2278s"), &vec![], &ctw("6s", true, "1z", "2z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        20,
        vec![FuReason::Base],
        2,
        vec![HanReason::Yaku(Yaku::MenzenTsumo), HanReason::Yaku(Yaku::Pinfu)],
        Limit::NoLimit,
        Points::Tsumo(400, 700),
    );
}

#[test]
fn score_2han_110fu() {
    let results = score(
        &tiles_from_string("234s1z"),
        &vec![
            Call::minkan(tile_from_string("9m")),
            Call::ankan(tile_from_string("3z")),
            Call::ankan(tile_from_string("1p")),
        ],
        &ct("1z", true),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        110,
        vec![
            FuReason::Base,
            FuReason::OpenQuadTerminals,
            FuReason::ClosedQuadHonours,
            FuReason::ClosedQuadTerminals,
            FuReason::Tanki,
            FuReason::YakuhaiPairRoundWind,
            FuReason::YakuhaiPairPlayerWind,
            FuReason::TsumoNoPinfu,
        ],
        2,
        vec![HanReason::Yaku(Yaku::Sankantsu)],
        Limit::NoLimit,
        Points::TsumoAll(3600),
    );
}

#[test]
fn score_2han_30fu_4colours_2waits() {
    let results = score(
        &tiles_from_string("1233p567s"),
        &vec![Call::pon(tile_from_string("6z")), Call::pon(tile_from_string("7z"))],
        &ct("3p", false),
    );

    assert_eq!(results.len(), 2);
    check(
        &results[0],
        30,
        vec![
            FuReason::Base,
            FuReason::OpenTripletHonours,
            FuReason::OpenTripletHonours,
            FuReason::Tanki,
        ],
        2,
        vec![HanReason::Yaku(Yaku::Hatsu), HanReason::Yaku(Yaku::Chun)],
        Limit::NoLimit,
        Points::Ron(2900),
    );
    check(
        &results[1],
        30,
        vec![
            FuReason::Base,
            FuReason::OpenTripletHonours,
            FuReason::OpenTripletHonours,
            FuReason::Penchan,
        ],
        2,
        vec![HanReason::Yaku(Yaku::Hatsu), HanReason::Yaku(Yaku::Chun)],
        Limit::NoLimit,
        Points::Ron(2900),
    );
}

// ---- 4 han

#[test]
fn score_4han_30fu_tsumo_3dora() {
    let mut context = ctw("4s", true, "1z", "2z");
    context.n_dora = 2;
    context.n_akadora = 1;

    let results = score(&tiles_from_string("234456m66p12344s"), &vec![], &context);
    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::ClosedTripletSimples, FuReason::TsumoNoPinfu],
        4,
        vec![
            HanReason::Yaku(Yaku::MenzenTsumo),
            HanReason::Yaku(Yaku::Dora),
            HanReason::Yaku(Yaku::Dora),
            HanReason::Yaku(Yaku::Akadora),
        ],
        Limit::NoLimit,
        Points::Tsumo(2000, 3900),
    );
}

// ---- 6 han

#[test]
fn score_6han_50fu_open_honitsu_4dora() {
    let mut context = ctw("2p", false, "1z", "2z");
    context.n_dora = 1;
    context.n_uradora = 3;

    let results = score(
        &tiles_from_string("2244456799p"),
        &vec![Call::minkan(tile_from_string("4z"))],
        &context,
    );
    assert_eq!(results.len(), 1);
    check(
        &results[0],
        50,
        vec![
            FuReason::Base,
            FuReason::OpenTripletSimples,
            FuReason::ClosedTripletSimples,
            FuReason::OpenQuadHonours,
        ],
        6,
        vec![
            HanReason::Yaku(Yaku::Honitsu),
            HanReason::Yaku(Yaku::Dora),
            HanReason::Yaku(Yaku::Uradora),
            HanReason::Yaku(Yaku::Uradora),
            HanReason::Yaku(Yaku::Uradora),
        ],
        Limit::Haneman,
        Points::Ron(12000),
    );
}

// ---- 8 han

#[test]
fn score_8han_25fu_chiitoi_tsumo_riichi_4dora() {
    let mut context = ct("4m", true);
    context.is_riichi = true;
    context.n_dora = 4;

    let results = score(&tiles_from_string("11334m55p22s3355z"), &vec![], &context);
    assert_eq!(results.len(), 1);
    check(
        &results[0],
        25,
        vec![FuReason::Chiitoitsu],
        8,
        vec![
            HanReason::Yaku(Yaku::Chiitoitsu),
            HanReason::Yaku(Yaku::Riichi),
            HanReason::Yaku(Yaku::MenzenTsumo),
            HanReason::Yaku(Yaku::Dora),
            HanReason::Yaku(Yaku::Dora),
            HanReason::Yaku(Yaku::Dora),
            HanReason::Yaku(Yaku::Dora),
        ],
        Limit::Baiman,
        Points::TsumoAll(8000),
    );
}

// ---- 11 han

#[test]
fn score_11han_30fu_40fu() {
    let mut context = ct("1m", true);
    context.is_double_riichi = true;
    context.is_ippatsu = true;

    // 30 fu if ryanmen, 40 fu if tanki
    let results = score(&tiles_from_string("1123456789m111z"), &vec![], &context);

    assert_eq!(results.len(), 2);
    check(
        &results[1],
        30,
        vec![FuReason::Base, FuReason::ClosedTripletHonours, FuReason::TsumoNoPinfu],
        11,
        vec![
            HanReason::Yaku(Yaku::MenzenTsumo),
            HanReason::Yaku(Yaku::Ippatsu),
            HanReason::Yaku(Yaku::Ton),
            HanReason::Yaku(Yaku::Ton),
            HanReason::Yaku(Yaku::DoubleRiichi),
            HanReason::Yaku(Yaku::Ittsu),
            HanReason::Yaku(Yaku::Honitsu),
        ],
        Limit::Sanbaiman,
        Points::TsumoAll(12000),
    );
    check(
        &results[0],
        40,
        vec![
            FuReason::Base,
            FuReason::ClosedTripletHonours,
            FuReason::Tanki,
            FuReason::TsumoNoPinfu,
        ],
        11,
        vec![
            HanReason::Yaku(Yaku::MenzenTsumo),
            HanReason::Yaku(Yaku::Ippatsu),
            HanReason::Yaku(Yaku::Ton),
            HanReason::Yaku(Yaku::Ton),
            HanReason::Yaku(Yaku::DoubleRiichi),
            HanReason::Yaku(Yaku::Ittsu),
            HanReason::Yaku(Yaku::Honitsu),
        ],
        Limit::Sanbaiman,
        Points::TsumoAll(12000),
    );
}

// ---- Yakuman

#[test]
fn score_kokushi() {
    let results =
        score(&tiles_from_string("1m19p19s12345667z"), &vec![], &ctw("9m", true, "1z", "4z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        0,
        vec![],
        13,
        vec![HanReason::Yakuman(Yakuman::KokushiMusou)],
        Limit::Yakuman,
        Points::Tsumo(8000, 16000),
    );

    // 13-way wait
    let results1 =
        score(&tiles_from_string("19m19p19s1234567z"), &vec![], &ctw("9m", true, "1z", "4z"));

    assert_eq!(results1.len(), 1);
    check(
        &results1[0],
        0,
        vec![],
        13,
        vec![HanReason::Yakuman(Yakuman::KokushiMusou)],
        Limit::Yakuman,
        Points::Tsumo(8000, 16000),
    );
}

#[test]
fn score_daisangen() {
    let results =
        score(&tiles_from_string("12399m55666777z"), &vec![], &ctw("5z", true, "1z", "1z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        50,
        vec![
            FuReason::Base,
            FuReason::TsumoNoPinfu,
            FuReason::ClosedTripletHonours,
            FuReason::ClosedTripletHonours,
            FuReason::ClosedTripletHonours,
        ],
        13,
        vec![HanReason::Yakuman(Yakuman::Daisangen)],
        Limit::Yakuman,
        Points::TsumoAll(16000),
    );
}

#[test]
fn score_suuankou() {
    let results = score(
        &tiles_from_string("333p2223777s"),
        &vec![Call::ankan(tile_from_string("1s"))],
        &ctw("3s", true, "1z", "4z"),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        70,
        vec![
            FuReason::Base,
            FuReason::TsumoNoPinfu,
            FuReason::ClosedTripletSimples,
            FuReason::ClosedTripletSimples,
            FuReason::ClosedTripletSimples,
            FuReason::ClosedQuadTerminals,
            FuReason::Tanki,
        ],
        13,
        vec![HanReason::Yakuman(Yakuman::Suuankou)],
        Limit::Yakuman,
        Points::Tsumo(8000, 16000),
    );
}

// ---- Bugfixes from Tenhou tests

#[test]
fn score_tenhou_tsumo_pinfu_nopinfu() {
    let results =
        score(&tiles_from_string("33456m567p56789s"), &vec![], &ctw("7s", true, "1z", "4z"));

    assert_eq!(results.len(), 2);
    check(
        &results[1],
        20,
        vec![FuReason::Base],
        2,
        vec![HanReason::Yaku(Yaku::MenzenTsumo), HanReason::Yaku(Yaku::Pinfu)],
        Limit::NoLimit,
        Points::Tsumo(400, 700),
    );
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::Penchan, FuReason::TsumoNoPinfu],
        1,
        vec![HanReason::Yaku(Yaku::MenzenTsumo)],
        Limit::NoLimit,
        Points::Tsumo(300, 500),
    );
}

#[test]
fn score_tenhou_mentan_sanankou_or_mentanpin_iipeikou() {
    let results =
        score(&tiles_from_string("23455p55666777s"), &vec![], &ctw("5s", true, "1z", "2z"));

    assert_eq!(results.len(), 2);

    // 20 fu, 4 han (mentanpin + iipeikou)
    check(
        &results[0],
        20,
        vec![FuReason::Base],
        4,
        vec![
            HanReason::Yaku(Yaku::MenzenTsumo),
            HanReason::Yaku(Yaku::Pinfu),
            HanReason::Yaku(Yaku::Tanyao),
            HanReason::Yaku(Yaku::Iipeikou),
        ],
        Limit::NoLimit,
        Points::Tsumo(1300, 2600),
    );

    // 40 fu, 4 han (mentan + sanankou)
    check(
        &results[1],
        40,
        vec![
            FuReason::Base,
            FuReason::ClosedTripletSimples,
            FuReason::ClosedTripletSimples,
            FuReason::ClosedTripletSimples,
            FuReason::TsumoNoPinfu,
        ],
        4,
        vec![
            HanReason::Yaku(Yaku::MenzenTsumo),
            HanReason::Yaku(Yaku::Tanyao),
            HanReason::Yaku(Yaku::Sanankou),
        ],
        Limit::Mangan,
        Points::Tsumo(2000, 4000),
    );
}

#[test]
fn score_tenhou_ron_pinfu_or_nopinfu() {
    let results =
        score(&tiles_from_string("12345567m456p22z"), &vec![], &ctw("3m", false, "1z", "3z"));

    assert_eq!(results.len(), 2);

    // 30 fu, 1 han (pinfu)
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::ClosedRon],
        1,
        vec![HanReason::Yaku(Yaku::Pinfu)],
        Limit::NoLimit,
        Points::Ron(1000),
    );

    // 40 fu, 0 han
    check(
        &results[1],
        40,
        vec![FuReason::Base, FuReason::ClosedRon, FuReason::Penchan],
        0,
        vec![],
        Limit::NoLimit,
        Points::NoPoints,
    );
}

#[test]
fn score_tenhou_tsumo() {
    let results =
        score(&tiles_from_string("34455699m789p88s"), &vec![], &ctw("8s", true, "1z", "1z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::ClosedTripletSimples, FuReason::TsumoNoPinfu],
        1,
        vec![HanReason::Yaku(Yaku::MenzenTsumo)],
        Limit::NoLimit,
        Points::TsumoAll(500),
    );
}

#[test]
fn score_tenhou_open_pinfu_or_open_tanyao() {
    let results = score(
        &tiles_from_string("88m345678p56s"),
        &vec![Call::chi(tile_from_string("3s"))],
        &ctw("4s", false, "1z", "3z"),
    );

    // TODO: Scores pinfu if open tanyao disallowed
    assert_eq!(results.len(), 1);
    check(
        &results[0],
        30,
        vec![FuReason::Base, FuReason::OpenPinfu],
        1,
        vec![HanReason::Yaku(Yaku::Tanyao)],
        Limit::NoLimit,
        Points::Ron(1000),
    );
}

#[test]
fn score_sanshoku_or_pinfu() {
    let results =
        score(&tiles_from_string("5667788m678p678s"), &vec![], &ctw("5m", false, "1z", "3z"));

    assert_eq!(results.len(), 2);

    // 30 fu, 3 han
    check(
        &results[1],
        30,
        vec![FuReason::Base, FuReason::ClosedRon],
        3,
        vec![
            HanReason::Yaku(Yaku::Tanyao),
            HanReason::Yaku(Yaku::Pinfu),
            HanReason::Yaku(Yaku::Iipeikou),
        ],
        Limit::NoLimit,
        Points::Ron(3900),
    );

    // 40 fu, 4 han
    check(
        &results[0],
        40,
        vec![FuReason::Base, FuReason::Tanki, FuReason::ClosedRon],
        4,
        vec![
            HanReason::Yaku(Yaku::Iipeikou),
            HanReason::Yaku(Yaku::Tanyao),
            HanReason::Yaku(Yaku::SanshokuDoujun),
        ],
        Limit::Mangan,
        Points::Ron(8000),
    );
}

#[test]
fn score_tenhou_hatsu() {
    let results =
        score(&tiles_from_string("789m789p567s6667z"), &vec![], &ctw("7z", false, "1z", "1z"));

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        50,
        vec![
            FuReason::Base,
            FuReason::ClosedTripletHonours,
            FuReason::YakuhaiPairColours,
            FuReason::Tanki,
            FuReason::ClosedRon,
        ],
        1,
        vec![HanReason::Yaku(Yaku::Hatsu)],
        Limit::NoLimit,
        Points::Ron(2400),
    );
}

#[test]
fn score_tenhou_toitoi_sanshoku_doukou() {
    let results = score(
        &tiles_from_string("444999m4z"),
        &vec![Call::minkan(tile_from_string("4s")), Call::pon(tile_from_string("4p"))],
        &ct("4z", true),
    );

    assert_eq!(results.len(), 1);
    check(
        &results[0],
        50,
        vec![
            FuReason::Base,
            FuReason::ClosedTripletSimples,
            FuReason::ClosedTripletTerminals,
            FuReason::OpenQuadSimples,
            FuReason::OpenTripletSimples,
            FuReason::Tanki,
            FuReason::TsumoNoPinfu,
        ],
        4,
        vec![HanReason::Yaku(Yaku::Toitoi), HanReason::Yaku(Yaku::SanshokuDoukou)],
        Limit::Mangan,
        Points::TsumoAll(4000),
    );
}

// ---- Helpers

fn check(
    result: &ScoreResult, fu_expected: u8, fu_reasons_expected: Vec<FuReason>, han_expected: u8,
    han_reasons_expected: Vec<HanReason>, limit_expected: Limit, points_expected: Points,
)
{
    assert_eq!(result.fu(), fu_expected);
    check_fu_reasons(&result.fu_reasons(), fu_reasons_expected);
    assert_eq!(result.han(), han_expected);
    check_han_reasons(&result.han_reasons(), han_reasons_expected);
    assert_eq!(result.limit(), limit_expected);
    assert_eq!(result.points(), points_expected);
}

fn check_fu_reasons(reasons: &Vec<(FuReason, u8)>, expected: Vec<FuReason>) {
    assert_eq!(
        reasons
            .into_iter()
            .map(|(r, _)| r)
            .filter(|&r| *r != FuReason::RoundUp)
            .sorted()
            .collect::<Vec<&FuReason>>(),
        expected.iter().sorted().collect::<Vec<&FuReason>>()
    )
}

fn check_han_reasons(han_reasons: &Vec<(HanReason, u8)>, expected: Vec<HanReason>) {
    assert_eq!(
        han_reasons.into_iter().map(|(y, _)| y).sorted().collect::<Vec<&HanReason>>(),
        expected.iter().sorted().collect::<Vec<&HanReason>>()
    )
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
