use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use crate::settings::Lang;

/// 和了時の手牌の形態
#[derive(Debug, Eq, PartialEq)]
pub enum WinningHandForm {
    /// 七対子
    SevenPairs,
    /// 国士無双
    ThirteenOrphens,
    /// 通常（4面子1雀頭）の手牌
    Normal,
}

/// 和了役を表す列挙型
///
/// <https://en.wikipedia.org/wiki/Japanese_Mahjong_yaku>による英語名
#[derive(Debug, PartialEq, Eq, Hash, EnumCountMacro, EnumIter)]
pub enum WinningHandKind {
    /// 立直
    ReadyHand,
    /// 七対子
    SevenPairs,
    /// 流し満貫
    NagashiMangan,
    /// 門前清自摸和
    SelfPick,
    /// 一発
    OneShot,
    /// 海底撈月
    LastTileFromTheWall,
    /// 河底撈魚
    LastDiscard,
    /// 嶺上開花
    DeadWallDraw,
    /// 搶槓
    RobbingAQuad,
    /// ダブル立直
    DoubleReady,
    /// 平和
    NoPointsHand,
    /// 一盃口
    OneSetOfIdenticalSequences,
    /// 三色同順
    ThreeColourStraight,
    /// 一気通貫
    Straight,
    /// 二盃口
    TwoSetsOfIdenticalSequences,
    /// 対々和
    AllTripletHand,
    /// 三暗刻
    ThreeClosedTriplets,
    /// 三色同刻
    ThreeColourTriplets,
    /// 断么九
    AllSimples,
    /// 役牌（自風牌）
    HonorTilesPlayersWind,
    /// 役牌（場風牌）
    HonorTilesPrevailingWind,
    /// 役牌（白）
    HonorTilesWhiteDragon,
    /// 役牌（發）
    HonorTilesGreenDragon,
    /// 役牌（中）
    HonorTilesRedDragon,
    /// 混全帯么九
    TerminalOrHonorInEachSet,
    /// 純全帯么九
    TerminalInEachSet,
    /// 混老頭
    AllTerminalsAndHonors,
    /// 小三元
    LittleThreeDragons,
    /// 混一色
    HalfFlush,
    /// 清一色
    Flush,
    /// 国士無双
    ThirteenOrphans,
    /// 四暗刻
    FourConcealedTriplets,
    /// 大三元
    BigThreeDragons,
    /// 小四喜
    LittleFourWinds,
    /// 大四喜
    BigFourWinds,
    /// 字一色
    AllHonors,
    /// 清老頭
    AllTerminals,
    /// 緑一色
    AllGreen,
    /// 九蓮宝燈
    NineGates,
    /// 四槓子
    FourKans,
    /// 天和
    HeavenlyHand,
    /// 地和
    HandOfEarth,
}

/// 和了役の名前を返す
///
/// # Arguments
/// * `hand_kind` - 和了役の種類
/// * `has_opened` - 副露しているか否か（喰い下がり役は`true`にすると名前の後に「（鳴）」が付く）
/// * `lang` - 言語
pub fn get_winning_hand_name(
    hand_kind: WinningHandKind,
    has_openned: bool,
    lang: Lang,
) -> &'static str {
    match lang {
        Lang::En => get_winning_hand_name_en(hand_kind, has_openned),
        Lang::Ja => get_winning_hand_name_ja(hand_kind, has_openned),
    }
}

/// 喰い下がり役に対しては「（鳴）」を付けるマクロ
///
/// # Examples
/// 
/// ```
/// assert_eq!(oppened_name!("三色同順", true, Lang::Ja), "三色同順（鳴）");
/// assert_eq!(oppened_name!("三色同順", false, Lang::Ja), "三色同順");
/// assert_eq!(oppened_name!("Three Colour Triplets", true, Lang::En), "Three Colour Triplets (Open)");
/// assert_eq!(oppened_name!("Three Colour Triplets", false, Lang::En), "Three Colour Triplets");
/// ```
macro_rules! openned_name {
    ($str:expr, $open:expr, $lang:expr) => {
        match $open {
            true => match $lang {
                Lang::En => concat!($str, " (Open)"),
                Lang::Ja => concat!($str, "（鳴）"),
            },
            _ => $str,
        }
    };
}

