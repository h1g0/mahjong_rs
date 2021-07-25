use std::cmp::*;

use crate::hand::Hand;
use crate::hand_info::block::*;
use crate::hand_info::winning_hand::WinningHandForm;
use crate::tile::*;

/// 与えられた手牌について、向聴数が最小になる時の面子・対子等の組み合わせを計算して格納する
///
/// 通常形・七対子の場合は面子・対子等の情報もVecに格納される。
/// 国士無双の場合は（今のところ）向聴数のみが格納される。
#[derive(Debug, Eq)]
pub struct HandAnalyzer {
    /// 向聴数：あと牌を何枚交換すれば聴牌できるかの最小数。聴牌状態が`0`、和了が`-1`。
    pub shanten: i32,
    /// どの和了形か
    pub form: WinningHandForm,
    /// 刻子（同じ牌が3枚）が入るVec
    pub same3: Vec<Same3>,
    /// 順子（連続した牌が3枚）が入るVec
    pub sequential3: Vec<Sequential3>,
    /// 対子（同じ牌が2枚）が入るVec
    pub same2: Vec<Same2>,
    /// 塔子（連続した牌が2枚）もしくは嵌張（順子の真ん中が抜けている2枚）が入るVec
    pub sequential2: Vec<Sequential2>,
    /// 面子や対子・塔子などを構成しない、単独の牌が入るVec
    pub single: Vec<TileType>,
}
impl Ord for HandAnalyzer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.shanten.cmp(&other.shanten)
    }
}

impl PartialOrd for HandAnalyzer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandAnalyzer {
    fn eq(&self, other: &Self) -> bool {
        self.shanten == other.shanten
    }
}

impl HandAnalyzer {
    /// 七対子・国士無双・通常の3つの和了形に対してそれぞれ向聴数を求め、最小のものを返す。
    /// # Examples
    ///
    /// ```
    /// use mahjong_rs::hand::*;
    /// use mahjong_rs::hand_info::hand_analyzer::*;
    /// use mahjong_rs::hand_info::winning_hand::*;
    ///
    /// // 通常型で和了る
    /// let nm_test_str = "222333444666s6z 6z";
    /// let nm_test = Hand::from(nm_test_str);
    /// let analyzer = HandAnalyzer::new(&nm_test);
    /// assert_eq!(
    ///   analyzer.shanten,
    ///   -1
    /// );
    /// assert_eq!(
    ///   analyzer.form,
    ///   WinningHandForm::Normal
    /// );
    /// ```
    pub fn new(hand: &Hand) -> HandAnalyzer {
        let sp = HandAnalyzer::new_by_form(hand, WinningHandForm::SevenPairs);
        let to = HandAnalyzer::new_by_form(hand, WinningHandForm::ThirteenOrphens);
        let normal = HandAnalyzer::new_by_form(hand, WinningHandForm::Normal);
        return min(min(sp, to), normal);
    }

    /// 和了形を指定して向聴数を計算する
    /// # Examples
    ///
    /// ```
    /// use mahjong_rs::hand::*;
    /// use mahjong_rs::hand_info::hand_analyzer::*;
    /// use mahjong_rs::hand_info::winning_hand::*;
    ///
    /// // 国士無双で和了る
    /// let to_test_str = "19m19p19s1234567z 1m";
    /// let to_test = Hand::from(to_test_str);
    /// assert_eq!(
    ///   HandAnalyzer::new_by_form(&to_test, WinningHandForm::ThirteenOrphens).shanten,
    ///   -1
    /// );
    ///
    /// // 七対子で和了る
    /// let sp_test_str = "1122m3344p5566s7z 7z";
    /// let sp_test = Hand::from(sp_test_str);
    /// assert_eq!(
    ///   HandAnalyzer::new_by_form(&sp_test, WinningHandForm::SevenPairs).shanten,
    ///   -1
    /// );
    ///
    /// // 通常型で和了る
    /// let nm_test_str = "1112345678999m 5m";
    /// let nm_test = Hand::from(nm_test_str);
    /// assert_eq!(
    ///   HandAnalyzer::new_by_form(&nm_test, WinningHandForm::Normal).shanten,
    ///   -1
    /// );
    /// ```
    pub fn new_by_form(hand: &Hand, form: WinningHandForm) -> HandAnalyzer {
        return match form {
            WinningHandForm::SevenPairs => HandAnalyzer::calc_seven_pairs(hand),
            WinningHandForm::ThirteenOrphens => HandAnalyzer::calc_thirteen_orphens(hand),
            WinningHandForm::Normal => HandAnalyzer::calc_normal_form(hand),
        };
    }

