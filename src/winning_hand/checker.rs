/// 役を判定する
use std::collections::HashMap;

use strum::{EnumCount, IntoEnumIterator};

use crate::hand_info::block::BlockProperty;
use crate::hand_info::hand_analyzer::HandAnalyzer;
use crate::hand_info::status::Status;
use crate::winning_hand::name::*;
use crate::settings::*;
use crate::tile::{Dragon, Tile};

pub fn check(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> HashMap<WinningHandKind, (&'static str, bool, u32)> {
    let mut result = HashMap::with_capacity(WinningHandKind::COUNT);
    for hand_kind in WinningHandKind::iter() {
        result.insert(hand_kind, ("Unknown", false, 0));
    }

    // 立直
    result.insert(
        WinningHandKind::ReadyHand,
        check_ready_hand(hand, status, settings),
    );
    // 七対子
    result.insert(
        WinningHandKind::SevenPairs,
        check_seven_pairs(hand, status, settings),
    );
    // 流し満貫
    result.insert(
        WinningHandKind::NagashiMangan,
        check_nagashi_mangan(hand, status, settings),
    );
    // 門前清自摸和
    result.insert(
        WinningHandKind::SelfPick,
        check_self_pick(hand, status, settings),
    );
    // 一発
    result.insert(
        WinningHandKind::OneShot,
        check_one_shot(hand, status, settings),
    );
    // 海底撈月
    result.insert(
        WinningHandKind::LastTileFromTheWall,
        check_last_tile_from_the_wall(hand, status, settings),
    );
    // 河底撈魚
    result.insert(
        WinningHandKind::LastDiscard,
        check_last_discard(hand, status, settings),
    );
    // 嶺上開花
    result.insert(
        WinningHandKind::DeadWallDraw,
        check_dead_wall_draw(hand, status, settings),
    );
    // 搶槓
    result.insert(
        WinningHandKind::RobbingAQuad,
        check_robbing_a_quad(hand, status, settings),
    );
    // ダブル立直
    result.insert(
        WinningHandKind::DoubleReady,
        check_double_ready(hand, status, settings),
    );
    // 平和
    result.insert(
        WinningHandKind::NoPointsHand,
        check_no_points_hand(hand, status, settings),
    );
    // 一盃口
    result.insert(
        WinningHandKind::OneSetOfIdenticalSequences,
        check_one_set_of_identical_sequences(hand, status, settings),
    );
    // 三色同順
    result.insert(
        WinningHandKind::ThreeColourStraight,
        check_three_colour_straight(hand, status, settings),
    );
    // 一気通貫
    result.insert(
        WinningHandKind::Straight,
        check_straight(hand, status, settings),
    );
    // 二盃口
    result.insert(
        WinningHandKind::TwoSetsOfIdenticalSequences,
        check_two_sets_of_identical_sequences(hand, status, settings),
    );
    // 対々和
    result.insert(
        WinningHandKind::AllTripletHand,
        check_all_triplet_hand(hand, status, settings),
    );
    // 三暗刻
    result.insert(
        WinningHandKind::ThreeClosedTriplets,
        check_three_closed_triplets(hand, status, settings),
    );
    // 三色同刻
    result.insert(
        WinningHandKind::ThreeColourTriplets,
        check_three_colour_triplets(hand, status, settings),
    );
    // 断么九
    result.insert(
        WinningHandKind::AllSimples,
        check_all_simples(hand, status, settings),
    );
    // 役牌（自風牌）
    result.insert(
        WinningHandKind::HonorTilesPlayersWind,
        check_honor_tiles_players_wind(hand, status, settings),
    );
    // 役牌（場風牌）
    result.insert(
        WinningHandKind::HonorTilesPrevailingWind,
        check_honor_tiles_prevailing_wind(hand, status, settings),
    );
    // 役牌（白）
    result.insert(
        WinningHandKind::HonorTilesWhiteDragon,
        check_honor_tiles_white_dragon(hand, status, settings),
    );
    // 役牌（發）
    result.insert(
        WinningHandKind::HonorTilesGreenDragon,
        check_honor_tiles_green_dragon(hand, status, settings),
    );
    // 役牌（中）
    result.insert(
        WinningHandKind::HonorTilesRedDragon,
        check_honor_tiles_red_dragon(hand, status, settings),
    );
    // 混全帯么九
    result.insert(
        WinningHandKind::TerminalOrHonorInEachSet,
        check_terminal_or_honor_in_each_set(hand, status, settings),
    );
    result.insert(
        WinningHandKind::TerminalInEachSet,
        check_terminal_in_each_set(hand, status, settings),
    );
    // 混老頭
    result.insert(
        WinningHandKind::AllTerminalsAndHonors,
        check_all_terminals_and_honors(hand, status, settings),
    );
    // 小三元
    result.insert(
        WinningHandKind::LittleThreeDragons,
        check_little_three_dragons(hand, status, settings),
        // 純全帯么九
    );
    // 混一色
    result.insert(
        WinningHandKind::HalfFlush,
        check_half_flush(hand, status, settings),
    );
    // 清一色
    result.insert(WinningHandKind::Flush, check_flush(hand, status, settings));
    // 国士無双
    result.insert(
        WinningHandKind::ThirteenOrphans,
        check_thirteen_orphans(hand, status, settings),
    );
    // 四暗刻
    result.insert(
        WinningHandKind::FourConcealedTriplets,
        check_four_concealed_triplets(hand, status, settings),
    );
    // 大三元
    result.insert(
        WinningHandKind::BigThreeDragons,
        check_big_three_dragons(hand, status, settings),
    );
    // 小四喜
    result.insert(
        WinningHandKind::LittleFourWinds,
        check_little_four_winds(hand, status, settings),
    );
    // 大四喜
    result.insert(
        WinningHandKind::BigFourWinds,
        check_big_four_winds(hand, status, settings),
    );
    // 字一色
    result.insert(
        WinningHandKind::AllHonors,
        check_all_honors(hand, status, settings),
    );
    // 清老頭
    result.insert(
        WinningHandKind::AllTerminals,
        check_all_terminals(hand, status, settings),
    );
    // 緑一色
    result.insert(
        WinningHandKind::AllGreen,
        check_all_green(hand, status, settings),
    );
    // 九蓮宝燈
    result.insert(
        WinningHandKind::NineGates,
        check_nine_gates(hand, status, settings),
    );
    // 四槓子
    result.insert(
        WinningHandKind::FourKans,
        check_four_kans(hand, status, settings),
    );
    // 天和
    result.insert(
        WinningHandKind::HeavenlyHand,
        check_heavenly_hand(hand, status, settings),
    );
    // 地和
    result.insert(
        WinningHandKind::HandOfEarth,
        check_hand_of_earth(hand, status, settings),
    );

    return result;
}

/// 和了しているか否か
fn has_won(hand: &HandAnalyzer) -> bool {
    hand.shanten == -1
}

/// 立直
fn check_ready_hand(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::ReadyHand,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    if status.has_claimed_open {
        return (name, false, 0);
    }
    return if status.has_claimed_ready {
        (name, true, 1)
    } else {
        (name, false, 0)
    };
}
/// 七対子
fn check_seven_pairs(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::SevenPairs,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    return if hand.form == WinningHandForm::SevenPairs {
        (name, true, 2)
    } else {
        (name, false, 0)
    };
}
/// 流し満貫
fn check_nagashi_mangan(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::NagashiMangan,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 門前清自摸和
fn check_self_pick(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::SelfPick,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    if !status.has_claimed_open && status.is_self_picked {
        return (name, true, 1);
    }
    return (name, false, 0);
}
/// 一発
fn check_one_shot(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::OneShot,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    if !check_ready_hand(hand, status, settings).1 {
        return (name, false, 0);
    }
    if status.is_one_shot {
        return (name, true, 1);
    }
    return (name, false, 0);
}
/// 海底撈月
fn check_last_tile_from_the_wall(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::LastTileFromTheWall,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 河底撈魚
fn check_last_discard(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::LastDiscard,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 嶺上開花
fn check_dead_wall_draw(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::DeadWallDraw,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 搶槓
fn check_robbing_a_quad(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::RobbingAQuad,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// ダブル立直
fn check_double_ready(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::DoubleReady,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 平和
fn check_no_points_hand(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::NoPointsHand,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 一盃口
fn check_one_set_of_identical_sequences(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::OneSetOfIdenticalSequences,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    // 鳴いていたら一盃口は成立しない
    if status.has_claimed_open {
        return (name, false, 0);
    }
    // 順子が2つ以上なければ一盃口はありえない
    if hand.sequential3.len() < 2 {
        return (name, false, 0);
    }
    for i in 0..hand.sequential3.len() - 1 {
        if let Some(v) = hand.sequential3.get(i) {
            for j in i + 1..hand.sequential3.len() {
                if let Some(v2) = hand.sequential3.get(j) {
                    if *v == *v2 {
                        return (name, true, 1);
                    }
                }
            }
        }
    }
    return (name, false, 0);
}
/// 三色同順
fn check_three_colour_straight(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::ThreeColourStraight,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 一気通貫
fn check_straight(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::Straight,
        status.has_claimed_open,
        settings.display_lang,
    );

    if !has_won(hand) {
        return (name, false, 0);
    }
    let mut m = [false; 3];
    let mut p = [false; 3];
    let mut s = [false; 3];

    for v in &hand.sequential3 {
        match v.get() {
            [Tile::M1, Tile::M2, Tile::M3] => m[0] = true,
            [Tile::M4, Tile::M5, Tile::M6] => m[1] = true,
            [Tile::M7, Tile::M8, Tile::M9] => m[2] = true,
            [Tile::P1, Tile::P2, Tile::P3] => p[0] = true,
            [Tile::P4, Tile::P5, Tile::P6] => p[1] = true,
            [Tile::P7, Tile::P8, Tile::P9] => p[2] = true,
            [Tile::S1, Tile::S2, Tile::S3] => s[0] = true,
            [Tile::S4, Tile::S5, Tile::S6] => s[1] = true,
            [Tile::S7, Tile::S8, Tile::S9] => s[2] = true,
            _ => {}
        }
    }

    if (m[0] && m[1] && m[2]) || (p[0] && p[1] && p[2]) || (s[0] && s[1] && s[2]) {
        if status.has_claimed_open {
            return (name, true, 1);
        } else {
            return (name, true, 2);
        }
    }
    return (name, false, 0);
}
/// 二盃口
fn check_two_sets_of_identical_sequences(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::TwoSetsOfIdenticalSequences,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 対々和
fn check_all_triplet_hand(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::AllTripletHand,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    if hand.same3.len() == 4 && hand.same2.len() == 1 {
        return (name, true, 2);
    }
    return (name, false, 0);
}
/// 三暗刻
fn check_three_closed_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::ThreeClosedTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 三色同刻
fn check_three_colour_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::ThreeColourTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 断么九
fn check_all_simples(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::AllSimples,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    // 喰いタンなしなら鳴いている時点で抜ける
    if !settings.openned_all_simples && status.has_claimed_open {
        return (name, false, 0);
    }
    let mut has_1_9_honor = false;
    // 面子

    // 刻子
    for same in &hand.same3 {
        if same.has_1_or_9() || same.has_honor() {
            has_1_9_honor = true;
        }
    }
    // 順子
    for seq in &hand.sequential3 {
        if seq.has_1_or_9() {
            has_1_9_honor = true;
        }
    }

    // 雀頭
    for head in &hand.same2 {
        if head.has_1_or_9() || head.has_honor() {
            has_1_9_honor = true;
        }
    }

    if has_1_9_honor {
        return (name, false, 0);
    }

    return (name, true, 1);
}
/// 役牌（自風牌）
fn check_honor_tiles_players_wind(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HonorTilesPlayersWind,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    let mut has_player_wind = false;
    // 刻子
    for same in &hand.same3 {
        if same.has_wind(status.player_wind) {
            has_player_wind = true;
        }
    }

    if has_player_wind {
        (name, true, 1)
    } else {
        (name, false, 0)
    }
}
/// 役牌（場風牌）
fn check_honor_tiles_prevailing_wind(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HonorTilesPrevailingWind,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    let mut has_prevailing_wind = false;
    // 刻子
    for same in &hand.same3 {
        if same.has_wind(status.prevailing_wind) {
            has_prevailing_wind = true;
        }
    }

    if has_prevailing_wind {
        (name, true, 1)
    } else {
        (name, false, 0)
    }
}

/// 面子に三元牌の順子が含まれるか調べる
fn check_honor_tiles_dragons(hand: &HandAnalyzer, dragon: Dragon) -> bool {
    if !has_won(hand) {
        return false;
    }
    let mut has_dragon = false;
    // 刻子
    for same in &hand.same3 {
        if same.has_dragon(dragon) {
            has_dragon = true;
        }
    }

    if has_dragon {
        true
    } else {
        false
    }
}

/// 役牌（白）
fn check_honor_tiles_white_dragon(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HonorTilesWhiteDragon,
        status.has_claimed_open,
        settings.display_lang,
    );
    if check_honor_tiles_dragons(hand, Dragon::White) {
        (name, true, 1)
    } else {
        (name, false, 0)
    }
}
/// 役牌（發）
fn check_honor_tiles_green_dragon(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HonorTilesGreenDragon,
        status.has_claimed_open,
        settings.display_lang,
    );
    if check_honor_tiles_dragons(hand, Dragon::Green) {
        (name, true, 1)
    } else {
        (name, false, 0)
    }
}
/// 役牌（中）
fn check_honor_tiles_red_dragon(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HonorTilesRedDragon,
        status.has_claimed_open,
        settings.display_lang,
    );
    if check_honor_tiles_dragons(hand, Dragon::Red) {
        (name, true, 1)
    } else {
        (name, false, 0)
    }
}
/// 混全帯么九
fn check_terminal_or_honor_in_each_set(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::TerminalOrHonorInEachSet,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }

    // 混老頭とは複合しないため、必ず順子が含まれる
    if hand.sequential3.len() == 0 {
        return (name, false, 0);
    }

    let mut no_1_9_honor = false;
    // 純全帯么九とは複合しないため、必ず三元牌が含まれる
    let mut has_honor = false;

    // 面子

    // 刻子
    for same in &hand.same3 {
        if !same.has_1_or_9() && !same.has_honor() {
            no_1_9_honor = true;
        }

        if same.has_honor() {
            has_honor = true;
        }
    }
    // 順子
    for seq in &hand.sequential3 {
        if !seq.has_1_or_9() {
            no_1_9_honor = true;
        }
    }

    // 雀頭
    for head in &hand.same2 {
        if !head.has_1_or_9() && !head.has_honor() {
            no_1_9_honor = true;
        }
        if head.has_honor() {
            has_honor = true;
        }
    }

    if no_1_9_honor || !has_honor {
        return (name, false, 0);
    }
    if status.has_claimed_open {
        return (name, true, 1);
    }
    return (name, true, 2);
}
/// 純全帯么九
fn check_terminal_in_each_set(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::TerminalInEachSet,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    // 清老頭とは複合しないため、必ず順子が含まれる
    if hand.sequential3.len() == 0 {
        return (name, false, 0);
    }

    let mut no_1_9 = false;
    // 面子

    // 刻子
    for same in &hand.same3 {
        if !same.has_1_or_9() {
            no_1_9 = true;
        }
    }
    // 順子
    for seq in &hand.sequential3 {
        if !seq.has_1_or_9() {
            no_1_9 = true;
        }
    }

    // 雀頭
    for head in &hand.same2 {
        if !head.has_1_or_9() {
            no_1_9 = true;
        }
    }

    if no_1_9 {
        return (name, false, 0);
    }
    if status.has_claimed_open {
        return (name, true, 2);
    }
    return (name, true, 3);
}
/// 混老頭
fn check_all_terminals_and_honors(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::AllTerminalsAndHonors,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 小三元
fn check_little_three_dragons(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::LittleThreeDragons,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 混一色
fn check_half_flush(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HalfFlush,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 清一色
fn check_flush(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::Flush,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 国士無双
fn check_thirteen_orphans(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::ThirteenOrphans,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    return if hand.form == WinningHandForm::ThirteenOrphens {
        (name, true, 13)
    } else {
        (name, false, 0)
    };
}
/// 四暗刻
fn check_four_concealed_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::FourConcealedTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 大三元
fn check_big_three_dragons(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::BigThreeDragons,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 小四喜
fn check_little_four_winds(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::LittleFourWinds,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 大四喜
fn check_big_four_winds(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::BigFourWinds,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 字一色
fn check_all_honors(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::AllHonors,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 清老頭
fn check_all_terminals(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::AllTerminals,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 緑一色
fn check_all_green(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::AllGreen,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 九蓮宝燈
fn check_nine_gates(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::NineGates,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 四槓子
fn check_four_kans(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::FourKans,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 天和
fn check_heavenly_hand(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HeavenlyHand,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 地和
fn check_hand_of_earth(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get_winning_hand_name(
        WinningHandKind::HandOfEarth,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}

/// ユニットテスト
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{hand::*, tile::*};

    #[test]
    /// 七対子で和了った
    fn test_win_by_seven_pairs() {
        let test_str = "1122m3344p5566s1z 1z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_seven_pairs(&test_analyzer, &status, &settings),
            ("七対子", true, 2)
        );
    }

    #[test]
    /// 国士無双で和了った
    fn test_win_by_thirteen_orphens() {
        let test_str = "19m19p19s1234567z 1m";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_thirteen_orphans(&test_analyzer, &status, &settings),
            ("国士無双", true, 13)
        );
    }

    #[test]
    /// 立直で和了った
    fn test_win_by_ready_hand() {
        let test_str = "123m45678p999s11z 9p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_ready = true;
        assert_eq!(
            check_ready_hand(&test_analyzer, &status, &settings),
            ("立直", true, 1)
        );
    }
    #[test]
    /// 立直に一発が付いた
    fn test_win_by_one_shot() {
        let test_str = "123m45678p999s11z 9p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_ready = true;
        status.is_one_shot = true;
        assert_eq!(
            check_one_shot(&test_analyzer, &status, &settings),
            ("一発", true, 1)
        );
    }
    #[test]
    /// 門前清自摸和で和了った
    fn test_win_by_self_pick() {
        let test_str = "123m45678p999s11z 9p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.is_self_picked = true;
        assert_eq!(
            check_self_pick(&test_analyzer, &status, &settings),
            ("門前清自摸和", true, 1)
        );
    }
    #[test]
    /// 鳴いている場合は門前清自摸和は付かない
    fn test_not_win_by_self_pick_with_claiming_open() {
        let test_str = "123m45678p999s11z 9p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.is_self_picked = true;
        status.has_claimed_open = true;
        assert_eq!(
            check_self_pick(&test_analyzer, &status, &settings),
            ("門前清自摸和", false, 0)
        );
    }
    #[test]
    /// 断么九で和了った（喰い断あり鳴きなし）
    fn test_win_by_all_simples_open_rule_close_hand() {
        let test_str = "222456m777p56s88s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Settings::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九", true, 1)
        );
    }
    #[test]
    /// 么九牌ありでは断么九にならない（一）
    fn test_not_win_by_all_simples_with_1() {
        let test_str = "111456m777p56s88s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Settings::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九", false, 0)
        );
    }
    #[test]
    /// 么九牌ありでは断么九にならない（九）
    fn test_not_win_by_all_simples_with_9() {
        let test_str = "222456m777p5699s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Settings::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九", false, 0)
        );
    }
    #[test]
    /// 么九牌ありでは断么九にならない（字牌）
    fn test_not_win_by_all_simples_with_honor() {
        let test_str = "222456m56s88s111z 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Settings::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九", false, 0)
        );
    }
    #[test]
    /// 断么九で和了った（喰い断あり鳴きあり）
    fn test_win_by_all_simples_open_rule_open_hand() {
        let test_str = "234m567m234p345s3s 3s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Settings::new();
        // 喰い断あり鳴きあり
        rules.openned_all_simples = true;
        status.has_claimed_open = true;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九", true, 1)
        );
    }
    #[test]
    /// 断么九で和了った（喰い断なし鳴きなし）
    fn test_win_by_all_simples_close_rule_close_hand() {
        let test_str = "678m23455p33345ss 5p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Settings::new();
        // 喰い断なし鳴きなし
        rules.openned_all_simples = false;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九", true, 1)
        );
    }
    #[test]
    /// 断么九で和了った（喰い断なし鳴きあり）->役無し
    fn test_win_by_all_simples_close_rule_open_hand() {
        let test_str = "222m456m777p56s88s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Settings::new();
        // 喰い断なし鳴きあり（役無し）
        rules.openned_all_simples = false;
        status.has_claimed_open = true;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九", false, 0)
        );
    }
    #[test]
    /// 一盃口で和了った
    fn test_win_by_one_set_of_identical_sequences() {
        let test_str = "112233m456p456s7z 7z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_one_set_of_identical_sequences(&test_analyzer, &status, &settings),
            ("一盃口", true, 1)
        );
    }
    #[test]
    /// 一盃口で和了った（鳴きあり）→役なし
    fn test_no_win_by_one_set_of_identical_sequences_with_openned() {
        let test_str = "112233m456p456s7z 7z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_one_set_of_identical_sequences(&test_analyzer, &status, &settings),
            ("一盃口", false, 0)
        );
    }
    #[test]
    /// 自風で和了った
    fn test_win_by_honor_tiles_players_wind() {
        let test_str = "222m456m777p5s 222z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_players_wind(&test_analyzer, &status, &settings),
            ("役牌（自風牌）", true, 1)
        );
    }
    #[test]
    /// 場風で和了った
    fn test_win_by_honor_tiles_prevailing_wind() {
        let test_str = "222m456m777p5s 111z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_prevailing_wind(&test_analyzer, &status, &settings),
            ("役牌（場風牌）", true, 1)
        );
    }
    #[test]
    /// 三元牌（白）で和了った
    fn test_win_by_honor_tiles_white_dragon() {
        let test_str = "222m456m777p5s 555z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_white_dragon(&test_analyzer, &status, &settings),
            ("役牌（白）", true, 1)
        );
    }
    #[test]
    /// 三元牌（發）で和了った
    fn test_win_by_honor_tiles_green_dragon() {
        let test_str = "222m456m777p5s 666z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_green_dragon(&test_analyzer, &status, &settings),
            ("役牌（發）", true, 1)
        );
    }
    #[test]
    /// 三元牌（中）で和了った
    fn test_win_by_honor_tiles_red_dragon() {
        let test_str = "222m456m777p5s 777z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_red_dragon(&test_analyzer, &status, &settings),
            ("役牌（中）", true, 1)
        );
    }

    #[test]
    /// 混全帯么九で和了った
    fn test_terminal_or_honor_in_each_set() {
        let test_str = "123999m111p79s44z 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings),
            ("混全帯么九", true, 2)
        );
    }
    #[test]
    /// 混全帯么九で和了った（食い下がり1翻）
    fn test_terminal_or_honor_in_each_set_open() {
        let test_str = "123m111p79s44z 789m 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings),
            ("混全帯么九（鳴）", true, 1)
        );
    }
    #[test]
    /// 純全帯么九で和了った
    fn test_terminal_in_each_set() {
        let test_str = "123999m11p11179s 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings),
            ("純全帯么九", true, 3)
        );
    }
    #[test]
    /// 純全帯么九で和了った（食い下がり2翻）
    fn test_terminal_in_each_set_open() {
        let test_str = "123m111p7999s 789m 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings),
            ("純全帯么九（鳴）", true, 2)
        );
    }

    #[test]
    /// 混全帯么九は純全帯么九と複合しない
    fn test_terminal_or_honor_in_each_set_does_not_combined_with_terminal_in_each_set() {
        let test_str = "111789m111p99s11z 1z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings).1,
            true
        );
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings).1,
            false
        );
    }
    #[test]
    /// 純全帯么九は混全帯么九と複合しない
    fn test_terminal_in_each_set_does_not_combined_with_terminal_or_honor_in_each_set() {
        let test_str = "111789m111p1199s 9s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings).1,
            false
        );
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings).1,
            true
        );
    }
    #[test]
    /// 対々和で和了った
    fn test_all_triplet_hand() {
        let test_str = "777m333p22z 555m 999s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_all_triplet_hand(&test_analyzer, &status, &settings),
            ("対々和", true, 2)
        );
    }

    #[test]
    /// 一気通貫で和了った
    fn test_straight() {
        let test_str = "123456789m78p22z 9p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_straight(&test_analyzer, &status, &settings),
            ("一気通貫", true, 2)
        );
    }

    #[test]
    /// 一気通貫で和了った（食い下がり1翻）
    fn test_straight_open() {
        let test_str = "123m1p123s 456s 789s 1p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_straight(&test_analyzer, &status, &settings),
            ("一気通貫（鳴）", true, 1)
        );
    }
}