fn get_winning_hand_name_en(hand_kind: WinningHandKind, has_openned: bool) -> &'static str {
    match hand_kind {
        // 立直
        WinningHandKind::ReadyHand => "Ready Hand",
        // 七対子
        WinningHandKind::SevenPairs => "Seven Pairs",
        // 流し満貫
        WinningHandKind::NagashiMangan => "Nagashi Mangan",
        // 門前清自摸和
        WinningHandKind::SelfPick => "Self Pick",
        // 一発
        WinningHandKind::OneShot => "One Shot",
        // 海底撈月
        WinningHandKind::LastTileFromTheWall => "Last Tile From The Wall",
        // 河底撈魚
        WinningHandKind::LastDiscard => "Last Discard",
        // 嶺上開花
        WinningHandKind::DeadWallDraw => "Dead Wall Draw",
        // 搶槓
        WinningHandKind::RobbingAQuad => "Robbing A Quad",
        // ダブル立直
        WinningHandKind::DoubleReady => "Double Ready",
        // 平和
        WinningHandKind::NoPointsHand => "No Points Hand",
        // 一盃口
        WinningHandKind::OneSetOfIdenticalSequences => "One Set Of Identical Sequences",
        // 三色同順
        WinningHandKind::ThreeColourStraight => {
            openned_name!("Three Colour Straight", has_openned, Lang::En)
        }
        // 一気通貫
        WinningHandKind::Straight => openned_name!("Straight", has_openned, Lang::En),

        // 二盃口
        WinningHandKind::TwoSetsOfIdenticalSequences => "Two Sets Of Identical Sequences",
        // 対々和
        WinningHandKind::AllTripletHand => "All Triplet Hand",
        // 三暗刻
        WinningHandKind::ThreeClosedTriplets => "Three Closed Triplets",
        // 三色同刻
        WinningHandKind::ThreeColourTriplets => "Three Colour Triplets",
        // 断么九
        WinningHandKind::AllSimples => "All Simples",
        // 役牌（自風牌）
        WinningHandKind::HonorTilesPlayersWind => "Honor Tiles (Players Wind)",
        // 役牌（場風牌）
        WinningHandKind::HonorTilesPrevailingWind => "Honor Tiles (Prevailing Wind)",
        // 役牌（白）
        WinningHandKind::HonorTilesWhiteDragon => "Honor Tiles (White Dragon)",
        // 役牌（發）
        WinningHandKind::HonorTilesGreenDragon => "Honor Tiles (Green Dragon)",
        // 役牌（中）
        WinningHandKind::HonorTilesRedDragon => "Honor Tiles (Red Dragon)",
        // 混全帯么九
        WinningHandKind::TerminalOrHonorInEachSet => {
            openned_name!("Terminal Or Honor In Each Set", has_openned, Lang::En)
        }
        // 純全帯么九
        WinningHandKind::TerminalInEachSet => {
            openned_name!("Terminal In Each Set", has_openned, Lang::En)
        }
        // 混老頭
        WinningHandKind::AllTerminalsAndHonors => "All Terminals And Honors",
        // 小三元
        WinningHandKind::LittleThreeDragons => "Little Three Dragons",
        // 混一色
        WinningHandKind::HalfFlush => {
            openned_name!("Half Flush", has_openned, Lang::En)
        }
        // 清一色
        WinningHandKind::Flush => {
            openned_name!("Flush", has_openned, Lang::En)
        }
        // 国士無双
        WinningHandKind::ThirteenOrphans => "Thirteen Orphans",
        // 四暗刻
        WinningHandKind::FourConcealedTriplets => "Four Concealed Triplets",
        // 大三元
        WinningHandKind::BigThreeDragons => "Big Three Dragons",
        // 小四喜
        WinningHandKind::LittleFourWinds => "Little Four Winds",
        // 大四喜
        WinningHandKind::BigFourWinds => "Big Four Winds",
        // 字一色
        WinningHandKind::AllHonors => "All Honors",
        // 清老頭
        WinningHandKind::AllTerminals => "All Terminals",
        // 緑一色
        WinningHandKind::AllGreen => "All Green",
        // 九蓮宝燈
        WinningHandKind::NineGates => "Nine Gates",
        // 四槓子
        WinningHandKind::FourKans => "Four Kans",
        // 天和
        WinningHandKind::HeavenlyHand => "Heavenly Hand",
        // 地和
        WinningHandKind::HandOfEarth => "Hand Of Earth",
    }
}
fn get_winning_hand_name_ja(hand_kind: WinningHandKind, has_openned: bool) -> &'static str {
    match hand_kind {
        // 立直
        WinningHandKind::ReadyHand => "立直",
        // 七対子
        WinningHandKind::SevenPairs => "七対子",
        // 流し満貫
        WinningHandKind::NagashiMangan => "流し満貫",
        // 門前清自摸和
        WinningHandKind::SelfPick => "門前清自摸和",
        // 一発
        WinningHandKind::OneShot => "一発",
        // 海底撈月
        WinningHandKind::LastTileFromTheWall => "海底撈月",
        // 河底撈魚
        WinningHandKind::LastDiscard => "河底撈魚",
        // 嶺上開花
        WinningHandKind::DeadWallDraw => "嶺上開花",
        // 搶槓
        WinningHandKind::RobbingAQuad => "搶槓",
        // ダブル立直
        WinningHandKind::DoubleReady => "ダブル立直",
        // 平和
        WinningHandKind::NoPointsHand => "平和",
        // 一盃口
        WinningHandKind::OneSetOfIdenticalSequences => "一盃口",
        // 三色同順
        WinningHandKind::ThreeColourStraight => {
            openned_name!("三色同順", has_openned, Lang::Ja)
        }
        // 一気通貫
        WinningHandKind::Straight => {
            openned_name!("一気通貫", has_openned, Lang::Ja)
        }
        // 二盃口
        WinningHandKind::TwoSetsOfIdenticalSequences => "二盃口",
        // 対々和
        WinningHandKind::AllTripletHand => "対々和",
        // 三暗刻
        WinningHandKind::ThreeClosedTriplets => "三暗刻",
        // 三色同刻
        WinningHandKind::ThreeColourTriplets => "三色同刻",
        // 断么九
        WinningHandKind::AllSimples => "断么九",
        // 役牌（自風牌）
        WinningHandKind::HonorTilesPlayersWind => "役牌（自風牌）",
        // 役牌（場風牌）
        WinningHandKind::HonorTilesPrevailingWind => "役牌（場風牌）",
        // 役牌（白）
        WinningHandKind::HonorTilesWhiteDragon => "役牌（白）",
        // 役牌（發）
        WinningHandKind::HonorTilesGreenDragon => "役牌（發）",
        // 役牌（中）
        WinningHandKind::HonorTilesRedDragon => "役牌（中）",
        // 混全帯么九
        WinningHandKind::TerminalOrHonorInEachSet => {
            openned_name!("混全帯么九", has_openned, Lang::Ja)
        }
        // 純全帯么九
        WinningHandKind::TerminalInEachSet => {
            openned_name!("純全帯么九", has_openned, Lang::Ja)
        }
        // 混老頭
        WinningHandKind::AllTerminalsAndHonors => "混老頭",
        // 小三元
        WinningHandKind::LittleThreeDragons => "小三元",
        // 混一色
        WinningHandKind::HalfFlush => {
            openned_name!("混一色", has_openned, Lang::Ja)
        }
        // 清一色
        WinningHandKind::Flush => {
            openned_name!("清一色", has_openned, Lang::Ja)
        }
        // 国士無双
        WinningHandKind::ThirteenOrphans => "国士無双",
        // 四暗刻
        WinningHandKind::FourConcealedTriplets => "四暗刻",
        // 大三元
        WinningHandKind::BigThreeDragons => "大三元",
        // 小四喜
        WinningHandKind::LittleFourWinds => "小四喜",
        // 大四喜
        WinningHandKind::BigFourWinds => "大四喜",
        // 字一色
        WinningHandKind::AllHonors => "字一色",
        // 清老頭
        WinningHandKind::AllTerminals => "清老頭",
        // 緑一色
        WinningHandKind::AllGreen => "緑一色",
        // 九蓮宝燈
        WinningHandKind::NineGates => "九蓮宝燈",
        // 四槓子
        WinningHandKind::FourKans => "四槓子",
        // 天和
        WinningHandKind::HeavenlyHand => "天和",
        // 地和
        WinningHandKind::HandOfEarth => "地和",
    }
}
