use crate::hand_info::block::BlockProperty;
use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::tile::Tile;
use crate::winning_hand::name::*;

/// 七対子
pub fn check_seven_pairs(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::SevenPairs,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    return if hand.form == Form::SevenPairs {
        (name, true, 2)
    } else {
        (name, false, 0)
    };
}

/// 三色同順
pub fn check_three_colour_straight(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::ThreeColourStraight,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 一気通貫
pub fn check_straight(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::Straight,
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
/// 対々和
pub fn check_all_triplet_hand(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::AllTripletHand,
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
pub fn check_three_closed_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::ThreeClosedTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 三色同刻
pub fn check_three_colour_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::ThreeColourTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 混全帯么九
pub fn check_terminal_or_honor_in_each_set(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::TerminalOrHonorInEachSet,
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
/// 混老頭
pub fn check_all_terminals_and_honors(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::AllTerminalsAndHonors,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}
/// 小三元
pub fn check_little_three_dragons(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::LittleThreeDragons,
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