    /// 七対子への向聴数を計算する
    ///
    /// Vecへの詰め込みは`same2`（対子）以外は`single`（単独）に詰め込まれる。
    /// 七対子はVecを使用する役として断么九・混老頭・混一色・清一色と複合しうる
    fn calc_seven_pairs(hand: &Hand) -> HandAnalyzer {
        let mut pair: u32 = 0;
        let mut kind: u32 = 0;
        let mut t = hand.summarize_tiles();
        let mut same2: Vec<Same2> = Vec::new();

        for i in 0..Tile::LEN {
            if t[i] > 0 {
                kind += 1;
                if t[i] >= 2 {
                    pair += 1;
                    same2.push(Same2::new(i as TileType, i as TileType));
                    t[i] -= 2;
                }
            }
        }
        let num_to_win: i32 = (7 - pair + if kind < 7 { 7 - kind } else { 0 }) as i32;
        let mut single: Vec<TileType> = Vec::new();
        for i in 0..Tile::LEN {
            if t[i] > 0 {
                for _ in 0..i {
                    single.push(i as TileType);
                }
            }
        }
        return HandAnalyzer {
            shanten: num_to_win - 1,
            form: WinningHandForm::SevenPairs,
            same3: Vec::new(),
            sequential3: Vec::new(),
            same2,
            sequential2: Vec::new(),
            single,
        };
    }

    /// 国士無双への向聴数を計算する
    ///
    /// Vecへの詰め込みは未実装（詰め込んでも意味がない）
    fn calc_thirteen_orphens(hand: &Hand) -> HandAnalyzer {
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
        return HandAnalyzer {
            shanten: num_to_win - 1,
            form: WinningHandForm::ThirteenOrphens,
            same3: Vec::new(),
            sequential3: Vec::new(),
            same2: Vec::new(),
            sequential2: Vec::new(),
            single: Vec::new(),
        };
    }

