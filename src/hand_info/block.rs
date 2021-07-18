use std::cmp::Ordering;

use crate::tile::*;

/// ブロック（対子、順子、刻子）の振る舞いを定義する
trait BlockProperty {
    /// 么九牌が含まれているか
    fn has_1_or_9(&self) -> bool;
    /// 字牌が含まれているか
    fn has_honor(&self) -> bool;
    /// 萬子のブロックか
    fn is_character(&self) -> bool;
    /// 筒子のブロックか
    fn is_circle(&self) -> bool;
    /// 索子のブロックか
    fn is_bamboo(&self) -> bool;
}

fn has_1_or_9(t: TileType) -> bool {
    if t >= Tile::LEN as TileType {
        panic!("has_1_or_9: invalid tile type");
    }
    match t {
        Tile::M1 | Tile::M9 => true,
        Tile::P1 | Tile::P9 => true,
        Tile::S1 | Tile::S9 => true,
        _ => false,
    }
}

fn has_honor(t: TileType) -> bool {
    if t >= Tile::LEN as TileType {
        panic!("has_honor: invalid tile type");
    }
    match t {
        Tile::Z1..=Tile::Z7 => true,
        _ => false,
    }
}

fn is_character(t: TileType) -> bool {
    if t >= Tile::LEN as TileType {
        panic!("is_character: invalid tile type");
    }
    match t {
        Tile::M1..=Tile::M9 => true,
        _ => false,
    }
}

fn is_circle(t: TileType) -> bool {
    if t >= Tile::LEN as TileType {
        panic!("is_circle: invalid tile type");
    }
    match t {
        Tile::P1..=Tile::P9 => true,
        _ => false,
    }
}

fn is_bamboo(t: TileType) -> bool {
    if t >= Tile::LEN as TileType {
        panic!("is_bamboo: invalid tile type");
    }
    match t {
        Tile::S1..=Tile::S9 => true,
        _ => false,
    }
}

fn is_same_suit(t1: TileType, t2: TileType) -> bool {
    if t1 >= Tile::LEN as TileType {
        panic!("is_same_suit: invalid tile type");
    }
    if t2 >= Tile::LEN as TileType {
        panic!("is_same_suit: invalid tile type");
    }
    if matches!(t1, Tile::M1..=Tile::M9) {
        return matches!(t2, Tile::M1..=Tile::M9);
    }
    if matches!(t1, Tile::P1..=Tile::P9) {
        return matches!(t2, Tile::P1..=Tile::P9);
    }
    if matches!(t1, Tile::S1..=Tile::S9) {
        return matches!(t2, Tile::S1..=Tile::S9);
    }
    if matches!(t1, Tile::Z1..=Tile::Z7) {
        return matches!(t2, Tile::Z1..=Tile::Z7);
    }

    return false;
}

#[derive(Debug, Eq)]
/// 対子（同じ2枚）
pub struct Same2 {
    tiles: [TileType; 2],
}
impl Same2 {
    pub fn new(tile1: TileType, tile2: TileType) -> Same2 {
        if tile1 != tile2 {
            panic!("Not same tiles in `Same2`!");
        }
        Same2 {
            tiles: [tile1, tile2],
        }
    }
    pub fn get(&self) -> [TileType; 2] {
        self.tiles
    }
}
impl Ord for Same2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tiles.cmp(&other.tiles)
    }
}

impl PartialOrd for Same2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Same2 {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}
impl BlockProperty for Same2 {
    fn has_1_or_9(&self) -> bool {
        // 2枚は同じ牌なので１枚目のみ調べれば良い
        has_1_or_9(self.tiles[0])
    }

    fn has_honor(&self) -> bool {
        // 2枚は同じ牌なので１枚目のみ調べれば良い
        has_honor(self.tiles[0])
    }

    fn is_character(&self) -> bool {
        // 2枚は同じ牌なので１枚目のみ調べれば良い
        is_character(self.tiles[0])
    }

