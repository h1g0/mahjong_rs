use crate::hand_info::hand_analyzer::HandAnalyzer;
/// 役を判定する
use std::collections::HashMap;

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
/// https://en.wikipedia.org/wiki/Japanese_Mahjong_yaku による英語名
pub const HAND_NAME: [&'static str; 40] = [
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
    // 役牌（三元牌）
    "honor_tiles_dragons",
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

pub fn check(hand: &HandAnalyzer) -> HashMap<&str, (&str,bool)> {
    let mut result = HashMap::new();
    for i in 0..HAND_NAME.len() {
        result.insert(HAND_NAME[i], ("unknown",false));
    }

    // 立直
    result.insert("ready_hand", check_ready_hand(hand));
    // 七対子
    result.insert("seven_pairs", check_seven_pairs(hand));
    // 流し満貫
    result.insert("nagashi_mangan", check_nagashi_mangan(hand));
    // 門前清自摸和
    result.insert("self_pick", check_self_pick(hand));
    // 一発
    result.insert("one_shot", check_one_shot(hand));
    // 海底撈月
    result.insert(
        "last_tile_from_the_wall",
        check_last_tile_from_the_wall(hand),
    );
    // 河底撈魚
    result.insert("last_discard", check_last_discard(hand));
    // 嶺上開花
    result.insert("dead_wall_draw", check_dead_wall_draw(hand));
    // 搶槓
    result.insert("robbing_a_quad", check_robbing_a_quad(hand));
    // ダブル立直
    result.insert("double_ready", check_double_ready(hand));
    // 平和
    result.insert("no_points_hand", check_no_points_hand(hand));
    // 一盃口
    result.insert(
        "one_set_of_identical_sequences",
        check_one_set_of_identical_sequences(hand),
    );
    // 三色同順
    result.insert("three_colour_straight", check_three_colour_straight(hand));
    // 一気通貫
    result.insert("straight", check_straight(hand));
    // 二盃口
    result.insert(
        "two_sets_of_identical_sequences",
        check_two_sets_of_identical_sequences(hand),
    );
    // 対々和
    result.insert("all_triplet_hand", check_all_triplet_hand(hand));
    // 三暗刻
    result.insert("three_closed_triplets", check_three_closed_triplets(hand));
    // 三色同刻
    result.insert("three_colour_triplets", check_three_colour_triplets(hand));
    // 断么九
    result.insert("all_simples", check_all_simples(hand));
    // 役牌（自風牌）
    result.insert(
        "honor_tiles_players_wind",
        check_honor_tiles_players_wind(hand),
    );
    // 役牌（場風牌）
    result.insert(
        "honor_tiles_prevailing_wind",
        check_honor_tiles_prevailing_wind(hand),
    );
    // 役牌（三元牌）
    result.insert("honor_tiles_dragons", check_honor_tiles_dragons(hand));
    // 混全帯么九
    result.insert(
        "terminal_or_honor_in_each_set",
        check_terminal_or_honor_in_each_set(hand),
    );
    // 純全帯么九
    result.insert("terminal_in_each_set", check_terminal_in_each_set(hand));
    // 混老頭
    result.insert(
        "all_terminals_and_honors",
        check_all_terminals_and_honors(hand),
    );
    // 小三元
    result.insert("little_three_dragons", check_little_three_dragons(hand));
    // 混一色
    result.insert("half_flush", check_half_flush(hand));
    // 清一色
    result.insert("flush", check_flush(hand));
    // 国士無双
    result.insert("thirteen_orphans", check_thirteen_orphans(hand));
    // 四暗刻
    result.insert(
        "four_concealed_triplets",
        check_four_concealed_triplets(hand),
    );
    // 大三元
    result.insert("big_three_dragons", check_big_three_dragons(hand));
    // 小四喜
    result.insert("little_four_winds", check_little_four_winds(hand));
    // 大四喜
    result.insert("big_four_winds", check_big_four_winds(hand));
    // 字一色
    result.insert("all_honors", check_all_honors(hand));
    // 清老頭
    result.insert("all_terminals", check_all_terminals(hand));
    // 緑一色
    result.insert("all_green", check_all_green(hand));
    // 九蓮宝燈
    result.insert("nine_gates", check_nine_gates(hand));
    // 四槓子
    result.insert("four_kans", check_four_kans(hand));
    // 天和
    result.insert("heavenly_hand", check_heavenly_hand(hand));
    // 地和
    result.insert("hand_of_earth", check_hand_of_earth(hand));

    return result;
}

/// 立直
fn check_ready_hand(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("立直",false);
    }
    unimplemented!();
}
/// 七対子
fn check_seven_pairs(hand: &HandAnalyzer) -> (&str,bool) {
    let name = "七対子";
    if hand.shanten > -1 {
        return (name,false);
    }
    return if hand.form == WinningHandForm::SevenPairs {
        (name,true)
    } else {
        (name,false)
    };
}
/// 流し満貫
fn check_nagashi_mangan(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("流し満貫",false);
    }
    unimplemented!();
}
/// 門前清自摸和
fn check_self_pick(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("門前清自摸和",false);
    }
    unimplemented!();
}
/// 一発
fn check_one_shot(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("一発",false);
    }
    unimplemented!();
}
/// 海底撈月
fn check_last_tile_from_the_wall(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("海底撈月",false);
    }
    unimplemented!();
}
/// 河底撈魚
fn check_last_discard(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("河底撈魚",false);
    }
    unimplemented!();
}
/// 嶺上開花
fn check_dead_wall_draw(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("嶺上開花",false);
    }
    unimplemented!();
}
/// 搶槓
fn check_robbing_a_quad(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("搶槓",false);
    }
    unimplemented!();
}
/// ダブル立直
fn check_double_ready(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("ダブル立直",false);
    }
    unimplemented!();
}
/// 平和
fn check_no_points_hand(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("平和",false);
    }
    unimplemented!();
}
/// 一盃口
fn check_one_set_of_identical_sequences(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("一盃口",false);
    }
    unimplemented!();
}
/// 三色同順
fn check_three_colour_straight(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("三色同順",false);
    }
    unimplemented!();
}
/// 一気通貫
fn check_straight(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("一気通貫",false);
    }
    unimplemented!();
}
/// 二盃口
fn check_two_sets_of_identical_sequences(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("二盃口",false);
    }
    unimplemented!();
}
/// 対々和
fn check_all_triplet_hand(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("対々和",false);
    }
    unimplemented!();
}
/// 三暗刻
fn check_three_closed_triplets(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("三暗刻",false);
    }
    unimplemented!();
}
/// 三色同刻
fn check_three_colour_triplets(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("三色同刻",false);
    }
    unimplemented!();
}
/// 断么九
fn check_all_simples(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("断么九",false);
    }
    unimplemented!();
}
/// 役牌（自風牌）
fn check_honor_tiles_players_wind(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("役牌（自風牌）",false);
    }
    unimplemented!();
}
/// 役牌（場風牌）
fn check_honor_tiles_prevailing_wind(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("役牌（場風牌）",false);
    }
    unimplemented!();
}
/// 役牌（三元牌）
fn check_honor_tiles_dragons(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("役牌（三元牌）",false);
    }
    unimplemented!();
}
/// 混全帯么九
fn check_terminal_or_honor_in_each_set(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("混全帯么九",false);
    }
    unimplemented!();
}
/// 純全帯么九
fn check_terminal_in_each_set(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("純全帯么九",false);
    }
    unimplemented!();
}
/// 混老頭
fn check_all_terminals_and_honors(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("混老頭",false);
    }
    unimplemented!();
}
/// 小三元
fn check_little_three_dragons(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("小三元",false);
    }
    unimplemented!();
}
/// 混一色
fn check_half_flush(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("混一色",false);
    }
    unimplemented!();
}
/// 清一色
fn check_flush(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("清一色",false);
    }
    unimplemented!();
}
/// 国士無双
fn check_thirteen_orphans(hand: &HandAnalyzer) -> (&str,bool) {
    let name = "国士無双";
    if hand.shanten > -1 {
        return (name,false);
    }
    return if hand.form == WinningHandForm::ThirteenOrphens {
        (name,true)
    } else {
        (name,false)
    };
}
/// 四暗刻
fn check_four_concealed_triplets(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("四暗刻",false);
    }
    unimplemented!();
}
/// 大三元
fn check_big_three_dragons(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("大三元",false);
    }
    unimplemented!();
}
/// 小四喜
fn check_little_four_winds(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("小四喜",false);
    }
    unimplemented!();
}
/// 大四喜
fn check_big_four_winds(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("大四喜",false);
    }
    unimplemented!();
}
/// 字一色
fn check_all_honors(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("字一色",false);
    }
    unimplemented!();
}
/// 清老頭
fn check_all_terminals(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("清老頭",false);
    }
    unimplemented!();
}
/// 緑一色
fn check_all_green(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("緑一色",false);
    }
    unimplemented!();
}
/// 九蓮宝燈
fn check_nine_gates(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("九蓮宝燈",false);
    }
    unimplemented!();
}
/// 四槓子
fn check_four_kans(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("四槓子",false);
    }
    unimplemented!();
}
/// 天和
fn check_heavenly_hand(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("天和",false);
    }
    unimplemented!();
}
/// 地和
fn check_hand_of_earth(hand: &HandAnalyzer) -> (&str,bool) {
    if hand.shanten > -1 {
        return ("地和",false);
    }
    unimplemented!();
}

/// ユニットテスト
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::*;

    #[test]
    /// 七対子で和了った
    fn win_by_seven_pairs() {
        let test_str = "1122m3344p5566s1z 1z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::calc(&test);
        assert_eq!(check_seven_pairs(&test_analyzer), ("七対子",true));
    }

    #[test]
    /// 国士無双で和了った
    fn win_by_thirteen_orphens() {
        let test_str = "19m19p19s1234567z 1m";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::calc(&test);
        assert_eq!(check_thirteen_orphans(&test_analyzer), ("国士無双",true));
    }
}