    /// 通常の役への向聴数を計算する
    fn calc_normal_form(hand: &Hand) -> HandAnalyzer {
        let mut t = hand.summarize_tiles();
        let mut shanten: i32 = 100;
        // 計算用
        let mut same3: Vec<Same3> = Vec::new();
        let mut sequential3: Vec<Sequential3> = Vec::new();
        let mut same2: Vec<Same2> = Vec::new();
        let mut sequential2: Vec<Sequential2> = Vec::new();
        // 結果保存用
        let mut same3_result: Vec<Same3> = Vec::new();
        let mut sequential3_result: Vec<Sequential3> = Vec::new();
        let mut same2_result: Vec<Same2> = Vec::new();
        let mut sequential2_result: Vec<Sequential2> = Vec::new();
        let mut single_result: Vec<TileType> = Vec::new();
        // 先に独立した牌を抜き出しておく
        let mut independent_same3 = HandAnalyzer::count_independent_same_3(&mut t);
        let mut independent_sequential3 = HandAnalyzer::count_independent_sequential_3(&mut t);
        let mut independent_single = HandAnalyzer::count_independent_single(&mut t);

        // 雀頭を抜き出す
        for i in Tile::M1..=Tile::Z7 {
            if t[i as usize] >= 2 {
                same2.push(Same2::new(i, i));
                t[i as usize] -= 2;
                shanten = count_normal_shanten_recursively(
                    0,
                    &independent_same3,
                    &independent_sequential3,
                    &mut same3,
                    &mut sequential3,
                    &mut same2,
                    &mut sequential2,
                    &mut t,
                    &mut shanten,
                    &mut same3_result,
                    &mut sequential3_result,
                    &mut same2_result,
                    &mut sequential2_result,
                    &mut single_result,
                );
                t[i as usize] += 2;
                same2.pop();
            }
        }

        // 雀頭がない場合
        shanten = count_normal_shanten_recursively(
            0,
            &independent_same3,
            &independent_sequential3,
            &mut same3,
            &mut sequential3,
            &mut same2,
            &mut sequential2,
            &mut t,
            &mut shanten,
            &mut same3_result,
            &mut sequential3_result,
            &mut same2_result,
            &mut sequential2_result,
            &mut single_result,
        );

        // 最後に結合
        same3_result.append(&mut independent_same3);
        sequential3_result.append(&mut independent_sequential3);
        single_result.append(&mut independent_single);

        return HandAnalyzer {
            shanten,
            form: WinningHandForm::Normal,
            same3: same3_result,
            sequential3: sequential3_result,
            same2: same2_result,
            sequential2: sequential2_result,
            single: single_result,
        };
    }
    /// 独立した（順子になり得ない）刻子の数を返す
    fn count_independent_same_3(summarized_hand: &mut TileSummarize) -> Vec<Same3> {
        let mut result: Vec<Same3> = Vec::new();
        for i in Tile::M1..=Tile::Z7 {
            match i {
                Tile::M1 | Tile::P1 | Tile::S1 => {
                    if summarized_hand[i as usize] >= 3
                        && summarized_hand[i as usize + 1] == 0
                        && summarized_hand[i as usize + 2] == 0
                    {
                        summarized_hand[i as usize] -= 3;
                        result.push(Same3::new(i, i, i));
                    }
                }
                Tile::M2 | Tile::P2 | Tile::S2 => {
                    if summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] >= 3
                        && summarized_hand[i as usize + 1] == 0
                        && summarized_hand[i as usize + 2] == 0
                    {
                        summarized_hand[i as usize] -= 3;
                        result.push(Same3::new(i, i, i));
                    }
                }
                Tile::M3..=Tile::M7 | Tile::P3..=Tile::P7 | Tile::S3..=Tile::S7 => {
                    if summarized_hand[i as usize - 2] == 0
                        && summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] >= 3
                        && summarized_hand[i as usize + 1] == 0
                        && summarized_hand[i as usize + 2] == 0
                    {
                        summarized_hand[i as usize] -= 3;
                        result.push(Same3::new(i, i, i));
                    }
                }
                Tile::M8 | Tile::P8 | Tile::S8 => {
                    if summarized_hand[i as usize - 2] == 0
                        && summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] >= 3
                        && summarized_hand[i as usize + 1] == 0
                    {
                        summarized_hand[i as usize] -= 3;
                        result.push(Same3::new(i, i, i));
                    }
                }
                Tile::M9 | Tile::P9 | Tile::S9 => {
                    if summarized_hand[i as usize - 2] == 0
                        && summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] >= 3
                    {
                        summarized_hand[i as usize] -= 3;
                        result.push(Same3::new(i, i, i));
                    }
                }
                Tile::Z1..=Tile::Z7 => {
                    if summarized_hand[i as usize] >= 3 {
                        summarized_hand[i as usize] -= 3;
                        result.push(Same3::new(i, i, i));
                    }
                }
                _ => {
                    panic!("unknown tile index!");
                }
            }
        }
        return result;
    }

    /// 独立した（他の順子と複合し得ない）順子の数を返す
    /// i.e. xx567xxのような順子
    fn count_independent_sequential_3(summarized_hand: &mut TileSummarize) -> Vec<Sequential3> {
        let mut result: Vec<Sequential3> = Vec::new();
        // 先に一盃口の処理をしてから通常の処理
        for i in (1..=2).rev() {
            // 一萬、一筒、一索のインデックス位置
            for j in (Tile::M1..=Tile::S9).step_by(9) {
                // 一*～七*のインデックス位置
                for k in 0..=6 {
                    let l: usize = (j + k) as usize;
                    //三*以上のとき-2の牌が存在したらチェックしない
                    // i.e. チェック下限はxx345
                    if k >= 2 && summarized_hand[l - 2] > 0 {
                        continue;
                    }
                    //二*以上のとき-1の牌が存在したらチェックしない
                    // i.e. チェック下限はx234
                    if k >= 1 && summarized_hand[l - 1] > 0 {
                        continue;
                    }
                    //六*以下で+3の牌が存在したらチェックしない
                    // i.e. チェック上限は678x
                    if k <= 5 && summarized_hand[l + 3] > 0 {
                        continue;
                    }
                    //五*以下で+4の牌が存在したらチェックしない
                    // i.e. チェック上限は567xx
                    if k <= 4 && summarized_hand[l + 4] > 0 {
                        continue;
                    }
                    if summarized_hand[l] == i
                        && summarized_hand[l + 1] == i
                        && summarized_hand[l + 2] == i
                    {
                        summarized_hand[l] -= i;
                        summarized_hand[l + 1] -= i;
                        summarized_hand[l + 2] -= i;
                        for _ in 0..i {
                            result.push(Sequential3::new(
                                l as TileType,
                                (l + 1) as TileType,
                                (l + 2) as TileType,
                            ));
                        }
                    }
                }
            }
        }
        return result;
    }

    /// 独立した（他の順子や刻子などと複合し得ない）牌の数を返す
    fn count_independent_single(summarized_hand: &mut TileSummarize) -> Vec<TileType> {
        let mut result: Vec<TileType> = Vec::new();
        for i in Tile::M1..=Tile::Z7 {
            match i {
                Tile::M1 | Tile::P1 | Tile::S1 => {
                    if summarized_hand[i as usize] == 1
                        && summarized_hand[i as usize + 1] == 0
                        && summarized_hand[i as usize + 2] == 0
                    {
                        summarized_hand[i as usize] -= 1;
                        result.push(i);
                    }
                }
                Tile::M2 | Tile::P2 | Tile::S2 => {
                    if summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] == 1
                        && summarized_hand[i as usize + 1] == 0
                        && summarized_hand[i as usize + 2] == 0
                    {
                        summarized_hand[i as usize] -= 1;
                        result.push(i);
                    }
                }
                Tile::M3..=Tile::M7 | Tile::P3..=Tile::P7 | Tile::S3..=Tile::S7 => {
                    if summarized_hand[i as usize - 2] == 0
                        && summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] == 1
                        && summarized_hand[i as usize + 1] == 0
                        && summarized_hand[i as usize + 2] == 0
                    {
                        summarized_hand[i as usize] -= 1;
                        result.push(i);
                    }
                }
                Tile::M8 | Tile::P8 | Tile::S8 => {
                    if summarized_hand[i as usize - 2] == 0
                        && summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] == 1
                        && summarized_hand[i as usize + 1] == 0
                    {
                        summarized_hand[i as usize] -= 1;
                        result.push(i);
                    }
                }
                Tile::M9 | Tile::P9 | Tile::S9 => {
                    if summarized_hand[i as usize - 2] == 0
                        && summarized_hand[i as usize - 1] == 0
                        && summarized_hand[i as usize] == 1
                    {
                        summarized_hand[i as usize] -= 1;
                        result.push(i);
                    }
                }
                Tile::Z1..=Tile::Z7 => {
                    if summarized_hand[i as usize] == 1 {
                        summarized_hand[i as usize] -= 1;
                        result.push(i);
                    }
                }
                _ => {
                    panic!("unknown tile index!");
                }
            }
        }
        return result;
    }
}

