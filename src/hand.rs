use super::tile::*;

/// 手牌
pub struct Hand{
    tiles: Vec<Tile>,
}
impl Hand{
    pub fn new(tiles:Vec<Tile>)->Hand{
        if tiles.len()!= 13{
            panic!("`Hand.tiles.len()` must be 13.");
        }
        return Hand{
            tiles,
        }
    }

    pub fn to_string(&self)->String{
        let mut result = String::new();
        for i in 0..self.tiles.len(){
            result.push(self.tiles[i].to_char());
        }
        return result;
    }
    pub fn to_ascii(&self)->String{
        let mut result = String::new();
        for i in 0..self.tiles.len(){
            result.push_str(&self.tiles[i].to_ascii());
        }
        return result;
    }
}