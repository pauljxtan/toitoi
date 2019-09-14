//! Shared public types.

use crate::tile::Tile;
use std::fmt;

/// Represents a call.
#[derive(Debug)]
pub struct Call {
    pub ctype: CallType,
    // First tile is enough to determine the rest
    pub tile: Tile,
}

/// Represents a call type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallType {
    Chi,
    Pon,
    Minkan,
    Ankan,
    NoCall,
}

/// Represents a fu reason.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FuReason {
    Base,
    Chiitoitsu,
    // Winning condition
    ClosedRon,
    TsumoNoPinfu,
    OpenPinfu,
    // Pair
    YakuhaiPairColours,
    YakuhaiPairRoundWind,
    YakuhaiPairPlayerWind,
    // Triplets
    OpenTripletHonours,
    OpenTripletTerminals,
    OpenTripletSimples,
    ClosedTripletHonours,
    ClosedTripletTerminals,
    ClosedTripletSimples,
    // Quads
    OpenQuadHonours,
    OpenQuadTerminals,
    OpenQuadSimples,
    ClosedQuadHonours,
    ClosedQuadTerminals,
    ClosedQuadSimples,
    // Wait
    Kanchan,
    Penchan,
    Tanki,
    // Misc
    RoundUp,
    NotWinning,
}

/// Represents the union of yaku and yakuman.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub enum HanReason {
    Yaku(Yaku),
    Yakuman(Yakuman),
}

/// Represents yaku.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Yaku {
    // 1 han, closed only
    MenzenTsumo,
    Riichi,
    Ippatsu,
    Pinfu,
    Iipeikou,
    // 1 han, may be open
    HaiteiRaoyue,
    HouteiRaoyui,
    RinshanKaihou,
    Chankan,
    Tanyao,
    Ton,
    Nan,
    Sha,
    Pei,
    Haku,
    Hatsu,
    Chun,
    // 2 han, closed only
    DoubleRiichi,
    // 2 han, may be open
    Chantaiyao,
    SanshokuDoujun,
    Sankantsu,
    Ittsu,
    Toitoi,
    Sanankou,
    SanshokuDoukou,
    Chiitoitsu,
    Honroutou,
    Shousangen,
    // 3 han, closed only
    Ryanpeikou,
    // 3 han, may be open
    Honitsu,
    JunchanTaiyao,
    // 6 han, may be open
    Chinitsu,
    // 1 han each
    Dora,
    Akadora,
    Uradora,
}

/// Represents yakuman.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Yakuman {
    KazoeYakuman,
    KokushiMusou,
    Suuankou,
    Daisangen,
    Shousuushii,
    Daisuushii,
    Tsuuiisou,
    Chinroutou,
    Ryuuiisou,
    ChuurenPoutou,
    Suukantsu,
    Tenhou,
    Chiihou,
    NagashiMangan,
}

/// Stores contextual information needed to score a hand.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct HandContext {
    pub winning_tile: Tile,
    pub is_tsumo: bool,
    pub round_wind: Tile,
    pub player_wind: Tile,
    pub is_riichi: bool,
    pub is_double_riichi: bool,
    pub is_ippatsu: bool,
    pub is_haitei: bool,
    pub is_houtei: bool,
    pub is_rinshan: bool,
    pub is_chankan: bool,
    pub is_tenhou: bool,
    pub is_chiihou: bool,
    pub winning_meld: Vec<Tile>,
    pub n_dora: u8,
    pub n_akadora: u8,
    pub n_uradora: u8,
}

/// Represents a scoring limit.
#[derive(Debug, Eq, PartialEq)]
pub enum Limit {
    NoLimit,
    Mangan,
    Haneman,
    Baiman,
    Sanbaiman,
    Yakuman,
}

/// Represents an allocation of points.
#[derive(Debug, Eq, PartialEq)]
pub enum Points {
    NoPoints,
    /// For dealer tsumo: the same value is paid by each nondealer.
    TsumoAll(u16),
    /// For nondealer tsumo: the first value is paid by each nondealer and the second value is
    /// paid by the dealer.
    Tsumo(u16, u16),
    /// For ron: the single value is paid in full by the discarder.
    Ron(u16),
}

impl fmt::Display for FuReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FuReason::Base => "Base",
                FuReason::Chiitoitsu => "Chiitoitsu",
                FuReason::ClosedRon => "Closed ron",
                FuReason::TsumoNoPinfu => "Tsumo, no pinfu",
                FuReason::OpenPinfu => "Open pinfu",
                FuReason::YakuhaiPairColours => "Yakuhai pair (colours)",
                FuReason::YakuhaiPairRoundWind => "Yakuhai pair (round wind)",
                FuReason::YakuhaiPairPlayerWind => "Yakuhai pair (player wind)",
                FuReason::OpenTripletHonours => "Open triplet (honours)",
                FuReason::OpenTripletTerminals => "Open triplet (terminals)",
                FuReason::OpenTripletSimples => "Open triplet (simples)",
                FuReason::ClosedTripletHonours => "Closed triplet (honours)",
                FuReason::ClosedTripletTerminals => "Closed triplet (terminals)",
                FuReason::ClosedTripletSimples => "Closed triplet (simples)",
                FuReason::OpenQuadHonours => "Open quad (honours)",
                FuReason::OpenQuadTerminals => "Open quad (terminals)",
                FuReason::OpenQuadSimples => "Open quad (simples)",
                FuReason::ClosedQuadHonours => "Closed quad (honours)",
                FuReason::ClosedQuadTerminals => "Closed quad (terminals)",
                FuReason::ClosedQuadSimples => "Closed quad (simples)",
                FuReason::Kanchan => "Kanchan wait",
                FuReason::Penchan => "Penchan wait",
                FuReason::Tanki => "Tanki wait",
                FuReason::RoundUp => "Round up",
                FuReason::NotWinning => "Not winning",
            }
        )
    }
}

