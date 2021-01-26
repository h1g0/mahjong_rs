/// 向聴数を計算する
///
/// 向聴数：あと牌を何枚交換すれば聴牌できるかの最小数。聴牌状態が`0`、和了が`-1`。
/// アルゴリズムは https://tomohxx.github.io/mahjong-algorithm-book/ssrf/ を参照した。
use std::cmp::*;

use super::super::hand::Hand;
use super::super::tile::Tile;
use super::super::winning_hand::WinningHandForm;
//use shanten_index::*;

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
        let to_tiles = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];
        let mut pair: u32 = 0;
        let mut kind: u32 = 0;
        let t = hand.summarize_tiles();

        for i in &to_tiles {
            if t[*i] > 0 {
                kind = kind + 1;
                if t[*i] >= 2 {
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

    fn add1(lhs: &mut Vec<u32>, rhs: &mut Vec<u32>) {
        let mentsu_num = 3;
        for i in (5..=mentsu_num + 5).rev() {
            let mut s = min(lhs[i] + rhs[0], lhs[0] + rhs[i]);
            for j in 5..i {
                s = min(s, min(lhs[j] + rhs[i - j], lhs[i - j] + rhs[j]));
            }
            lhs[i] = s;
        }
        for i in (0..=mentsu_num).rev() {
            let mut s = lhs[i] + rhs[0];

            for j in 0..i {
                s = min(s, lhs[j] + rhs[i - j]);
            }
            lhs[i] = s;
        }
    }

    fn add2(lhs: &mut Vec<u32>, rhs: &mut Vec<u32>) {
        let mentsu_num = 3;
        let i = mentsu_num + 5;
        let mut s = min(lhs[i] + rhs[0], lhs[0] + rhs[i]);

        for j in 5..i {
            s = min(s, min(lhs[j] + rhs[i - j], lhs[i - j] + rhs[j]));
        }
        lhs[i] = s;
    }
}
