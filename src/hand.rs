use super::tile::*;

/// 副露の種類
pub enum FuroType{
    /// チー
    Chi,
    /// ポン
    Pon,
    /// カン
    Kan,
}

/// どこから副露したか
pub enum FuroFrom{
    /// 上家
    Previous,
    /// 自家
    Myself,
    /// 下家
    Folloing,
    /// 対面
    Opposite,
}


pub struct Furo{
    tiles: Vec<Tile>,
    furo_type: FuroType,
    from: FuroFrom,
}

/// 手牌
pub struct Hand{
    /// 現在の手牌（副露がなければ13枚）
    tiles: Vec<Tile>,
    /// ツモってきた牌
    drawn: Option<Tile>,
    /// 副露
    furo: Vec<Furo>,

}
impl Hand{
    pub fn new(tiles:Vec<Tile>,drawn:Option<Tile>)->Hand{
        if tiles.len()!= 13{
            panic!("`Hand.tiles.len()` must be 13.");
        }
        return Hand{
            tiles,
            drawn,
            furo: Vec::new(),
        }
    }

    fn sort(&mut self){
        self.tiles.sort();
    }

    pub fn to_string(&self)->String{
        let mut result = String::new();
        for i in 0..self.tiles.len(){
            result.push(self.tiles[i].to_char());
        }
        if let Some(tsumo) = self.drawn{
            result.push_str(&format!(" {}",tsumo.to_char()));
        }
        return result;
    }
    pub fn to_ascii(&self)->String{
        let mut result = String::new();
        for i in 0..self.tiles.len(){
            result.push_str(&self.tiles[i].to_ascii());
        }
        if let Some(tsumo) = self.drawn{
            result.push_str(&format!(" {}",tsumo.to_ascii()));
        }
        return result;
    }
}