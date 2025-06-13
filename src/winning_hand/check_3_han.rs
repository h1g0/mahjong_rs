use anyhow::Result;

use crate::hand_info::block::BlockProperty;
use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::tile::{TileType, Tile};
use crate::winning_hand::name::*;

/// 二盃口
pub fn check_two_sets_of_identical_sequences(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::TwoSetsOfIdenticalSequences,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if status.has_claimed_open {
        return Ok((name, false, 0));
    }
    if hand.sequential3.len() < 4 {
        return Ok((name, false, 0));
    }
    use std::collections::HashMap;
    let mut map: HashMap<[TileType;3], usize> = HashMap::new();
    for seq in &hand.sequential3 {
        *map.entry(seq.get()).or_insert(0) += 1;
    }
    let pairs = map.values().map(|c| c / 2).sum::<usize>();
    if pairs >= 2 {
        Ok((name, true, 3))
    } else {
        Ok((name, false, 0))
    }
}
/// 純全帯么九
pub fn check_terminal_in_each_set(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::TerminalInEachSet,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    // 清老頭とは複合しないため、必ず順子が含まれる
    if hand.sequential3.len() == 0 {
        return Ok((name, false, 0));
    }

    let mut no_1_9 = false;
    // 面子

    // 刻子
    for same in &hand.same3 {
        if !same.has_1_or_9()? {
            no_1_9 = true;
        }
    }
    // 順子
    for seq in &hand.sequential3 {
        if !seq.has_1_or_9()? {
            no_1_9 = true;
        }
    }

    // 雀頭
    for head in &hand.same2 {
        if !head.has_1_or_9()? {
            no_1_9 = true;
        }
    }

    if no_1_9 {
        return Ok((name, false, 0));
    }
    if status.has_claimed_open {
        Ok((name, true, 2))
    }else{
        Ok((name, true, 3))
    }
}
/// 混一色
pub fn check_half_flush(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::HalfFlush,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    let mut char = false;
    let mut circle = false;
    let mut bamboo = false;

    for same in &hand.same3 {
        if same.is_character()? { char = true; }
        if same.is_circle()? { circle = true; }
        if same.is_bamboo()? { bamboo = true; }
    }
    for seq in &hand.sequential3 {
        if seq.is_character()? { char = true; }
        if seq.is_circle()? { circle = true; }
        if seq.is_bamboo()? { bamboo = true; }
    }
    for pair in &hand.same2 {
        if pair.is_character()? { char = true; }
        if pair.is_circle()? { circle = true; }
        if pair.is_bamboo()? { bamboo = true; }
    }

    let suits = char as u32 + circle as u32 + bamboo as u32;
    if suits == 1 {
        if status.has_claimed_open {
            Ok((name, true, 2))
        } else {
            Ok((name, true, 3))
        }
    } else {
        Ok((name, false, 0))
    }
}

/// ユニットテスト
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::*;
    use crate::winning_hand::check_2_han::*;

    #[test]
    /// 純全帯么九で和了った
    fn test_terminal_in_each_set() {
        let test_str = "123999m11p11179s 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings).unwrap(),
            ("純全帯么九", true, 3)
        );
    }
    #[test]
    /// 純全帯么九で和了った（食い下がり2翻）
    fn test_terminal_in_each_set_open() {
        let test_str = "123m111p7999s 789m 8s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings).unwrap(),
            ("純全帯么九（鳴）", true, 2)
        );
    }

    #[test]
    /// 混全帯么九は純全帯么九と複合しない
    fn test_terminal_or_honor_in_each_set_does_not_combined_with_terminal_in_each_set() {
        let test_str = "111789m111p99s11z 1z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings).unwrap().1,
            true
        );
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings).unwrap().1,
            false
        );
    }
    #[test]
    /// 純全帯么九は混全帯么九と複合しない
    fn test_terminal_in_each_set_does_not_combined_with_terminal_or_honor_in_each_set() {
        let test_str = "111789m111p1199s 9s";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = false;
        assert_eq!(
            check_terminal_or_honor_in_each_set(&test_analyzer, &status, &settings).unwrap().1,
            false
        );
        assert_eq!(
            check_terminal_in_each_set(&test_analyzer, &status, &settings).unwrap().1,
            true
        );
    }
    #[test]
    /// 二盃口の判定
    fn test_two_sets_of_identical_sequences() {
        let test_str = "112233m778899p7z 7z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_two_sets_of_identical_sequences(&analyzer, &status, &settings).unwrap(),
            ("二盃口", false, 0)
        );
    }

    #[test]
    /// 混一色（門前）で和了った
    fn test_half_flush_closed() {
        let test_str = "123456789m1112z 2z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_half_flush(&analyzer, &status, &settings).unwrap(),
            ("混一色", true, 3)
        );
    }

    #[test]
    /// 混一色（鳴き）で和了った
    fn test_half_flush_open() {
        let test_str = "111m789m2z 123m 222z 2z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_half_flush(&analyzer, &status, &settings).unwrap(),
            ("混一色（鳴）", true, 2)
        );
    }
}
