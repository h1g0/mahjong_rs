/// 牌の種類を示す型
pub type TileType = u32;

/// 牌
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tile {
    index: TileType,
    red_dora: bool,
}

impl Tile {
    /// 一萬
    pub const M1: TileType = 0;
    /// 二萬
    pub const M2: TileType = 1;
    /// 三萬
    pub const M3: TileType = 2;
    /// 四萬
    pub const M4: TileType = 3;
    /// 五萬
    pub const M5: TileType = 4;
    /// 六萬
    pub const M6: TileType = 5;
    /// 七萬
    pub const M7: TileType = 6;
    /// 八萬
    pub const M8: TileType = 7;
    /// 九萬
    pub const M9: TileType = 8;
    /// 一筒
    pub const P1: TileType = 9;
    /// 二筒
    pub const P2: TileType = 10;
    /// 三筒
    pub const P3: TileType = 11;
    /// 四筒
    pub const P4: TileType = 12;
    /// 五筒
    pub const P5: TileType = 13;
    /// 六筒
    pub const P6: TileType = 14;
    /// 七筒
    pub const P7: TileType = 15;
    /// 八筒
    pub const P8: TileType = 16;
    /// 九筒
    pub const P9: TileType = 17;
    /// 一索
    pub const S1: TileType = 18;
    /// 二索
    pub const S2: TileType = 19;
    /// 三索
    pub const S3: TileType = 20;
    /// 四索
    pub const S4: TileType = 21;
    /// 五索
    pub const S5: TileType = 22;
    /// 六索
    pub const S6: TileType = 23;
    /// 七索
    pub const S7: TileType = 24;
    /// 八索
    pub const S8: TileType = 25;
    /// 九索
    pub const S9: TileType = 26;
    /// 東
    pub const Z1: TileType = 27;
    /// 南
    pub const Z2: TileType = 28;
    /// 西
    pub const Z3: TileType = 29;
    /// 北
    pub const Z4: TileType = 30;
    /// 白
    pub const Z5: TileType = 31;
    /// 發
    pub const Z6: TileType = 32;
    /// 中
    pub const Z7: TileType = 33;
    /// 牌の種類の数（インデックスは常にこの数よりも少ない整数値）
    pub const LEN: usize = 34;

    /// Unicode表記
    const CHARS: [char; Tile::LEN] = [
        '🀇', '🀈', '🀉', '🀊', '🀋', '🀌', '🀍', '🀎', '🀏', '🀙', '🀚', '🀛', '🀜', '🀝', '🀞', '🀟', '🀠', '🀡',
        '🀐', '🀑', '🀒', '🀓', '🀔', '🀕', '🀖', '🀗', '🀘', '🀀', '🀁', '🀂', '🀃', '🀆', '🀅', '🀄',
    ];
    /// Ascii表記
    const ASCII: [&'static str; Tile::LEN] = [
        "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "1p", "2p", "3p", "4p", "5p", "6p",
        "7p", "8p", "9p", "1s", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "1z", "2z", "3z",
        "4z", "5z", "6z", "7z",
    ];

    pub fn new(tile_type: TileType) -> Tile {
        return Tile { index: tile_type, red_dora: false };
    }

    pub fn get(&self) -> TileType {
        return self.index;
    }

    /// 萬子か否かを返す
    pub fn is_character(&self) -> bool {
        return matches!(self.index, Tile::M1..=Tile::M9);
    }
    /// 筒子か否かを返す
    pub fn is_circle(&self) -> bool {
        return matches!(self.index, Tile::P1..=Tile::P9);
    }
    /// 索子か否かを返す
    pub fn is_bamboo(&self) -> bool {
        return matches!(self.index, Tile::S1..=Tile::S9);
    }
    /// 風牌か否かを返す
    pub fn is_wind(&self) -> bool {
        return matches!(self.index, Tile::Z1..=Tile::Z4);
    }
    /// 三元牌か否かを返す
    pub fn is_dragon(&self) -> bool {
        return matches!(self.index, Tile::Z5..=Tile::Z7);
    }
    /// 字牌か否かを返す
    pub fn is_honor(&self) -> bool {
        return self.is_wind() || self.is_dragon();
    }

