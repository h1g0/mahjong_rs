//use crate::tile::Tile;
/*
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

/// 対子（同じ2枚）
pub struct Same2 {
    tiles: (Tile, Tile),
}
impl Same2 {
    fn new(&mut self, tile1: Tile, tile2: Tile) -> Same2 {
        if tile1.get() != tile2.get() {
            panic!("Not same tiles in `Same2`!");
        }
        return Same2 {
            tiles: (tile1, tile2),
        };
    }
}
impl BlockProperty for Same2 {
    fn has_1_or_9(&self) -> bool {
        return self.tiles.0.is_1_or_9();
    }
    fn has_honor(&self) -> bool {
        return self.tiles.0.is_honor();
    }

    fn is_character(&self) -> bool {
        return self.tiles.0.is_character();
    }

    fn is_circle(&self) -> bool {
        return self.tiles.0.is_circle();
    }

    fn is_bamboo(&self) -> bool {
        return self.tiles.0.is_bamboo();
    }
}
/// 刻子（同じ3枚）
pub struct Same3 {
    tiles: (Tile, Tile, Tile),
}
impl Same3 {
    fn new(&mut self, tile1: Tile, tile2: Tile, tile3: Tile) -> Same3 {
        if tile1.get() != tile2.get() || tile2.get() != tile3.get() {
            panic!("Not same tiles in `Same3`!");
        }
        return Same3 {
            tiles: (tile1, tile2, tile3),
        };
    }
}
impl BlockProperty for Same3 {
    fn has_1_or_9(&self) -> bool {
        return self.tiles.0.is_1_or_9();
    }
    fn has_honor(&self) -> bool {
        return self.tiles.0.is_honor();
    }
    fn is_character(&self) -> bool {
        return self.tiles.0.is_character();
    }

    fn is_circle(&self) -> bool {
        return self.tiles.0.is_circle();
    }

    fn is_bamboo(&self) -> bool {
        return self.tiles.0.is_bamboo();
    }
}

/// 順子（連続した3枚）
pub struct Sequential3 {
    tiles: (Tile, Tile, Tile),
}
impl Sequential3 {
    fn new(&mut self, tile1: Tile, tile2: Tile, tile3: Tile) -> Sequential3 {
        if !tile1.is_sequential_to(tile2) || !tile2.is_sequential_to(tile3) {
            panic!("Not same tiles in `Sequential3`!");
        }
        return Sequential3 {
            tiles: (tile1, tile2, tile3),
        };
    }
}
impl BlockProperty for Sequential3 {
    fn has_1_or_9(&self) -> bool {
        return self.tiles.0.is_1_or_9() || self.tiles.2.is_1_or_9();
    }
    fn has_honor(&self) -> bool {
        return self.tiles.0.is_honor() || self.tiles.2.is_honor();
    }
    fn is_character(&self) -> bool {
        return self.tiles.0.is_character();
    }

    fn is_circle(&self) -> bool {
        return self.tiles.0.is_circle();
    }

    fn is_bamboo(&self) -> bool {
        return self.tiles.0.is_bamboo();
    }
}
*/