/// 再帰的にシャンテン数が最小のものを探す
fn count_normal_shanten_recursively(
    idx: TileType,
    independent_same3: &Vec<Same3>,
    independent_sequential3: &Vec<Sequential3>,
    same3: &mut Vec<Same3>,
    sequential3: &mut Vec<Sequential3>,
    same2: &mut Vec<Same2>,
    sequential2: &mut Vec<Sequential2>,
    summarized_hand: &mut TileSummarize,

    shanten_min: &mut i32,
    same3_result: &mut Vec<Same3>,
    sequential3_result: &mut Vec<Sequential3>,
    same2_result: &mut Vec<Same2>,
    sequential2_result: &mut Vec<Sequential2>,
    single_result: &mut Vec<TileType>,
) -> i32 {
    count_same_or_sequential_3(
        idx,
        independent_same3,
        independent_sequential3,
        same3,
        sequential3,
        same2,
        sequential2,
        summarized_hand,
        shanten_min,
        same3_result,
        sequential3_result,
        same2_result,
        sequential2_result,
        single_result,
    );
    count_same_or_sequential_2(
        idx,
        independent_same3,
        independent_sequential3,
        same3,
        sequential3,
        same2,
        sequential2,
        summarized_hand,
        shanten_min,
        same3_result,
        sequential3_result,
        same2_result,
        sequential2_result,
        single_result,
    );
    let shanten = calc_normal_shanten(
        independent_same3,
        independent_sequential3,
        same3,
        sequential3,
        same2,
        sequential2,
    );
    if shanten < *shanten_min {
        *shanten_min = shanten;
        *same3_result = same3.clone();
        *sequential3_result = sequential3.clone();
        *same2_result = same2.clone();
        *sequential2_result = sequential2.clone();
        single_result.clear();
        for i in Tile::M1..=Tile::Z7 {
            for _ in 0..summarized_hand[i as usize] {
                single_result.push(i);
            }
        }
    }
    return *shanten_min;
}

