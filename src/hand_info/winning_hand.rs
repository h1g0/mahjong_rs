use crate::board::Rules;
use crate::hand_info::hand_analyzer::HandAnalyzer;
use crate::hand_info::status::Status;
use crate::tile::Dragon;

/// 役を判定する
use std::collections::HashMap;

use super::block::BlockProperty;

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

/// 役の名前
///
/// <https://en.wikipedia.org/wiki/Japanese_Mahjong_yaku>による英語名
pub const HAND_NAME: [&'static str; 42] = [
    // 立直
    "ready_hand",
    // 七対子
    "seven_pairs",
    // 流し満貫
    "nagashi_mangan",
    // 門前清自摸和
    "self_pick",
    // 一発
    "one_shot",
    // 海底撈月
    "last_tile_from_the_wall",
    // 河底撈魚
    "last_discard",
    // 嶺上開花
    "dead_wall_draw",
    // 搶槓
    "robbing_a_quad",
    // ダブル立直
    "double_ready",
    // 平和
    "no_points_hand",
    // 一盃口
    "one_set_of_identical_sequences",
    // 三色同順
    "three_colour_straight",
    // 一気通貫
    "straight",
    // 二盃口
    "two_sets_of_identical_sequences",
    // 対々和
    "all_triplet_hand",
    // 三暗刻
    "three_closed_triplets",
    // 三色同刻
    "three_colour_triplets",
    // 断么九
    "all_simples",
    // 役牌（自風牌）
    "honor_tiles_players_wind",
    // 役牌（場風牌）
    "honor_tiles_prevailing_wind",
    // 役牌（白）
    "honor_tiles_white_dragon",
    // 役牌（發）
    "honor_tiles_green_dragon",
    // 役牌（中）
    "honor_tiles_red_dragon",
    // 混全帯么九
    "terminal_or_honor_in_each_set",
    // 純全帯么九
    "terminal_in_each_set",
    // 混老頭
    "all_terminals_and_honors",
    // 小三元
    "little_three_dragons",
    // 混一色
    "half_flush",
    // 清一色
    "flush",
    // 国士無双
    "thirteen_orphans",
    // 四暗刻
    "four_concealed_triplets",
    // 大三元
    "big_three_dragons",
    // 小四喜
    "little_four_winds",
    // 大四喜
    "big_four_winds",
    // 字一色
    "all_honors",
    // 清老頭
    "all_terminals",
    // 緑一色
    "all_green",
    // 九蓮宝燈
    "nine_gates",
    // 四槓子
    "four_kans",
    // 天和
    "heavenly_hand",
    // 地和
    "hand_of_earth",
];

/// 食い下がりした場合に役名に付く接尾辞
const OPEN_SUFFIX: &str = "（鳴）";