    /// 老頭牌か否かを返す
    pub fn is_1_or_9(&self) -> bool {
        return matches!(
            self.index,
            Tile::M1 | Tile::M9 | Tile::P1 | Tile::P9 | Tile::S1 | Tile::S9
        );
    }
    /// 么九牌（老頭牌＋字牌）か否かを返す
    pub fn is_1_9_honor(&self) -> bool {
        return self.is_1_or_9() || self.is_honor();
    }

    /// 対子（同じ2枚）か否かを返す
    pub fn is_same_to(&self, tile: Tile) -> bool {
        return self.get() == tile.get();
    }
    /// 搭子（連続した2枚）か否かを返す
    pub fn is_sequential_to(&self, tile: Tile) -> bool {
        // 字牌ならば連続はありえない
        if self.is_honor() {
            return false;
        }
        // 一萬・一筒・一索の時に1つ前（九萬・九筒）が来ても連続とはみなさない
        if matches!(self.index, Tile::M1 | Tile::P1 | Tile::S1) && self.get() == tile.get() + 1 {
            return false;
        }
        // 九萬・九筒・九索の時に1つ後（一筒・一索・東）が来ても連続とはみなさない
        if matches!(self.index, Tile::M9 | Tile::P9 | Tile::S9) && self.get() == tile.get() - 1 {
            return false;
        } else if self.get() == tile.get() - 1 || self.get() == tile.get() + 1 {
            return true;
        }
        return false;
    }

    pub fn to_char(&self) -> char {
        return Tile::CHARS[self.index as usize];
    }
    pub fn to_string(&self) -> String {
        return Tile::ASCII[self.index as usize].to_string();
    }

