mod tile;
use tile::*;

mod hand;
use hand::*;

fn main() {
    let hand = Hand::new(vec!(Tile::new(Tile::M1);13));
    println!("{}",hand.to_ascii());
}