pub fn check<'a, 'b>(
    hand: &'a HandAnalyzer,
    status: &'b Status,
    rules: &'b Rules,
) -> HashMap<&'a str, (String, bool, u32)> {
    let mut result = HashMap::new();
    for i in 0..HAND_NAME.len() {
        result.insert(HAND_NAME[i], ("unknown".to_string(), false, 0));
    }

    // 立直
    result.insert("ready_hand", check_ready_hand(hand, status));
    // 七対子
    result.insert("seven_pairs", check_seven_pairs(hand));
    // 流し満貫
    result.insert("nagashi_mangan", check_nagashi_mangan(hand, status));
    // 門前清自摸和
    result.insert("self_pick", check_self_pick(hand, status));
    // 一発
    result.insert("one_shot", check_one_shot(hand, status));
    // 海底撈月
    result.insert(
        "last_tile_from_the_wall",
        check_last_tile_from_the_wall(hand, status),
    );
    // 河底撈魚
    result.insert("last_discard", check_last_discard(hand, status));
    // 嶺上開花
    result.insert("dead_wall_draw", check_dead_wall_draw(hand, status));
    // 搶槓
    result.insert("robbing_a_quad", check_robbing_a_quad(hand, status));
    // ダブル立直
    result.insert("double_ready", check_double_ready(hand, status));
    // 平和
    result.insert("no_points_hand", check_no_points_hand(hand, status));
    // 一盃口
    result.insert(
        "one_set_of_identical_sequences",
        check_one_set_of_identical_sequences(hand, status),
    );
    // 三色同順
    result.insert(
        "three_colour_straight",
        check_three_colour_straight(hand, status),
    );
    // 一気通貫
    result.insert("straight", check_straight(hand, status));
    // 二盃口
    result.insert(
        "two_sets_of_identical_sequences",
        check_two_sets_of_identical_sequences(hand, status),
    );
    // 対々和
    result.insert("all_triplet_hand", check_all_triplet_hand(hand, status));
    // 三暗刻
    result.insert(
        "three_closed_triplets",
        check_three_closed_triplets(hand, status),
    );
    // 三色同刻
    result.insert(
        "three_colour_triplets",
        check_three_colour_triplets(hand, status),
    );
    // 断么九
    result.insert("all_simples", check_all_simples(hand, status, rules));
    // 役牌（自風牌）
    result.insert(
        "honor_tiles_players_wind",
        check_honor_tiles_players_wind(hand, status),
    );
    // 役牌（場風牌）
    result.insert(
        "honor_tiles_prevailing_wind",
        check_honor_tiles_prevailing_wind(hand, status),
    );
    // 役牌（白）
    result.insert(
        "honor_tiles_white_dragon",
        check_honor_tiles_white_dragon(hand),
    );
    // 役牌（發）
    result.insert(
        "honor_tiles_green_dragon",
        check_honor_tiles_green_dragon(hand),
    );
    // 役牌（中）
    result.insert(
        "honor_tiles_red_dragons",
        check_honor_tiles_red_dragon(hand),
    );
    // 混全帯么九
    result.insert(
        "terminal_or_honor_in_each_set",
        check_terminal_or_honor_in_each_set(hand, status),
    );
    // 純全帯么九
    result.insert(
        "terminal_in_each_set",
        check_terminal_in_each_set(hand, status),
    );
    // 混老頭
    result.insert(
        "all_terminals_and_honors",
        check_all_terminals_and_honors(hand, status),
    );
    // 小三元
    result.insert(
        "little_three_dragons",
        check_little_three_dragons(hand, status),
    );
    // 混一色
    result.insert("half_flush", check_half_flush(hand, status));
    // 清一色
    result.insert("flush", check_flush(hand, status));
    // 国士無双
    result.insert("thirteen_orphans", check_thirteen_orphans(hand));
    // 四暗刻
    result.insert(
        "four_concealed_triplets",
        check_four_concealed_triplets(hand, status),
    );
    // 大三元
    result.insert("big_three_dragons", check_big_three_dragons(hand, status));
    // 小四喜
    result.insert("little_four_winds", check_little_four_winds(hand, status));
    // 大四喜
    result.insert("big_four_winds", check_big_four_winds(hand, status));
    // 字一色
    result.insert("all_honors", check_all_honors(hand, status));
    // 清老頭
    result.insert("all_terminals", check_all_terminals(hand, status));
    // 緑一色
    result.insert("all_green", check_all_green(hand, status));
    // 九蓮宝燈
    result.insert("nine_gates", check_nine_gates(hand, status));
    // 四槓子
    result.insert("four_kans", check_four_kans(hand, status));
    // 天和
    result.insert("heavenly_hand", check_heavenly_hand(hand, status));
    // 地和
    result.insert("hand_of_earth", check_hand_of_earth(hand, status));

    return result;
}

/// 和了しているか否か
fn has_won(hand: &HandAnalyzer) -> bool {
    hand.shanten == -1
}

