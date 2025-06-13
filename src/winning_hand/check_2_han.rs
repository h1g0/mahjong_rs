use anyhow::Result;

use crate::hand_info::block::BlockProperty;
use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::tile::{Tile, Dragon};
use crate::winning_hand::name::*;

/// 七対子
pub fn check_seven_pairs(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::SevenPairs,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if hand.form == Form::SevenPairs {
        Ok((name, true, 2))
    } else {
        Ok((name, false, 0))
    }
}

/// 三色同順
pub fn check_three_colour_straight(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::ThreeColourStraight,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    let mut m = [false; 7];
    let mut p = [false; 7];
    let mut s = [false; 7];

    for v in &hand.sequential3 {
        match v.get() {
            [Tile::M1, Tile::M2, Tile::M3] => m[0] = true,
            [Tile::M2, Tile::M3, Tile::M4] => m[1] = true,
            [Tile::M3, Tile::M4, Tile::M5] => m[2] = true,
            [Tile::M4, Tile::M5, Tile::M6] => m[3] = true,
            [Tile::M5, Tile::M6, Tile::M7] => m[4] = true,
            [Tile::M6, Tile::M7, Tile::M8] => m[5] = true,
            [Tile::M7, Tile::M8, Tile::M9] => m[6] = true,
            [Tile::P1, Tile::P2, Tile::P3] => p[0] = true,
            [Tile::P2, Tile::P3, Tile::P4] => p[1] = true,
            [Tile::P3, Tile::P4, Tile::P5] => p[2] = true,
            [Tile::P4, Tile::P5, Tile::P6] => p[3] = true,
            [Tile::P5, Tile::P6, Tile::P7] => p[4] = true,
            [Tile::P6, Tile::P7, Tile::P8] => p[5] = true,
            [Tile::P7, Tile::P8, Tile::P9] => p[6] = true,
            [Tile::S1, Tile::S2, Tile::S3] => s[0] = true,
            [Tile::S2, Tile::S3, Tile::S4] => s[1] = true,
            [Tile::S3, Tile::S4, Tile::S5] => s[2] = true,
            [Tile::S4, Tile::S5, Tile::S6] => s[3] = true,
            [Tile::S5, Tile::S6, Tile::S7] => s[4] = true,
            [Tile::S6, Tile::S7, Tile::S8] => s[5] = true,
            [Tile::S7, Tile::S8, Tile::S9] => s[6] = true,
            _ => {}
        }
    }

    for i in 0..7 {
        if m[i] && p[i] && s[i] {
            if status.has_claimed_open {
                return Ok((name, true, 1));
            } else {
                return Ok((name, true, 2));
            }
        }
    }

    Ok((name, false, 0))
}
/// 一気通貫
pub fn check_straight(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::Straight,
        status.has_claimed_open,
        settings.display_lang,
    );

    if !has_won(hand) {
        return Ok((name, false, 0));
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
            return Ok((name, true, 1));
        } else {
            return Ok((name, true, 2));
        }
    }
    Ok((name, false, 0))
}
/// 対々和
pub fn check_all_triplet_hand(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::AllTripletHand,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if hand.same3.len() == 4 && hand.same2.len() == 1 {
        return Ok((name, true, 2));
    }
    Ok((name, false, 0))
}
/// 三暗刻
pub fn check_three_closed_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::ThreeClosedTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if !status.has_claimed_open && hand.same3.len() >= 3 {
        return Ok((name, true, 2));
    }
    Ok((name, false, 0))
}
/// 三色同刻
pub fn check_three_colour_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::ThreeColourTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    let mut m = [false; 9];
    let mut p = [false; 9];
    let mut s = [false; 9];

    for v in &hand.same3 {
        let t = v.get()[0];
        match t {
            Tile::M1..=Tile::M9 => m[(t - Tile::M1) as usize] = true,
            Tile::P1..=Tile::P9 => p[(t - Tile::P1) as usize] = true,
            Tile::S1..=Tile::S9 => s[(t - Tile::S1) as usize] = true,
            _ => {}
        }
    }

    for i in 0..9 {
        if m[i] && p[i] && s[i] {
            return Ok((name, true, 2));
        }
    }

    Ok((name, false, 0))
}
/// 混全帯么九
pub fn check_terminal_or_honor_in_each_set(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::TerminalOrHonorInEachSet,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }

    // 混老頭とは複合しないため、必ず順子が含まれる
    if hand.sequential3.len() == 0 {
        return Ok((name, false, 0));
    }

    let mut no_1_9_honor = false;
    // 純全帯么九とは複合しないため、必ず三元牌が含まれる
    let mut has_honor = false;

    // 面子

    // 刻子
    for same in &hand.same3 {
        if !same.has_1_or_9()? && !same.has_honor()? {
            no_1_9_honor = true;
        }

        if same.has_honor()? {
            has_honor = true;
        }
    }
    // 順子
    for seq in &hand.sequential3 {
        if !seq.has_1_or_9()? {
            no_1_9_honor = true;
        }
    }

    // 雀頭
    for head in &hand.same2 {
        if !head.has_1_or_9()? && !head.has_honor()? {
            no_1_9_honor = true;
        }
        if head.has_honor()? {
            has_honor = true;
        }
    }

    if no_1_9_honor || !has_honor {
        return Ok((name, false, 0));
    }
    if status.has_claimed_open {
        return Ok((name, true, 1));
    }
    Ok((name, true, 2))
}
/// 混老頭
pub fn check_all_terminals_and_honors(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::AllTerminalsAndHonors,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if !hand.sequential3.is_empty() {
        return Ok((name, false, 0));
    }

    if hand.same3.len() != 4 || hand.same2.len() != 1 {
        return Ok((name, false, 0));
    }

    for same in &hand.same3 {
        if !(same.has_1_or_9()? || same.has_honor()?) {
            return Ok((name, false, 0));
        }
    }

    for pair in &hand.same2 {
        if !(pair.has_1_or_9()? || pair.has_honor()?) {
            return Ok((name, false, 0));
        }
    }

    Ok((name, true, 2))
}
/// 小三元
pub fn check_little_three_dragons(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::LittleThreeDragons,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    let mut triplet_cnt = 0;
    let mut pair_found = false;

    for same in &hand.same3 {
        if same.has_dragon(Dragon::White)?
            || same.has_dragon(Dragon::Green)?
            || same.has_dragon(Dragon::Red)?
        {
            triplet_cnt += 1;
        }
    }

    for head in &hand.same2 {
        if head.has_dragon(Dragon::White)?
            || head.has_dragon(Dragon::Green)?
            || head.has_dragon(Dragon::Red)?
        {
            pair_found = true;
        }
    }

    if triplet_cnt == 2 && pair_found {
        return Ok((name, true, 2));
    }

    Ok((name, false, 0))
}