impl fmt::Display for HanReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HanReason::Yaku(Yaku::MenzenTsumo) => "Menzen tsumo",
                HanReason::Yaku(Yaku::Riichi) => "Riichi",
                HanReason::Yaku(Yaku::Ippatsu) => "Ippatsu",
                HanReason::Yaku(Yaku::Pinfu) => "Pinfu",
                HanReason::Yaku(Yaku::Iipeikou) => "Iipeikou",
                HanReason::Yaku(Yaku::HaiteiRaoyue) => "Haitei raoyue",
                HanReason::Yaku(Yaku::HouteiRaoyui) => "Houtei raoyui",
                HanReason::Yaku(Yaku::RinshanKaihou) => "Rinshan kaihou",
                HanReason::Yaku(Yaku::Chankan) => "Chankan",
                HanReason::Yaku(Yaku::Tanyao) => "Tanyao",
                HanReason::Yaku(Yaku::Ton) => "Ton",
                HanReason::Yaku(Yaku::Nan) => "Nan",
                HanReason::Yaku(Yaku::Sha) => "Sha",
                HanReason::Yaku(Yaku::Pei) => "Pei",
                HanReason::Yaku(Yaku::Haku) => "Haku",
                HanReason::Yaku(Yaku::Hatsu) => "Hatsu",
                HanReason::Yaku(Yaku::Chun) => "Chun",
                HanReason::Yaku(Yaku::DoubleRiichi) => "Double riichi",
                HanReason::Yaku(Yaku::Chantaiyao) => "Chanta",
                HanReason::Yaku(Yaku::SanshokuDoujun) => "Sanshoku doujun",
                HanReason::Yaku(Yaku::Sankantsu) => "Sankantsu",
                HanReason::Yaku(Yaku::Ittsu) => "Ittsu",
                HanReason::Yaku(Yaku::Toitoi) => "Toitoi",
                HanReason::Yaku(Yaku::Sanankou) => "Sanankou",
                HanReason::Yaku(Yaku::SanshokuDoukou) => "Sanshoku doukou",
                HanReason::Yaku(Yaku::Chiitoitsu) => "Chiitoitsu",
                HanReason::Yaku(Yaku::Honroutou) => "Honroutou",
                HanReason::Yaku(Yaku::Shousangen) => "Shousangen",
                HanReason::Yaku(Yaku::Ryanpeikou) => "Ryanpeikou",
                HanReason::Yaku(Yaku::Honitsu) => "Honitsu",
                HanReason::Yaku(Yaku::JunchanTaiyao) => "Junchan",
                HanReason::Yaku(Yaku::Chinitsu) => "Chinitsu",
                HanReason::Yaku(Yaku::Dora) => "Dora",
                HanReason::Yaku(Yaku::Akadora) => "Akadora",
                HanReason::Yaku(Yaku::Uradora) => "Uradora",

                HanReason::Yakuman(Yakuman::KazoeYakuman) => "Kazoe yakuman",
                HanReason::Yakuman(Yakuman::KokushiMusou) => "Kokushi musou",
                HanReason::Yakuman(Yakuman::Suuankou) => "Suuankou",
                HanReason::Yakuman(Yakuman::Daisangen) => "Daisangen",
                HanReason::Yakuman(Yakuman::Shousuushii) => "Shousuushii",
                HanReason::Yakuman(Yakuman::Daisuushii) => "Daisuushii",
                HanReason::Yakuman(Yakuman::Tsuuiisou) => "Tsuuiisou",
                HanReason::Yakuman(Yakuman::Chinroutou) => "Chinroutou",
                HanReason::Yakuman(Yakuman::Ryuuiisou) => "Ryuuiisou",
                HanReason::Yakuman(Yakuman::ChuurenPoutou) => "Chuuren poutou",
                HanReason::Yakuman(Yakuman::Suukantsu) => "Suukantsu",
                HanReason::Yakuman(Yakuman::Tenhou) => "Tenhou",
                HanReason::Yakuman(Yakuman::Chiihou) => "Chiihou",
                HanReason::Yakuman(Yakuman::NagashiMangan) => "Nagashi mangan",
            }
        )
    }
}

impl fmt::Display for Limit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Limit::NoLimit => "No limit",
            Limit::Mangan => "Mangan",
            Limit::Haneman => "Haneman",
            Limit::Baiman => "Baiman",
            Limit::Sanbaiman => "Sanbaiman",
            Limit::Yakuman => "Yakuman"
        })
    }
}
