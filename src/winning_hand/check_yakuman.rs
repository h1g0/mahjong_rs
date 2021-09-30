use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::winning_hand::name::*;

/// 国士無双
pub fn check_thirteen_orphans(
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
pub fn check_four_concealed_triplets(
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
pub fn check_big_three_dragons(
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
pub fn check_little_four_winds(
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
pub fn check_big_four_winds(
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
pub fn check_all_honors(
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
pub fn check_all_terminals(
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
pub fn check_all_green(
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
pub fn check_nine_gates(
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
pub fn check_four_kans(
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
pub fn check_heavenly_hand(
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
pub fn check_hand_of_earth(
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
    use crate::hand::Hand;

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
}
