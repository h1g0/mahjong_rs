use crate::settings::Lang;
use crate::hand_info::winning_hand::*;

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
            if has_openned {
                "Three Colour Straight (Open)"
            } else {
                "Three Colour Straight"
            }
        }
        // 一気通貫
        WinningHandKind::Straight => {
            if has_openned {
                "Straight (Open)"
            } else {
                "Straight"
            }
        }
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
            if has_openned {
                "Terminal Or Honor In Each Set (Open)"
            } else {
                "Terminal Or Honor In Each Set"
            }
        }
        // 純全帯么九
        WinningHandKind::TerminalInEachSet => {
            if has_openned {
                "Terminal In Each Set (Open)"
            } else {
                "Terminal In Each Set"
            }
        }
        // 混老頭
        WinningHandKind::AllTerminalsAndHonors => "All Terminals And Honors",
        // 小三元
        WinningHandKind::LittleThreeDragons => "Little Three Dragons",
        // 混一色
        WinningHandKind::HalfFlush => {
            if has_openned {
                "Half Flush (Open)"
            } else {
                "Half Flush"
            }
        }
        // 清一色
        WinningHandKind::Flush => {
            if has_openned {
                "Flush (Open)"
            } else {
                "Flush"
            }
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
            if has_openned {
                "三色同順（鳴）"
            } else {
                "三色同順"
            }
        }
        // 一気通貫
        WinningHandKind::Straight => {
            if has_openned {
                "一気通貫（鳴）"
            } else {
                "一気通貫"
            }
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
            if has_openned {
                "混全帯么九（鳴）"
            } else {
                "混全帯么九"
            }
        }
        // 純全帯么九
        WinningHandKind::TerminalInEachSet => {
            if has_openned {
                "純全帯么九（鳴）"
            } else {
                "純全帯么九"
            }
        }
        // 混老頭
        WinningHandKind::AllTerminalsAndHonors => "混老頭",
        // 小三元
        WinningHandKind::LittleThreeDragons => "小三元",
        // 混一色
        WinningHandKind::HalfFlush => {
            if has_openned {
                "混一色（鳴）"
            } else {
                "混一色"
            }
        }
        // 清一色
        WinningHandKind::Flush => {
            if has_openned {
                "清一色（鳴）"
            } else {
                "清一色"
            }
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
