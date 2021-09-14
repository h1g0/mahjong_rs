use crate::tile::Wind;

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