    pub fn from(tile_name: &str) -> Tile {
        let t = match tile_name {
            "1m" | "🀇" => Tile::M1,
            "2m" | "🀈" => Tile::M2,
            "3m" | "🀉" => Tile::M3,
            "4m" | "🀊" => Tile::M4,
            "5m" | "🀋" => Tile::M5,
            "6m" | "🀌" => Tile::M6,
            "7m" | "🀍" => Tile::M7,
            "8m" | "🀎" => Tile::M8,
            "9m" | "🀏" => Tile::M9,
            "1p" | "🀙" => Tile::P1,
            "2p" | "🀚" => Tile::P2,
            "3p" | "🀛" => Tile::P3,
            "4p" | "🀜" => Tile::P4,
            "5p" | "🀝" => Tile::P5,
            "6p" | "🀞" => Tile::P6,
            "7p" | "🀟" => Tile::P7,
            "8p" | "🀠" => Tile::P8,
            "9p" | "🀡" => Tile::P9,
            "1s" | "🀐" => Tile::S1,
            "2s" | "🀑" => Tile::S2,
            "3s" | "🀒" => Tile::S3,
            "4s" | "🀓" => Tile::S4,
            "5s" | "🀔" => Tile::S5,
            "6s" | "🀕" => Tile::S6,
            "7s" | "🀖" => Tile::S7,
            "8s" | "🀗" => Tile::S8,
            "9s" | "🀘" => Tile::S9,
            "1z" | "🀀" => Tile::Z1,
            "2z" | "🀁" => Tile::Z2,
            "3z" | "🀂" => Tile::Z3,
            "4z" | "🀃" => Tile::Z4,
            "5z" | "🀆" => Tile::Z5,
            "6z" | "🀅" => Tile::Z6,
            "7z" | "🀄" => Tile::Z7,
            _ => {
                panic!("unknown string")
            }
        };
        return Tile::new(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 萬子の属性テスト
    #[test]
    fn suit_char_test() {
        for i in Tile::M1..=Tile::M9 {
            let t = Tile::new(i);
            assert_eq!(t.is_character(), true);
            assert_eq!(t.is_bamboo(), false);
            assert_eq!(t.is_circle(), false);
            assert_eq!(t.is_honor(), false);
            assert_eq!(
                t.is_1_or_9(),
                if i == Tile::M1 || i == Tile::M9 {
                    true
                } else {
                    false
                }
            );
        }
    }

    /// 筒子の属性テスト
    #[test]
    fn suit_circle_test() {
        for i in Tile::P1..=Tile::P9 {
            let t = Tile::new(i);
            assert_eq!(t.is_character(), false);
            assert_eq!(t.is_bamboo(), false);
            assert_eq!(t.is_circle(), true);
            assert_eq!(t.is_honor(), false);
            assert_eq!(
                t.is_1_or_9(),
                if i == Tile::P1 || i == Tile::P9 {
                    true
                } else {
                    false
                }
            );
        }
    }
    /// 索子の属性テスト
    #[test]
    fn suit_bamboo_test() {
        for i in Tile::S1..=Tile::S9 {
            let t = Tile::new(i);
            assert_eq!(t.is_character(), false);
            assert_eq!(t.is_bamboo(), true);
            assert_eq!(t.is_circle(), false);
            assert_eq!(t.is_honor(), false);
            assert_eq!(
                t.is_1_or_9(),
                if i == Tile::S1 || i == Tile::S9 {
                    true
                } else {
                    false
                }
            );
        }
    }
    /// 風牌の属性テスト
    #[test]
    fn suit_wind_test() {
        for i in Tile::Z1..=Tile::Z4 {
            let t = Tile::new(i);
            assert_eq!(t.is_character(), false);
            assert_eq!(t.is_bamboo(), false);
            assert_eq!(t.is_circle(), false);
            assert_eq!(t.is_wind(), true);
            assert_eq!(t.is_dragon(), false);
            assert_eq!(t.is_honor(), true);
        }
    }
    /// 三元牌の属性テスト
    #[test]
    fn suit_dragon_test() {
        for i in Tile::Z5..=Tile::Z7 {
            let t = Tile::new(i);
            assert_eq!(t.is_character(), false);
            assert_eq!(t.is_bamboo(), false);
            assert_eq!(t.is_circle(), false);
            assert_eq!(t.is_wind(), false);
            assert_eq!(t.is_dragon(), true);
            assert_eq!(t.is_honor(), true);
        }
    }
    /// 字牌の属性テスト
    #[test]
    fn suit_honor_test() {
        for i in Tile::Z1..=Tile::Z7 {
            let t = Tile::new(i);
            assert_eq!(t.is_character(), false);
            assert_eq!(t.is_bamboo(), false);
            assert_eq!(t.is_circle(), false);
            assert_eq!(t.is_honor(), true);
        }
    }

    /// 対子テスト
    #[test]
    fn sameness_test() {
        // 1m→1mは対子
        assert_eq!(Tile::new(Tile::M1).is_same_to(Tile::new(Tile::M1)), true);
        // 1m→1pは対子ではない
        assert_eq!(Tile::new(Tile::M1).is_same_to(Tile::new(Tile::P1)), false);
        // 1z→1zは対子
        assert_eq!(Tile::new(Tile::Z1).is_same_to(Tile::new(Tile::Z1)), true);
    }

    /// 搭子テスト
    #[test]
    fn sequential_test() {
        // 1m→2mは搭子
        assert_eq!(
            Tile::new(Tile::M1).is_sequential_to(Tile::new(Tile::M2)),
            true
        );
        // 3p→3pは搭子ではない
        assert_eq!(
            Tile::new(Tile::P3).is_sequential_to(Tile::new(Tile::P3)),
            false
        );
        // 7s→8sは搭子
        assert_eq!(
            Tile::new(Tile::S7).is_sequential_to(Tile::new(Tile::S8)),
            true
        );
        // 1m→1pは搭子ではない
        assert_eq!(
            Tile::new(Tile::M1).is_sequential_to(Tile::new(Tile::P1)),
            false
        );
        // 9m→8mは搭子
        assert_eq!(
            Tile::new(Tile::M9).is_sequential_to(Tile::new(Tile::M8)),
            true
        );
        // 9m→1pは搭子ではない
        assert_eq!(
            Tile::new(Tile::M9).is_sequential_to(Tile::new(Tile::P1)),
            false
        );
        // 1s→9pは搭子ではない
        assert_eq!(
            Tile::new(Tile::S1).is_sequential_to(Tile::new(Tile::P9)),
            false
        );
        // 9s→1zは搭子ではない
        assert_eq!(
            Tile::new(Tile::S9).is_sequential_to(Tile::new(Tile::Z1)),
            false
        );
        // 1z→2zは搭子ではない
        assert_eq!(
            Tile::new(Tile::Z1).is_sequential_to(Tile::new(Tile::Z2)),
            false
        );
    }
}
