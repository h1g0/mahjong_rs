use anyhow::Result;

use crate::hand_info::block::BlockProperty;
use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::tile::Tile;
use crate::winning_hand::name::*;

/// 清一色
pub fn check_flush(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::Flush,
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
        if same.has_honor()? {
            return Ok((name, false, 0));
        }
        if same.is_character()? { char = true; }
        if same.is_circle()? { circle = true; }
        if same.is_bamboo()? { bamboo = true; }
    }

    for seq in &hand.sequential3 {
        if seq.has_honor()? { return Ok((name, false, 0)); }
        if seq.is_character()? { char = true; }
        if seq.is_circle()? { circle = true; }
        if seq.is_bamboo()? { bamboo = true; }
    }

    for head in &hand.same2 {
        if head.has_honor()? { return Ok((name, false, 0)); }
        if head.is_character()? { char = true; }
        if head.is_circle()? { circle = true; }
        if head.is_bamboo()? { bamboo = true; }
    }

    let suits = char as u32 + circle as u32 + bamboo as u32;
    if suits == 1 {
        if status.has_claimed_open {
            Ok((name, true, 5))
        } else {
            Ok((name, true, 6))
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

    #[test]
    /// 清一色（門前）で和了った
    fn test_flush_closed() {
        let test_str = "1112223334445m 5m";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_flush(&analyzer, &status, &settings).unwrap(),
            ("清一色", true, 6)
        );
    }

    #[test]
    /// 清一色（鳴き）で和了った
    fn test_flush_open() {
        let test_str = "1234568889m 111m 9m";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let mut status = Status::new();
        let settings = Settings::new();
        status.has_claimed_open = true;
        assert_eq!(
            check_flush(&analyzer, &status, &settings).unwrap(),
            ("清一色（鳴）", true, 5)
        );
    }
}
