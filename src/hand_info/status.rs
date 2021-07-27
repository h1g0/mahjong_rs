use crate::tile::Wind;

/// 手牌の（牌以外の）状態
pub struct Status {
    /// 立直したか
    pub has_claimed_ready: bool,
    /// 鳴いたか
    pub has_claimed_open: bool,
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
            player_wind: Wind::East,
            prevailing_wind: Wind::East,
        }
    }
}