/// 立直
fn check_ready_hand(hand: &HandAnalyzer, status: &Status) -> (String, bool, u32) {
    let name = "立直".to_string();
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
fn check_seven_pairs(hand: &HandAnalyzer) -> (String, bool, u32) {
    let name = "七対子".to_string();
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
fn check_nagashi_mangan(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("流し満貫".to_string(), false, 0);
    }
    unimplemented!();
}
/// 門前清自摸和
fn check_self_pick(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("門前清自摸和".to_string(), false, 0);
    }
    unimplemented!();
}
/// 一発
fn check_one_shot(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("一発".to_string(), false, 0);
    }
    unimplemented!();
}
/// 海底撈月
fn check_last_tile_from_the_wall(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("海底撈月".to_string(), false, 0);
    }
    unimplemented!();
}
/// 河底撈魚
fn check_last_discard(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("河底撈魚".to_string(), false, 0);
    }
    unimplemented!();
}
/// 嶺上開花
fn check_dead_wall_draw(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("嶺上開花".to_string(), false, 0);
    }
    unimplemented!();
}
/// 搶槓
fn check_robbing_a_quad(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("搶槓".to_string(), false, 0);
    }
    unimplemented!();
}
/// ダブル立直
fn check_double_ready(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("ダブル立直".to_string(), false, 0);
    }
    unimplemented!();
}
/// 平和
fn check_no_points_hand(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("平和".to_string(), false, 0);
    }
    unimplemented!();
}
/// 一盃口
fn check_one_set_of_identical_sequences(
    hand: &HandAnalyzer,
    status: &Status,
) -> (String, bool, u32) {
    let name = "一盃口".to_string();
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
fn check_three_colour_straight(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("三色同順".to_string(), false, 0);
    }
    unimplemented!();
}
/// 一気通貫
fn check_straight(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("一気通貫".to_string(), false, 0);
    }
    unimplemented!();
}
/// 二盃口
fn check_two_sets_of_identical_sequences(
    hand: &HandAnalyzer,
    _status: &Status,
) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("二盃口".to_string(), false, 0);
    }
    unimplemented!();
}
/// 対々和
fn check_all_triplet_hand(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("対々和".to_string(), false, 0);
    }
    unimplemented!();
}
/// 三暗刻
fn check_three_closed_triplets(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("三暗刻".to_string(), false, 0);
    }
    unimplemented!();
}
/// 三色同刻
fn check_three_colour_triplets(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("三色同刻".to_string(), false, 0);
    }
    unimplemented!();
}
/// 断么九
fn check_all_simples(hand: &HandAnalyzer, status: &Status, rules: &Rules) -> (String, bool, u32) {
    let name = "断么九".to_string();
    if !has_won(hand) {
        return (name, false, 0);
    }
    // 喰いタンなしなら鳴いている時点で抜ける
    if !rules.openned_all_simples && status.has_claimed_open {
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
fn check_honor_tiles_players_wind(hand: &HandAnalyzer, status: &Status) -> (String, bool, u32) {
    let name = "役牌（自風牌）".to_string();
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
fn check_honor_tiles_prevailing_wind(hand: &HandAnalyzer, status: &Status) -> (String, bool, u32) {
    let name = "役牌（場風牌）".to_string();
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
fn check_honor_tiles_white_dragon(hand: &HandAnalyzer) -> (String, bool, u32) {
    let name = "役牌（白）".to_string();
    if check_honor_tiles_dragons(hand, Dragon::White) {
        (name, true, 1)
    } else {
        (name, false, 0)
    }
}
/// 役牌（發）
fn check_honor_tiles_green_dragon(hand: &HandAnalyzer) -> (String, bool, u32) {
    let name = "役牌（發）".to_string();
    if check_honor_tiles_dragons(hand, Dragon::Green) {
        (name, true, 1)
    } else {
        (name, false, 0)
    }
}
/// 役牌（中）
fn check_honor_tiles_red_dragon(hand: &HandAnalyzer) -> (String, bool, u32) {
    let name = "役牌（中）".to_string();
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
) -> (String, bool, u32) {
    let name = "混全帯么九".to_string();
    if !has_won(hand) {
        return (name, false, 0);
    }

    // 混老頭とは複合しないため、必ず順子が含まれる
    if hand.sequential3.len() == 0 {
        return (name, false, 0);
    }

    let mut no_1_9_honor = false;
    // 面子

    // 刻子
    for same in &hand.same3 {
        if !same.has_1_or_9() && !same.has_honor() {
            no_1_9_honor = true;
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
    }

    if no_1_9_honor {
        return (name, false, 0);
    }
    if status.has_claimed_open {
        return (name + OPEN_SUFFIX, true, 1);
    }
    return (name, true, 2);
}
/// 純全帯么九
fn check_terminal_in_each_set(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("純全帯么九".to_string(), false, 0);
    }
    unimplemented!();
}
/// 混老頭
fn check_all_terminals_and_honors(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("混老頭".to_string(), false, 0);
    }
    unimplemented!();
}
/// 小三元
fn check_little_three_dragons(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("小三元".to_string(), false, 0);
    }
    unimplemented!();
}
/// 混一色
fn check_half_flush(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("混一色".to_string(), false, 0);
    }
    unimplemented!();
}
/// 清一色
fn check_flush(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("清一色".to_string(), false, 0);
    }
    unimplemented!();
}
/// 国士無双
fn check_thirteen_orphans(hand: &HandAnalyzer) -> (String, bool, u32) {
    let name = "国士無双".to_string();
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
fn check_four_concealed_triplets(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("四暗刻".to_string(), false, 0);
    }
    unimplemented!();
}
/// 大三元
fn check_big_three_dragons(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("大三元".to_string(), false, 0);
    }
    unimplemented!();
}
/// 小四喜
fn check_little_four_winds(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("小四喜".to_string(), false, 0);
    }
    unimplemented!();
}
/// 大四喜
fn check_big_four_winds(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("大四喜".to_string(), false, 0);
    }
    unimplemented!();
}
/// 字一色
fn check_all_honors(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("字一色".to_string(), false, 0);
    }
    unimplemented!();
}
/// 清老頭
fn check_all_terminals(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("清老頭".to_string(), false, 0);
    }
    unimplemented!();
}
/// 緑一色
fn check_all_green(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("緑一色".to_string(), false, 0);
    }
    unimplemented!();
}
/// 九蓮宝燈
fn check_nine_gates(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("九蓮宝燈".to_string(), false, 0);
    }
    unimplemented!();
}
/// 四槓子
fn check_four_kans(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("四槓子".to_string(), false, 0);
    }
    unimplemented!();
}
/// 天和
fn check_heavenly_hand(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("天和".to_string(), false, 0);
    }
    unimplemented!();
}
/// 地和
fn check_hand_of_earth(hand: &HandAnalyzer, _status: &Status) -> (String, bool, u32) {
    if !has_won(hand) {
        return ("地和".to_string(), false, 0);
    }
    unimplemented!();
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
        assert_eq!(
            check_seven_pairs(&test_analyzer),
            ("七対子".to_string(), true, 2)
        );
    }

    #[test]
    /// 国士無双で和了った
    fn test_win_by_thirteen_orphens() {
        let test_str = "19m19p19s1234567z 1m";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        assert_eq!(
            check_thirteen_orphans(&test_analyzer),
            ("国士無双".to_string(), true, 13)
        );
    }

    #[test]
    /// 立直で和了った
    fn test_win_by_ready_hand() {
        let test_str = "123m45678p999s11z 9p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        status.has_claimed_ready = true;
        assert_eq!(
            check_ready_hand(&test_analyzer, &status),
            ("立直".to_string(), true, 1)
        );
    }

    #[test]
    /// 断么九で和了った（喰い断あり鳴きなし）
    fn test_win_by_all_simples_open_rule_close_hand() {
        let test_str = "222456m777p56s88s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Rules::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九".to_string(), true, 1)
        );
    }
    #[test]
    /// 么九牌ありでは断么九にならない（一）
    fn test_not_win_by_all_simples_with_1() {
        let test_str = "111456m777p56s88s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Rules::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九".to_string(), false, 0)
        );
    }
    #[test]
    /// 么九牌ありでは断么九にならない（九）
    fn test_not_win_by_all_simples_with_9() {
        let test_str = "222456m777p5699s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Rules::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九".to_string(), false, 0)
        );
    }
    #[test]
    /// 么九牌ありでは断么九にならない（字牌）
    fn test_not_win_by_all_simples_with_honor() {
        let test_str = "222456m56s88s111z 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Rules::new();
        // 喰い断あり鳴きなし
        rules.openned_all_simples = true;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九".to_string(), false, 0)
        );
    }
    #[test]
    /// 断么九で和了った（喰い断あり鳴きあり）
    fn test_win_by_all_simples_open_rule_open_hand() {
        let test_str = "234m567m234p345s3s 3s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Rules::new();
        // 喰い断あり鳴きあり
        rules.openned_all_simples = true;
        status.has_claimed_open = true;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九".to_string(), true, 1)
        );
    }
    #[test]
    /// 断么九で和了った（喰い断なし鳴きなし）
    fn test_win_by_all_simples_close_rule_close_hand() {
        let test_str = "678m23455p33345ss 5p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Rules::new();
        // 喰い断なし鳴きなし
        rules.openned_all_simples = false;
        status.has_claimed_open = false;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九".to_string(), true, 1)
        );
    }
    #[test]
    /// 断么九で和了った（喰い断なし鳴きあり）->役無し
    fn test_win_by_all_simples_close_rule_open_hand() {
        let test_str = "222m456m777p56s88s 7s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        let mut rules = Rules::new();
        // 喰い断なし鳴きあり（役無し）
        rules.openned_all_simples = false;
        status.has_claimed_open = true;
        assert_eq!(
            check_all_simples(&test_analyzer, &status, &rules),
            ("断么九".to_string(), false, 0)
        );
    }
    #[test]
    /// 一盃口で和了った
    fn test_win_by_one_set_of_identical_sequences() {
        let test_str = "112233m456p456s7z 7z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();

        status.has_claimed_open = false;
        assert_eq!(
            check_one_set_of_identical_sequences(&test_analyzer, &status),
            ("一盃口".to_string(), true, 1)
        );
    }
    #[test]
    /// 一盃口で和了った（鳴きあり）→役なし
    fn test_no_win_by_one_set_of_identical_sequences_with_openned() {
        let test_str = "112233m456p456s7z 7z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();

        status.has_claimed_open = true;
        assert_eq!(
            check_one_set_of_identical_sequences(&test_analyzer, &status),
            ("一盃口".to_string(), false, 0)
        );
    }
    #[test]
    /// 自風で和了った
    fn test_win_by_honor_tiles_players_wind() {
        let test_str = "222m456m777p5s 222z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_players_wind(&test_analyzer, &status),
            ("役牌（自風牌）".to_string(), true, 1)
        );
    }
    #[test]
    /// 場風で和了った
    fn test_win_by_honor_tiles_prevailing_wind() {
        let test_str = "222m456m777p5s 111z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_prevailing_wind(&test_analyzer, &status),
            ("役牌（場風牌）".to_string(), true, 1)
        );
    }
    #[test]
    /// 三元牌（白）で和了った
    fn test_win_by_honor_tiles_white_dragon() {
        let test_str = "222m456m777p5s 555z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_white_dragon(&test_analyzer),
            ("役牌（白）".to_string(), true, 1)
        );
    }
    #[test]
    /// 三元牌（發）で和了った
    fn test_win_by_honor_tiles_green_dragon() {
        let test_str = "222m456m777p5s 666z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_green_dragon(&test_analyzer),
            ("役牌（發）".to_string(), true, 1)
        );
    }
    #[test]
    /// 三元牌（中）で和了った
    fn test_win_by_honor_tiles_red_dragon() {
        let test_str = "222m456m777p5s 777z 5s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();
        // 東場
        status.prevailing_wind = Wind::East;
        // プレイヤーは南家=`2z`
        status.player_wind = Wind::South;
        assert_eq!(
            check_honor_tiles_red_dragon(&test_analyzer),
            ("役牌（中）".to_string(), true, 1)
        );
    }

    #[test]
    /// 混全帯么九で和了った
    fn test_terminal_or_honor_in_each_set(){
        let test_str = "123999m111p79s44z 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();

        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status),
            ("混全帯么九".to_string(), true, 2)
        );
    }
    #[test]
    /// 混全帯么九で和了った（食い下がり1翻）
    fn test_terminal_or_honor_in_each_set_open(){
        let test_str = "123m111p79s44z 789m 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test);
        let mut status = Status::new();

        status.has_claimed_open = true;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status),
            ("混全帯么九（鳴）".to_string(), true, 1)
        );
    }
}
