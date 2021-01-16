use super::tile::*;
use std::collections::VecDeque;

/// 副露の種類
pub enum OpenType {
    /// チー
    Chi,
    /// ポン
    Pon,
    /// カン
    Kan,
}

/// 誰から副露したか
pub enum OpenFrom {
    /// 上家（チー・ポン・明カン）
    Previous,
    /// 自家（暗カンしたときのみ）
    Myself,
    /// 下家（ポン・明カン）
    Following,
    /// 対面（ポン・明カン）
    Opposite,
}

/// 副露状態を表す構造体
pub struct OpenTiles {
    /// 3枚の牌が入る。カンした時も3枚（4枚目は自明）
    tiles: [Tile; 3],
    /// 副露の種類
    r#type: OpenType,
    /// 誰から副露したか
    from: OpenFrom,
}

/// 手牌
pub struct Hand {
    /// 現在の手牌（副露がなければ13枚）
    tiles: Vec<Tile>,
    /// ツモってきた牌
    drawn: Option<Tile>,
    /// 副露
    opened: Vec<OpenTiles>,
}
impl Hand {
    pub fn new(tiles: Vec<Tile>, drawn: Option<Tile>) -> Hand {
        if tiles.len() != 13 {
            panic!("`Hand.tiles.len()` must be 13.");
        }
        return Hand {
            tiles,
            drawn,
            opened: Vec::new(),
        };
    }

    fn sort(&mut self) {
        self.tiles.sort();
    }
    /// 種類別に各牌の数をカウントする
    fn summarize_tiles(&self) -> Vec<TileType> {
        let mut result: Vec<TileType> = vec![0, Tile::LEN as u32];

        // 通常の手牌をカウント
        for i in 0..self.tiles.len() {
            result[self.tiles[i].get() as usize] += 1;
        }

        // 鳴いている牌があればカウント
        for i in 0..self.opened.len() {
            for j in 0..self.opened[i].tiles.len() {
                result[self.opened[i].tiles[j].get() as usize] += 1;
            }
        }

        // ツモった牌があればカウント
        if let Some(t) = self.drawn {
            result[t.get() as usize] += 1;
        }

        return result;
    }

    pub fn to_emoji(&self) -> String {
        let mut result = String::new();
        for i in 0..self.tiles.len() {
            result.push(self.tiles[i].to_char());
        }

        for i in 0..self.opened.len() {
            result.push_str(&format!(
                " {}{}{}",
                self.opened[i].tiles[0].to_char(),
                self.opened[i].tiles[1].to_char(),
                self.opened[i].tiles[2].to_char()
            ))
        }

        if let Some(tsumo) = self.drawn {
            result.push_str(&format!(" {}", tsumo.to_char()));
        }
        return result;
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.tiles.len() {
            result.push_str(&self.tiles[i].to_string());
        }

        for i in 0..self.opened.len() {
            result.push_str(&format!(
                " {}{}{}",
                self.opened[i].tiles[0].to_string(),
                self.opened[i].tiles[1].to_string(),
                self.opened[i].tiles[2].to_string()
            ))
        }

        if let Some(tsumo) = self.drawn {
            result.push_str(&format!(" {}", tsumo.to_string()));
        }
        return result;
    }

    fn make_short_str(mut tiles: Vec<Tile>) -> String {
        if tiles.len() == 0 {
            return String::from("");
        } else if tiles.len() == 1 {
            return tiles[1].to_string();
        }
        tiles.sort();
        let mut result = String::new();
        let mut prev_suit = 'x';
        for i in 0..tiles.len() {
            let now_suit = tiles[i].to_string().chars().nth(1).unwrap();
            if i > 0 {
                result.push(tiles[i - 1].to_string().chars().nth(0).unwrap());
                if now_suit != prev_suit {
                    result.push(prev_suit);
                }
            }
            if i == tiles.len() - 1 {
                result.push(tiles[i].to_string().chars().nth(0).unwrap());
                result.push(now_suit);
                break;
            }
            prev_suit = now_suit;
        }
        return result;
    }
    pub fn to_short_string(&self) -> String {
        let tiles = self.tiles.clone();
        let mut result = Hand::make_short_str(tiles);
        if let Some(tsumo) = self.drawn {
            result.push_str(&format!(" {}", tsumo.to_string()));
        }
        return result;
    }

    fn str_to_tiles(hand_str: &str) -> Vec<Tile> {
        let mut result: Vec<Tile> = Vec::new();

        let mut stack: VecDeque<char> = VecDeque::new();
        while let Some(c) = hand_str.chars().next() {
            if matches!(c, '1'..='9') {
                stack.push_back(c);
            } else if matches!(c, 'm' | 'p' | 's' | 'z') {
                while let Some(t) = stack.pop_front() {
                    result.push(Tile::from(&format!("{}{}", t, c)));
                }
            }
        }
        return result;
    }

    pub fn from(hand_str: &str) -> Hand {
        todo!();
    }
}