/// 面子（刻子および順子）の数を返す
/// # returns
/// (刻子, 順子)
fn count_same_or_sequential_3(
    idx: TileType,
    independent_same3: &Vec<Same3>,
    independent_sequential3: &Vec<Sequential3>,
    same3: &mut Vec<Same3>,
    sequential3: &mut Vec<Sequential3>,
    same2: &mut Vec<Same2>,
    sequential2: &mut Vec<Sequential2>,
    summarized_hand: &mut TileSummarize,
    shanten_min: &mut i32,
    same3_result: &mut Vec<Same3>,
    sequential3_result: &mut Vec<Sequential3>,
    same2_result: &mut Vec<Same2>,
    sequential2_result: &mut Vec<Sequential2>,
    single_result: &mut Vec<TileType>,
) {
    for i in idx..=Tile::Z7 {
        // 刻子カウント
        if summarized_hand[i as usize] >= 3 {
            //block3 += 1;
            same3.push(Same3::new(i, i, i));
            summarized_hand[i as usize] -= 3;
            *shanten_min = count_normal_shanten_recursively(
                i,
                independent_same3,
                independent_sequential3,
                same3,
                sequential3,
                same2,
                sequential2,
                summarized_hand,
                shanten_min,
                same3_result,
                sequential3_result,
                same2_result,
                sequential2_result,
                single_result,
            );
            summarized_hand[i as usize] += 3;
            same3.pop();
        }

        //順子カウント
        if ((i >= Tile::M1 && i <= Tile::M7)
            || (i >= Tile::P1 && i <= Tile::P7)
            || (i >= Tile::S1 && i <= Tile::S7))
            && summarized_hand[i as usize] >= 1
            && summarized_hand[i as usize + 1] >= 1
            && summarized_hand[i as usize + 2] >= 1
        {
            //block3 += 1;
            sequential3.push(Sequential3::new(i, i + 1, i + 2));
            summarized_hand[i as usize] -= 1;
            summarized_hand[i as usize + 1] -= 1;
            summarized_hand[i as usize + 2] -= 1;
            *shanten_min = count_normal_shanten_recursively(
                i,
                independent_same3,
                independent_sequential3,
                same3,
                sequential3,
                same2,
                sequential2,
                summarized_hand,
                shanten_min,
                same3_result,
                sequential3_result,
                same2_result,
                sequential2_result,
                single_result,
            );
            summarized_hand[i as usize] += 1;
            summarized_hand[i as usize + 1] += 1;
            summarized_hand[i as usize + 2] += 1;
            sequential3.pop();
        }
    }
    //return (same3, sequential3);
}

/// 対子・塔子・嵌張カウント
/// # returns
/// (対子,塔子・嵌張)
fn count_same_or_sequential_2(
    idx: TileType,
    independent_same3: &Vec<Same3>,
    independent_sequential3: &Vec<Sequential3>,
    same3: &mut Vec<Same3>,
    sequential3: &mut Vec<Sequential3>,
    same2: &mut Vec<Same2>,
    sequential2: &mut Vec<Sequential2>,
    summarized_hand: &mut TileSummarize,
    shanten_min: &mut i32,
    same3_result: &mut Vec<Same3>,
    sequential3_result: &mut Vec<Sequential3>,
    same2_result: &mut Vec<Same2>,
    sequential2_result: &mut Vec<Sequential2>,
    single_result: &mut Vec<TileType>,
) {
    for i in idx..=Tile::Z7 {
        // 対子
        if summarized_hand[i as usize] == 2 {
            same2.push(Same2::new(i, i));
            summarized_hand[i as usize] -= 2;
            *shanten_min = count_normal_shanten_recursively(
                idx,
                independent_same3,
                independent_sequential3,
                same3,
                sequential3,
                same2,
                sequential2,
                summarized_hand,
                shanten_min,
                same3_result,
                sequential3_result,
                same2_result,
                sequential2_result,
                single_result,
            );
            summarized_hand[i as usize] += 2;
            same2.pop();
        }
        //数牌
        if i <= Tile::S9 && (i >= Tile::M1 && i <= Tile::M7)
            || (i >= Tile::P1 && i <= Tile::P7)
            || (i >= Tile::S1 && i <= Tile::S7)
        {
            // 塔子
            if summarized_hand[i as usize] >= 1 && summarized_hand[i as usize + 1] >= 1 {
                sequential2.push(Sequential2::new(i, i + 1));
                summarized_hand[i as usize] -= 1;
                summarized_hand[i as usize + 1] -= 1;
                *shanten_min = count_normal_shanten_recursively(
                    idx,
                    independent_same3,
                    independent_sequential3,
                    same3,
                    sequential3,
                    same2,
                    sequential2,
                    summarized_hand,
                    shanten_min,
                    same3_result,
                    sequential3_result,
                    same2_result,
                    sequential2_result,
                    single_result,
                );
                summarized_hand[i as usize] += 1;
                summarized_hand[i as usize + 1] += 1;
                sequential2.pop();
            }
            //嵌張
            if summarized_hand[i as usize] >= 1
                && summarized_hand[i as usize + 1] == 0
                && summarized_hand[i as usize + 2] >= 1
            {
                sequential2.push(Sequential2::new(i, i + 2));
                summarized_hand[i as usize] -= 1;
                summarized_hand[i as usize + 2] -= 1;
                *shanten_min = count_normal_shanten_recursively(
                    idx,
                    independent_same3,
                    independent_sequential3,
                    same3,
                    sequential3,
                    same2,
                    sequential2,
                    summarized_hand,
                    shanten_min,
                    same3_result,
                    sequential3_result,
                    same2_result,
                    sequential2_result,
                    single_result,
                );
                summarized_hand[i as usize] += 1;
                summarized_hand[i as usize + 2] += 1;
                sequential2.pop();
            }
        }
    }
    //return (same2, sequential2);
}

