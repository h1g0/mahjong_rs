use crate::tile::Wind;

/// 手牌の（牌以外の）状態
pub struct Status {
    /// 立直したか
    has_claimed_ready: bool,
    /// 鳴いたか
    has_claimed_open: bool,
    /// 自風
    player_wind: Wind,
    /// 場風
    prevailed_wind: Wind,
}

impl Status {
    pub fn new() -> Status {
        Status {
            has_claimed_ready: false,
            has_claimed_open: false,
            player_wind: Wind::East,
            prevailed_wind: Wind::East,
        }
    }
}
