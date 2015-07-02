mod board;

use board::Board;
use board::Tile;
use board::Coord;


fn main() {
    let mut board = Board::new();

    board.place(Coord{ x: 10, y: 10}, Tile::White).ok().expect("blah");
    board.place(Coord{ x: 11, y: 10}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 9, y: 10}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 10, y: 9}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 10, y: 11}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 10, y: 10}, Tile::White).err().expect("shoulda yelled at u");
}


