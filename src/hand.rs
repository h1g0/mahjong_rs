use super::tile::*;

/// 副露の種類
pub enum MeldType{
    /// チー
    Chi,
    /// ポン
    Pon,
    /// カン
    Kan,
}

/// どこから副露したか
pub enum MeldFrom{
    /// 上家
    Previous,
    /// 自家
    Myself,
    /// 下家
    Folloing,
    /// 対面
    Opposite,
}



/// 副露状態を表す構造体
pub struct Meld{
    tiles: Vec<Tile>,
    r#type: MeldType,
    from: MeldFrom,
}

/// 手牌
pub struct Hand{
    /// 現在の手牌（副露がなければ13枚）
    tiles: Vec<Tile>,
    /// ツモってきた牌
    drawn: Option<Tile>,
    /// 副露
    meld: Vec<Meld>,

}
impl Hand{
    pub fn new(tiles:Vec<Tile>,drawn:Option<Tile>)->Hand{
        if tiles.len()!= 13{
            panic!("`Hand.tiles.len()` must be 13.");
        }
        return Hand{
            tiles,
            drawn,
            meld: Vec::new(),
        }
    }

    fn sort(&mut self){
        self.tiles.sort();
    }

    pub fn to_emoji(&self)->String{
        let mut result = String::new();
        for i in 0..self.tiles.len(){
            result.push(self.tiles[i].to_char());
        }
        if let Some(tsumo) = self.drawn{
            result.push_str(&format!(" {}",tsumo.to_char()));
        }
        return result;
    }
    pub fn to_string(&self)->String{
        let mut result = String::new();
        for i in 0..self.tiles.len(){
            result.push_str(&self.tiles[i].to_string());
        }
        if let Some(tsumo) = self.drawn{
            result.push_str(&format!(" {}",tsumo.to_string()));
        }
        return result;
    }
}