/// ユニットテスト
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::*;
    #[test]
    /// 七対子で和了った
    fn test_win_by_seven_pairs() {
        let test_str = "1122m3344p5566s1z 1z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_seven_pairs(&test_analyzer, &status, &settings).unwrap(),
            ("七対子", true, 2)
        );
    }
    #[test]
    /// 混全帯么九で和了った
    fn test_terminal_or_honor_in_each_set() {
        let test_str = "123999m111p79s44z 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings).unwrap(),
            ("混全帯么九", true, 2)
        );
    }
    #[test]
    /// 混全帯么九で和了った（食い下がり1翻）
    fn test_terminal_or_honor_in_each_set_open() {
        let test_str = "123m111p79s44z 789m 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings).unwrap(),
            ("混全帯么九（鳴）", true, 1)
        );
    }
    #[test]
    /// 対々和で和了った
    fn test_all_triplet_hand() {
        let test_str = "777m333p22z 555m 999s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_all_triplet_hand(&test_analyzer, &status, &settings).unwrap(),
            ("対々和", true, 2)
        );
    }

    #[test]
    /// 一気通貫で和了った
    fn test_straight() {
        let test_str = "123456789m78p22z 9p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_straight(&test_analyzer, &status, &settings).unwrap(),
            ("一気通貫", true, 2)
        );
    }

    #[test]
    /// 一気通貫で和了った（食い下がり1翻）
    fn test_straight_open() {
        let test_str = "123m1p123s 456s 789s 1p";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_straight(&test_analyzer, &status, &settings).unwrap(),
            ("一気通貫（鳴）", true, 1)
        );
    }

    #[test]
    /// 三色同順で和了った（門前）
    fn test_three_colour_straight_closed() {
        let test_str = "123m123p123s789m9p 9p";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_three_colour_straight(&analyzer, &status, &settings).unwrap(),
            ("三色同順", true, 2)
        );
    }

    #[test]
    /// 三色同順で和了った（鳴き）
    fn test_three_colour_straight_open() {
        let test_str = "123m789m9p 123p 123s 9p";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_three_colour_straight(&analyzer, &status, &settings).unwrap(),
            ("三色同順（鳴）", true, 1)
        );
    }

    #[test]
    /// 三暗刻で和了った
    fn test_three_closed_triplets() {
        let test_str = "111333m444s1777z 1z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_three_closed_triplets(&analyzer, &status, &settings).unwrap(),
            ("三暗刻", true, 2)
        );
    }

    #[test]
    /// 三色同刻で和了った
    fn test_three_colour_triplets() {
        let test_str = "111m111p111s444m5z 5z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_three_colour_triplets(&analyzer, &status, &settings).unwrap(),
            ("三色同刻", true, 2)
        );
    }

    #[test]
    /// 混老頭で和了った
    fn test_all_terminals_and_honors() {
        let test_str = "111m999p111s777z55z 5z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_all_terminals_and_honors(&analyzer, &status, &settings).unwrap(),
            ("混老頭", false, 0)
        );
    }

    #[test]
    /// 小三元で和了った
    fn test_little_three_dragons() {
        let test_str = "111m111p555z666z7z 7z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_little_three_dragons(&analyzer, &status, &settings).unwrap(),
            ("小三元", true, 2)
        );
    }
}