    fn is_circle(&self) -> bool {
        // 2枚は同じ牌なので１枚目のみ調べれば良い
        is_circle(self.tiles[0])
    }

    fn is_bamboo(&self) -> bool {
        // 2枚は同じ牌なので１枚目のみ調べれば良い
        is_bamboo(self.tiles[0])
    }
}

#[derive(Debug, Eq)]
/// 刻子（同じ3枚）
pub struct Same3 {
    tiles: [TileType; 3],
}
impl Same3 {
    pub fn new(tile1: TileType, tile2: TileType, tile3: TileType) -> Same3 {
        if tile1 != tile2 || tile1 != tile3 {
            panic!("Not same tiles in `Same3`!");
        }
        Same3 {
            tiles: [tile1, tile2, tile3],
        }
    }
    /// 牌の配列を返す
    pub fn get(&self) -> [TileType; 3] {
        self.tiles
    }
}
impl Ord for Same3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tiles.cmp(&other.tiles)
    }
}

impl PartialOrd for Same3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Same3 {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}
impl BlockProperty for Same3 {
    fn has_1_or_9(&self) -> bool {
        // 3枚は同じ牌なので１枚目のみ調べれば良い
        has_1_or_9(self.tiles[0])
    }
    fn has_honor(&self) -> bool {
        // 3枚は同じ牌なので１枚目のみ調べれば良い
        has_honor(self.tiles[0])
    }
    fn is_character(&self) -> bool {
        // 3枚は同じ牌なので１枚目のみ調べれば良い
        is_character(self.tiles[0])
    }
    fn is_circle(&self) -> bool {
        // 3枚は同じ牌なので１枚目のみ調べれば良い
        is_circle(self.tiles[0])
    }
    fn is_bamboo(&self) -> bool {
        // 3枚は同じ牌なので１枚目のみ調べれば良い
        is_bamboo(self.tiles[0])
    }
}

#[derive(Debug, Eq)]
/// 塔子（連続した牌が2枚）もしくは嵌張（順子の真ん中が抜けている2枚）
pub struct Sequential2 {
    tiles: [TileType; 2],
}
impl Sequential2 {
    pub fn new(tile1: TileType, tile2: TileType) -> Sequential2 {
        // まず連続でなければパニック
        if !(tile2 == tile1 + 1 || tile2 == tile1 + 2) {
            panic!("Not sequential tiles in `Sequential2`!");
        }
        // 字牌は順子にならない
        if has_honor(tile1) || has_honor(tile2) {
            panic!("Cannot assign Honor tiles to `Sequential2`!");
        }
        if !is_same_suit(tile1, tile2) {
            panic!("Cannot assign different suits to `Sequential2`!");
        }
        Sequential2 {
            tiles: [tile1, tile2],
        }
    }
    /// 牌の配列を返す
    pub fn get(&self) -> [TileType; 2] {
        self.tiles
    }
}
impl Ord for Sequential2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tiles.cmp(&other.tiles)
    }
}

impl PartialOrd for Sequential2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sequential2 {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}
impl BlockProperty for Sequential2 {
    fn has_1_or_9(&self) -> bool {
        has_1_or_9(self.tiles[0]) || has_1_or_9(self.tiles[1])
    }
    fn has_honor(&self) -> bool {
        //字牌は塔子にならない
        false
    }
    fn is_character(&self) -> bool {
        is_character(self.tiles[0])
    }
    fn is_circle(&self) -> bool {
        is_circle(self.tiles[0])
    }
    fn is_bamboo(&self) -> bool {
        is_bamboo(self.tiles[0])
    }
}

