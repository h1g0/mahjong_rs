/// 牌の種類を示す型
pub type TileType = u32;

/// 牌
#[derive(Debug,Clone,Copy)]
pub struct Tile{
    index:TileType,
}

impl Tile{
    /// 一萬
    pub const M1: TileType = 0;
    /// 二萬
    pub const M2: TileType = 1;
    /// 三萬
    pub const M3: TileType = 2;
    /// 四萬
    pub const M4: TileType = 3;
    /// 五萬
    pub const M5: TileType = 4;
    /// 六萬
    pub const M6: TileType = 5;
    /// 七萬
    pub const M7: TileType = 6;
    /// 八萬
    pub const M8: TileType = 7;
    /// 九萬
    pub const M9: TileType = 8;
    /// 一筒
    pub const P1: TileType = 9;
    /// 二筒
    pub const P2: TileType = 10;
    /// 三筒
    pub const P3: TileType = 11;
    /// 四筒
    pub const P4: TileType = 12;
    /// 五筒
    pub const P5: TileType = 13;
    /// 六筒
    pub const P6: TileType = 14;
    /// 七筒
    pub const P7: TileType = 15;
    /// 八筒
    pub const P8: TileType = 16;
    /// 九筒
    pub const P9: TileType = 17;
    /// 一索
    pub const S1: TileType = 18;
    /// 二索
    pub const S2: TileType = 19;
    /// 三索
    pub const S3: TileType = 20;
    /// 四索
    pub const S4: TileType = 21;
    /// 五索
    pub const S5: TileType = 22;
    /// 六索
    pub const S6: TileType = 23;
    /// 七索
    pub const S7: TileType = 24;
    /// 八索
    pub const S8: TileType = 25;
    /// 九索
    pub const S9: TileType = 26;
    /// 東
    pub const Z1: TileType = 27;
    /// 南
    pub const Z2: TileType = 28;
    /// 西
    pub const Z3: TileType = 29;
    /// 北
    pub const Z4: TileType = 30;
    /// 白
    pub const Z5: TileType = 31;
    /// 發
    pub const Z6: TileType = 32;
    /// 中
    pub const Z7: TileType = 33;
    /// 牌の種類の数（インデックスは常にこの数よりも少ない整数値）
    pub const LEN: usize = 34;

    /// Unicode表記
    const CHARS: [char;Tile::LEN] = [
        '🀇','🀈','🀉','🀊','🀋','🀌','🀍','🀎','🀏',
        '🀙','🀚','🀛','🀜','🀝','🀞','🀟','🀠','🀡',
        '🀐','🀑','🀒','🀓','🀔','🀕','🀖','🀗','🀘',
        '🀀','🀁','🀂','🀃',
        '🀆','🀅','🀄'];
    /// Ascii表記
    const ASCII: [&'static str;Tile::LEN] = [
        "1m","2m","3m","4m","5m","6m","7m","8m","9m",
        "1p","2p","3p","4p","5p","6p","7p","8p","9p",
        "1s","2s","3s","4s","5s","6s","7s","8s","9s",
        "1z","2z","3z","4z",
        "5z","6z","7z"];

    pub fn new(tile_type: TileType)->Tile{
        return Tile{
            index: tile_type,
        }
    }

    pub fn get(&self)->TileType{
        return self.index;
    }

    /// 萬子か否かを返す
    pub fn is_character(&self)->bool{
        return matches!(self.index,Tile::M1 ..= Tile::M9);
    }
    /// 筒子か否かを返す
    pub fn is_circle(&self)->bool{
        return matches!(self.index,Tile::P1 ..= Tile::P9);
    }
    /// 索子か否かを返す
    pub fn is_bamboo(&self)->bool{
        return matches!(self.index,Tile::S1 ..= Tile::S9);
    }
    /// 風牌か否かを返す
    pub fn is_wind(&self)->bool{
        return matches!(self.index,Tile::Z1 ..= Tile::Z4);
    }
    /// 三元牌か否かを返す
    pub fn is_dragon(&self)->bool{
        return matches!(self.index,Tile::Z1 ..= Tile::Z4);
    }
    /// 字牌か否かを返す
    pub fn is_honor_tile(&self)->bool{
        return self.is_wind() || self.is_dragon();
    }

    pub fn to_char(&self)->char{
        return Tile::CHARS[self.index as usize];
    }
    pub fn to_ascii(&self)->String{
        return Tile::ASCII[self.index as usize].to_string();
    }

    pub fn from_string(tile_name: &str)->TileType{
        match tile_name{
            "1m"|"一萬" => Tile::M1,
            "2m"|"二萬" => Tile::M2,
            "3m"|"三萬" => Tile::M3,
            "4m"|"四萬" => Tile::M4,
            "5m"|"五萬" => Tile::M5,
            "6m"|"六萬" => Tile::M6,
            "7m"|"七萬" => Tile::M7,
            "8m"|"八萬" => Tile::M8,
            "9m"|"九萬" => Tile::M9,
            _ => {panic!("unknown string")}
        }
    }
}