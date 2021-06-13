use super::super::tile::Tile;

/// ブロック（対子、順子、刻子、槓子etc）の振る舞いを定義する
trait BlockProperty {
    /// 么九牌が含まれているか
    fn have_1_or_9(&self) -> bool;
    /// 字牌が含まれているか
    fn have_honor(&self) -> bool;
}

/// 対子（同じ2枚）
pub struct Same2 {
    tiles: (Tile, Tile),
}

impl BlockProperty for Same2 {
    fn have_1_or_9(&self) -> bool {
        return if self.tiles.0.is_1_or_9() || self.tiles.1.is_1_or_9() {
            true
        } else {
            false
        };
    }
    fn have_honor(&self) -> bool {
        return if self.tiles.0.is_honor() || self.tiles.1.is_honor() {
            true
        } else {
            false
        };
    }
}
