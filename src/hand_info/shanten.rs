
use std::cmp::*;

use crate::hand::Hand;
use crate::hand_info::winning_hand::WinningHandForm;
use crate::tile::Tile;

/// 向聴数を計算する
///
/// 向聴数：あと牌を何枚交換すれば聴牌できるかの最小数。聴牌状態が`0`、和了が`-1`。
#[derive(Debug, Eq)]
pub struct Shanten {
    pub num: i32,
    pub form: WinningHandForm,
}
impl Ord for Shanten {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num)
    }
}

impl PartialOrd for Shanten {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Shanten {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl Shanten {
    /// 向聴数を計算する
    ///
    /// 七対子・国士無双・通常の3つの和了形に対してそれぞれ向聴数を求め、最小のものを返す。
    pub fn calc(hand: &Hand) -> Shanten {
        let sp = Shanten::calc_by_form(hand, WinningHandForm::SevenPairs);
        let to = Shanten::calc_by_form(hand, WinningHandForm::ThirteenOrphens);
        let normal = Shanten::calc_by_form(hand, WinningHandForm::Normal);
        return min(min(sp, to), normal);
    }

    /// 和了形を指定して向聴数を計算する
    /// # Examples
    /// 
    /// ```
    /// use mahjong_rs::hand::*;
    /// use mahjong_rs::hand_info::shanten::*;
    /// use mahjong_rs::hand_info::winning_hand::*;
    /// 
    /// // 国士無双で和了る
    /// let to_test_str = "19m19p19s1234567z 1m";
    /// let to_test = Hand::from(to_test_str);
    /// assert_eq!(
    ///   Shanten::calc_by_form(&to_test, WinningHandForm::ThirteenOrphens).num,
    ///   -1
    /// );
    /// 
    /// // 七対子で和了る
    /// let sp_test_str = "1122m3344p5566s7z 7z";
    /// let sp_test = Hand::from(sp_test_str);
    /// assert_eq!(
    ///   Shanten::calc_by_form(&sp_test, WinningHandForm::SevenPairs).num,
    ///   -1
    /// );
    /// ```
    pub fn calc_by_form(hand: &Hand, form: WinningHandForm) -> Shanten {
        return match form {
            WinningHandForm::SevenPairs => Shanten {
                num: Shanten::calc_seven_pairs(hand),
                form: WinningHandForm::SevenPairs,
            },
            WinningHandForm::ThirteenOrphens => Shanten {
                num: Shanten::calc_thirteen_orphens(hand),
                form: WinningHandForm::ThirteenOrphens,
            },
            WinningHandForm::Normal => Shanten {
                num: Shanten::calc_normal_form(hand),
                form: WinningHandForm::Normal,
            },
        };
    }

    /// 七対子への向聴数を計算する
    fn calc_seven_pairs(hand: &Hand) -> i32 {
        let mut pair: u32 = 0;
        let mut kind: u32 = 0;
        let t = hand.summarize_tiles();

        for i in 0..Tile::LEN {
            if t[i] > 0 {
                kind += 1;
                if t[i] >= 2 {
                    pair += 1;
                }
            }
        }
        let num_to_win: i32 = (7 - pair + if kind < 7 { 7 - kind } else { 0 }) as i32;
        return num_to_win - 1;
    }

    /// 国士無双への向聴数を計算する
    fn calc_thirteen_orphens(hand: &Hand) -> i32 {
        let to_tiles = [
            Tile::M1,
            Tile::M9,
            Tile::P1,
            Tile::P9,
            Tile::S1,
            Tile::S9,
            Tile::Z1,
            Tile::Z2,
            Tile::Z3,
            Tile::Z4,
            Tile::Z5,
            Tile::Z6,
            Tile::Z7,
        ];
        let mut pair: u32 = 0;
        let mut kind: u32 = 0;
        let t = hand.summarize_tiles();

        for i in &to_tiles {
            if t[*i as usize] > 0 {
                kind = kind + 1;
                if t[*i as usize] >= 2 {
                    pair += 1;
                }
            }
        }
        let num_to_win: i32 = (14 - kind - if pair > 0 { 1 } else { 0 }) as i32;
        return num_to_win - 1;
    }

    /// 通常の役への向聴数を計算する
    fn calc_normal_form(hand: &Hand) -> i32 {
        unimplemented!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// 七対子で和了った
    fn win_by_seven_pairs() {
        let test_str = "1122m3344p5566s1z 1z";
        let test = Hand::from(test_str);
        assert_eq!(
            Shanten::calc_by_form(&test, WinningHandForm::SevenPairs).num,
            -1
        );
    }

    #[test]
    /// 国士無双で和了った
    fn win_by_thirteen_orphens() {
        let test_str = "19m19p19s1234567z 1m";
        let test = Hand::from(test_str);
        assert_eq!(
            Shanten::calc_by_form(&test, WinningHandForm::ThirteenOrphens).num,
            -1
        );
    }

    #[test]
    /// 七対子を聴牌
    fn zero_shanten_to_seven_pairs() {
        let test_str = "226699m99p228s66z 1z";
        let test = Hand::from(test_str);
        assert_eq!(
            Shanten::calc_by_form(&test, WinningHandForm::SevenPairs).num,
            0
        );
    }
    #[test]
    /// 同じ牌が3枚ある状態で七対子を聴牌
    fn zero_shanten_to_seven_pairs_2() {
        let test_str = "226699m99p222s66z 1z";
        let test = Hand::from(test_str);
        assert_eq!(
            Shanten::calc_by_form(&test, WinningHandForm::SevenPairs).num,
            0
        );
    }
    #[test]
    /// 国士無双を聴牌
    fn zero_shanten_to_orphens() {
        let test_str = "19m19p11s1234567z 5m";
        let test = Hand::from(test_str);
        assert_eq!(
            Shanten::calc_by_form(&test, WinningHandForm::ThirteenOrphens).num,
            0
        );
    }

    #[test]
    /// 同じ牌が4枚ある状態で七対子は認められない（一向聴とみなす）
    fn seven_pairs_with_4_same_tiles() {
        let test_str = "1122m3344p5555s1z 1z";
        let test = Hand::from(test_str);
        assert_eq!(
            Shanten::calc_by_form(&test, WinningHandForm::SevenPairs).num,
            1
        );
    }
}
