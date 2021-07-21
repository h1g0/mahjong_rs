/// ルールを設定する
pub struct Rules{
    /// 喰いタンありかなしか（デフォルトはあり）
    pub openned_all_simples: bool,
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            openned_all_simples: true,
        }
    }

}