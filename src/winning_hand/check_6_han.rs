use crate::hand_info::hand_analyzer::*;
use crate::hand_info::status::*;
use crate::settings::*;
use crate::winning_hand::name::*;

/// 清一色
pub fn check_flush(
    hand: &HandAnalyzer,
    status: &Status,
    settings: &Settings,
) -> (&'static str, bool, u32) {
    let name = get(
        Kind::Flush,
        status.has_claimed_open,
        settings.display_lang,
    );
    if !has_won(hand) {
        return (name, false, 0);
    }
    todo!();
}

/// ユニットテスト
#[cfg(test)]
mod tests {}
