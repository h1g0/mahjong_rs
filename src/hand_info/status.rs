use crate::tile::{Tile, TileSummarize, TileType, Wind};

/// 手牌の（牌以外の）状態
pub struct Status {
    /// 立直したか
    pub has_claimed_ready: bool,
    /// 鳴いたか
    pub has_claimed_open: bool,
    /// 自摸しているか
    pub is_self_picked: bool,
    /// 一発が有効な間立てるフラグ
    pub is_one_shot: bool,
    /// 自風
    pub player_wind: Wind,
    /// 場風
    pub prevailing_wind: Wind,
}

impl Status {
    pub fn new() -> Status {
        Status {
            has_claimed_ready: false,
            has_claimed_open: false,
            is_self_picked: false,
            is_one_shot: false,
            player_wind: Wind::East,
            prevailing_wind: Wind::East,
        }
    }
}

/// 与えられた牌と手牌の構成から両面待ちか判定する
pub fn is_two_sided_wait(tile: TileType, counts: &TileSummarize) -> bool {
    // 字牌は両面待ちになり得ない
    if !(Tile::M1..=Tile::S9).contains(&tile) {
        return false;
    }

    let offset = if (Tile::M1..=Tile::M9).contains(&tile) {
        tile - Tile::M1 + 1
    } else if (Tile::P1..=Tile::P9).contains(&tile) {
        tile - Tile::P1 + 1
    } else {
        tile - Tile::S1 + 1
    };

    // 左側が存在する形 : xx[tile-2][tile-1] + tile かつ123または789にならない
    if offset >= 3
        && counts[(tile - 1) as usize] > 0
        && counts[(tile - 2) as usize] > 0
        && offset != 3
        && offset != 9
    {
        return true;
    }

    // 右側が存在する形 : tile + [tile+1][tile+2] かつ123または789にならない
    if offset <= 7
        && counts[(tile + 1) as usize] > 0
        && counts[(tile + 2) as usize] > 0
        && offset != 1
        && offset != 7
    {
        return true;
    }

    false
}