#[derive(Debug, Eq)]
/// 順子（連続した3枚）
pub struct Sequential3 {
    tiles: [TileType; 3],
}
impl Sequential3 {
    pub fn new(tile1: TileType, tile2: TileType, tile3: TileType) -> Sequential3 {
        // まず連続でなければパニック
        if tile2 != tile1 + 1 || tile3 != tile2 + 1 {
            panic!("Not sequential tiles in `Sequential3`!");
        }
        // 字牌は順子にならない
        if has_honor(tile1) || has_honor(tile2) || has_honor(tile3) {
            panic!("Cannot assign Honor tiles to `Sequential3`!");
        }

        if !is_same_suit(tile1, tile2) || !is_same_suit(tile2, tile3) {
            panic!("Cannot assign different suits to `Sequential3`!");
        }
        Sequential3 {
            tiles: [tile1, tile2, tile3],
        }
    }
    /// 牌の配列を返す
    pub fn get(&self) -> [TileType; 3] {
        self.tiles
    }
}
impl Ord for Sequential3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tiles.cmp(&other.tiles)
    }
}

impl PartialOrd for Sequential3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sequential3 {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}
impl BlockProperty for Sequential3 {
    fn has_1_or_9(&self) -> bool {
        has_1_or_9(self.tiles[0]) || has_1_or_9(self.tiles[2])
    }
    fn has_honor(&self) -> bool {
        //字牌は順子にならない
        false
    }
    fn is_character(&self) -> bool {
        is_character(self.tiles[0])
    }
    fn is_circle(&self) -> bool {
        is_circle(self.tiles[0])
    }
    fn is_bamboo(&self) -> bool {
        is_bamboo(self.tiles[0])
    }
}

/// ユニットテスト
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_same2_normal() {
        assert_eq!(Same2::new(Tile::M1, Tile::M1).get(), [Tile::M1; 2]);
    }
    #[test]
    #[should_panic]
    fn test_same2_panics_when_not_same() {
        Same2::new(Tile::M1, Tile::M2);
    }
    #[test]
    fn test_same3_normal() {
        assert_eq!(
            Same3::new(Tile::M1, Tile::M1, Tile::M1).get(),
            [Tile::M1; 3]
        );
    }
    #[test]
    #[should_panic]
    fn test_same3_panics_when_not_same() {
        Same3::new(Tile::M1, Tile::M1, Tile::M2);
    }

    #[test]
    fn test_sequential2_normal() {
        assert_eq!(
            Sequential2::new(Tile::M1, Tile::M2).get(),
            [Tile::M1, Tile::M2]
        );
    }
    #[test]
    fn test_sequential2_normal2() {
        assert_eq!(
            Sequential2::new(Tile::M1, Tile::M3).get(),
            [Tile::M1, Tile::M3]
        );
    }
    #[test]
    #[should_panic]
    fn test_sequential2_panics_when_not_sequential() {
        Sequential2::new(Tile::M1, Tile::M4);
    }
    #[test]
    #[should_panic]
    fn test_sequential2_panics_when_honor() {
        Sequential2::new(Tile::Z1, Tile::Z2);
    }
    #[test]
    #[should_panic]
    fn test_sequential2_panics_when_other_kind() {
        Sequential2::new(Tile::M9, Tile::P1);
    }
    #[test]
    #[should_panic]
    fn test_sequential2_panics_when_other_kind2() {
        Sequential2::new(Tile::P8, Tile::S1);
    }
    #[test]
    fn test_sequential3_normal() {
        assert_eq!(
            Sequential3::new(Tile::M1, Tile::M2, Tile::M3).get(),
            [Tile::M1, Tile::M2, Tile::M3]
        );
    }
    #[test]
    #[should_panic]
    fn test_sequential3_panics_when_not_sequential() {
        Sequential3::new(Tile::M1, Tile::M2, Tile::M4);
    }
    #[test]
    #[should_panic]
    fn test_sequential3_panics_when_honor() {
        Sequential3::new(Tile::Z1, Tile::Z2, Tile::Z3);
    }
    #[test]
    #[should_panic]
    fn test_sequential3_panics_when_other_kind() {
        Sequential3::new(Tile::M8, Tile::M9, Tile::P1);
    }
    #[test]
    #[should_panic]
    fn test_sequential3_panics_when_other_kind2() {
        Sequential3::new(Tile::P9, Tile::S1, Tile::S2);
    }
}
