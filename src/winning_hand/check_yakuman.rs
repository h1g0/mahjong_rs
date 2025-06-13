use anyhow::Result;

use crate::hand_info::block::BlockProperty;
use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::tile::{Dragon, Wind, Tile, TileType};
use crate::winning_hand::name::*;

/// 国士無双
pub fn check_thirteen_orphans(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::ThirteenOrphans,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    return if hand.form == Form::ThirteenOrphens {
        Ok((name, true, 13))
    } else {
        Ok((name, false, 0))
    };
}
/// 四暗刻
pub fn check_four_concealed_triplets(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::FourConcealedTriplets,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if !status.has_claimed_open && hand.same3.len() == 4 && status.is_self_picked {
        Ok((name, true, 13))
    } else {
        Ok((name, false, 0))
    }
}
/// 大三元
pub fn check_big_three_dragons(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::BigThreeDragons,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    let mut dragons = [false; 3];
    for same in &hand.same3 {
        if same.has_dragon(Dragon::White)? {
            dragons[0] = true;
        } else if same.has_dragon(Dragon::Green)? {
            dragons[1] = true;
        } else if same.has_dragon(Dragon::Red)? {
            dragons[2] = true;
        }
    }
    if dragons.iter().all(|&b| b) {
        Ok((name, true, 13))
    } else {
        Ok((name, false, 0))
    }
}
/// 小四喜
pub fn check_little_four_winds(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::LittleFourWinds,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    let mut winds_triplet = [false; 4];
    let mut pair_wind = [false; 4];

    for same in &hand.same3 {
        if let Some(w) = Wind::is_tile_type(same.get()[0]) {
            winds_triplet[w as usize - Tile::Z1 as usize] = true;
        }
    }
    for head in &hand.same2 {
        if let Some(w) = Wind::is_tile_type(head.get()[0]) {
            pair_wind[w as usize - Tile::Z1 as usize] = true;
        }
    }

    let triplet_count = winds_triplet.iter().filter(|&&b| b).count();
    if triplet_count == 3 {
        for i in 0..4 {
            if !winds_triplet[i] && pair_wind[i] {
                return Ok((name, true, 13));
            }
        }
    }
    Ok((name, false, 0))
}
/// 大四喜
pub fn check_big_four_winds(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::BigFourWinds,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    let mut winds_triplet = [false; 4];
    for same in &hand.same3 {
        if let Some(w) = Wind::is_tile_type(same.get()[0]) {
            winds_triplet[w as usize - Tile::Z1 as usize] = true;
        }
    }
    if winds_triplet.iter().all(|&b| b) {
        Ok((name, true, 13))
    } else {
        Ok((name, false, 0))
    }
}
/// 字一色
pub fn check_all_honors(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::AllHonors,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if hand.sequential3.len() > 0 {
        return Ok((name, false, 0));
    }
    for same in &hand.same3 {
        if !same.has_honor()? {
            return Ok((name, false, 0));
        }
    }
    for pair in &hand.same2 {
        if !pair.has_honor()? {
            return Ok((name, false, 0));
        }
    }
    Ok((name, true, 13))
}
/// 清老頭
pub fn check_all_terminals(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::AllTerminals,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    if hand.sequential3.len() > 0 {
        return Ok((name, false, 0));
    }
    for same in &hand.same3 {
        if !(same.has_1_or_9()? && !same.has_honor()?) {
            return Ok((name, false, 0));
        }
    }
    for pair in &hand.same2 {
        if !(pair.has_1_or_9()? && !pair.has_honor()?) {
            return Ok((name, false, 0));
        }
    }
    Ok((name, true, 13))
}
/// 緑一色
pub fn check_all_green(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::AllGreen,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    const GREEN: [TileType; 6] = [Tile::S2, Tile::S3, Tile::S4, Tile::S6, Tile::S8, Tile::Z6];

    let mut is_green = |t: TileType| GREEN.contains(&t);

    for same in &hand.same3 {
        let tile = same.get()[0];
        if !is_green(tile) {
            return Ok((name, false, 0));
        }
    }
    for seq in &hand.sequential3 {
        for t in seq.get() {
            if !is_green(t) {
                return Ok((name, false, 0));
            }
        }
    }
    for pair in &hand.same2 {
        if !is_green(pair.get()[0]) {
            return Ok((name, false, 0));
        }
    }
    Ok((name, true, 13))
}
/// 九蓮宝燈
pub fn check_nine_gates(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::NineGates,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    // まず清一色かどうかを確認する
    let mut suit: Option<u32> = None;
    let mut counts = [0u32; Tile::LEN];

    for same in &hand.same3 {
        let t = same.get()[0];
        counts[t as usize] += 3;
        match t {
            Tile::M1..=Tile::M9 => { suit.get_or_insert(0); }
            Tile::P1..=Tile::P9 => { suit.get_or_insert(1); }
            Tile::S1..=Tile::S9 => { suit.get_or_insert(2); }
            _ => return Ok((name, false, 0)),
        }
        if let Some(s) = suit {
            match s {
                0 if !(Tile::M1..=Tile::M9).contains(&t) => return Ok((name, false, 0)),
                1 if !(Tile::P1..=Tile::P9).contains(&t) => return Ok((name, false, 0)),
                2 if !(Tile::S1..=Tile::S9).contains(&t) => return Ok((name, false, 0)),
                _ => {}
            }
        }
    }
    for seq in &hand.sequential3 {
        for t in seq.get() {
            counts[t as usize] += 1;
            match t {
                Tile::M1..=Tile::M9 => { suit.get_or_insert(0); }
                Tile::P1..=Tile::P9 => { suit.get_or_insert(1); }
                Tile::S1..=Tile::S9 => { suit.get_or_insert(2); }
                _ => return Ok((name, false, 0)),
            }
            if let Some(s) = suit {
                match s {
                    0 if !(Tile::M1..=Tile::M9).contains(&t) => return Ok((name, false, 0)),
                    1 if !(Tile::P1..=Tile::P9).contains(&t) => return Ok((name, false, 0)),
                    2 if !(Tile::S1..=Tile::S9).contains(&t) => return Ok((name, false, 0)),
                    _ => {}
                }
            }
        }
    }
    for pair in &hand.same2 {
        let t = pair.get()[0];
        counts[t as usize] += 2;
        match t {
            Tile::M1..=Tile::M9 => { suit.get_or_insert(0); }
            Tile::P1..=Tile::P9 => { suit.get_or_insert(1); }
            Tile::S1..=Tile::S9 => { suit.get_or_insert(2); }
            _ => return Ok((name, false, 0)),
        }
        if let Some(s) = suit {
            match s {
                0 if !(Tile::M1..=Tile::M9).contains(&t) => return Ok((name, false, 0)),
                1 if !(Tile::P1..=Tile::P9).contains(&t) => return Ok((name, false, 0)),
                2 if !(Tile::S1..=Tile::S9).contains(&t) => return Ok((name, false, 0)),
                _ => {}
            }
        }
    }

    let base = match suit {
        Some(0) => Tile::M1,
        Some(1) => Tile::P1,
        Some(2) => Tile::S1,
        _ => return Ok((name, false, 0)),
    } as usize;

    if counts[base] < 3 || counts[base + 8] < 3 {
        return Ok((name, false, 0));
    }
    for i in 1..8 {
        if counts[base + i] == 0 {
            return Ok((name, false, 0));
        }
    }
    Ok((name, true, 13))
}
/// 四槓子
pub fn check_four_kans(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::FourKans,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    todo!()
}
/// 天和
pub fn check_heavenly_hand(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::HeavenlyHand,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    todo!()
}
/// 地和
pub fn check_hand_of_earth(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> Result<(&'static str, bool, u32)> {
    let name = get(
        Kind::HandOfEarth,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return Ok((name, false, 0));
    }
    todo!()
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
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_thirteen_orphans(&test_analyzer, &status, &settings).unwrap(),
            ("国士無双", true, 13)
        );
    }

    #[test]
    /// 四暗刻単騎で和了った
    fn test_win_by_four_concealed_triplets_single() {
        let test_str = "111333m444s1777z 1z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        status.is_self_picked = true; // 自摸和了
        let settings = Settings::new();
        assert_eq!(
            check_four_concealed_triplets(&test_analyzer, &status, &settings).unwrap(),
            ("四暗刻", true, 13)
        );
    }

    #[test]
    /// 通常の四暗刻では、自摸和了のみ（ロンした場合は三暗刻＋対々和になる）
    fn test_not_win_by_four_concealed_triplets_single_if_not_self_pick() {
        let test_str = "111333m444s1777z 1z";
        let test = Hand::from(test_str);
        let test_analyzer = HandAnalyzer::new(&test).unwrap();
        let mut status = Status::new();
        status.is_self_picked = false;
        let settings = Settings::new();
        assert_eq!(
            check_four_concealed_triplets(&test_analyzer, &status, &settings).unwrap(),
            ("四暗刻", false, 0)
        );
    }
    #[test]
    /// 大三元で和了った
    fn test_big_three_dragons() {
        let test_str = "1234m555666z 777z 1m";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_big_three_dragons(&analyzer, &status, &settings).unwrap(),
            ("大三元", true, 13)
        );
    }

    #[test]
    /// 小四喜で和了った
    fn test_little_four_winds() {
        let test_str = "111m1112223334z 4z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_little_four_winds(&analyzer, &status, &settings).unwrap(),
            ("小四喜", true, 13)
        );
    }

    #[test]
    /// 大四喜で和了った
    fn test_big_four_winds() {
        let test_str = "1m111222333444z 1m";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_big_four_winds(&analyzer, &status, &settings).unwrap(),
            ("大四喜", true, 13)
        );
    }

    #[test]
    /// 字一色で和了った
    fn test_all_honors() {
        let test_str = "1112223335556z 6z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_all_honors(&analyzer, &status, &settings).unwrap(),
            ("字一色", true, 13)
        );
    }

    #[test]
    /// 清老頭で和了った
    fn test_all_terminals() {
        let test_str = "111999m111999p1s 1s";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_all_terminals(&analyzer, &status, &settings).unwrap(),
            ("清老頭", true, 13)
        );
    }

    #[test]
    /// 緑一色で和了ったかを確認
    fn test_all_green() {
        let test_str = "22334466s66z 888s 6z";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_all_green(&analyzer, &status, &settings).unwrap(),
            ("緑一色", true, 13)
        );
    }

    #[test]
    /// 九蓮宝燈で和了った
    fn test_nine_gates() {
        let test_str = "1112345678999m 5m";
        let hand = Hand::from(test_str);
        let analyzer = HandAnalyzer::new(&hand).unwrap();
        let status = Status::new();
        let settings = Settings::new();
        assert_eq!(
            check_nine_gates(&analyzer, &status, &settings).unwrap(),
            ("九蓮宝燈", true, 13)
        );
    }
}