fn calc_normal_shanten(
    independent_same3: &Vec<Same3>,
    independent_sequential3: &Vec<Sequential3>,
    same3: &Vec<Same3>,
    sequential3: &Vec<Sequential3>,
    same2: &Vec<Same2>,
    sequential2: &Vec<Sequential2>,
) -> i32 {
    let block3 =
        independent_same3.len() + independent_sequential3.len() + same3.len() + sequential3.len();
    let block2 = same2.len() + sequential2.len();
    return 8 - (block3 * 2 + block2) as i32;
}

/// ユニットテスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// 七対子を聴牌
    fn zero_shanten_to_seven_pairs() {
        let test_str = "226699m99p228s66z 1z";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::SevenPairs).shanten,
            0
        );
    }
    #[test]
    /// 同じ牌が3枚ある状態で七対子を聴牌
    fn zero_shanten_to_seven_pairs_2() {
        let test_str = "226699m99p222s66z 1z";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::SevenPairs).shanten,
            0
        );
    }
    #[test]
    /// 国士無双を聴牌
    fn zero_shanten_to_orphens() {
        let test_str = "19m19p11s1234567z 5m";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::ThirteenOrphens).shanten,
            0
        );
    }

    #[test]
    /// 同じ牌が4枚ある状態で七対子は認められない（一向聴とみなす）
    fn seven_pairs_with_4_same_tiles() {
        let test_str = "1122m3344p5555s1z 1z";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::SevenPairs).shanten,
            1
        );
    }

    #[test]
    /// 立直で和了った
    fn win_by_ready_hand() {
        let test_str = "123m444p789s1112z 2z";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::Normal).shanten,
            -1
        );
    }

    #[test]
    /// 自風牌で和了った
    fn win_by_honor_tiles_players_wind() {
        let test_str = "333m456p1789s 333z 1s";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::Normal).shanten,
            -1
        );
    }

    #[test]
    /// 場風で和了った
    fn win_by_honor_tiles_prevailing_wind() {
        let test_str = "234567m6789s 111z 6s";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::Normal).shanten,
            -1
        );
    }
    #[test]
    /// 三元牌で和了った
    fn win_by_honor_tiles_dragons() {
        let test_str = "5m123456p888s 777z 5m";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::Normal).shanten,
            -1
        );
    }
    #[test]
    /// 断么九で和了った
    fn win_by_all_simples() {
        let test_str = "234m8s 567m 333p 456s 8s";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::Normal).shanten,
            -1
        );
    }

    #[test]
    /// 平和で和了った
    fn win_by_no_points() {
        let test_str = "123567m234p6799s 5s";
        let test = Hand::from(test_str);
        assert_eq!(
            HandAnalyzer::new_by_form(&test, WinningHandForm::Normal).shanten,
            -1
        );
    }
}
