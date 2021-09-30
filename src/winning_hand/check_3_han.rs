use crate::hand_info::block::BlockProperty;
use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::winning_hand::name::*;

/// 二盃口
pub fn check_two_sets_of_identical_sequences(
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
/// 純全帯么九
pub fn check_terminal_in_each_set(
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
/// 混一色
pub fn check_half_flush(
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
}
