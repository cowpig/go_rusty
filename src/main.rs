use std::fmt;
use std::result::Result::{self, Ok, Err};
use std::ops::Index;

const BOARD_SIZE: usize = 19;

#[derive(Clone, Copy)]
enum Tile {
    White,
    Black,
    Empty
}

impl fmt::Display for Tile {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(
            match *self {
                Tile::White => "X",
                Tile::Black => "O",
                Tile::Empty => ".",
            }
        )
    }
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize
}

struct Board {
    tiles: [[Tile; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {

    fn new() -> Board {
        Board{ tiles: [[Tile::Empty; BOARD_SIZE]; BOARD_SIZE] }
    }

    fn place(&mut self, index: Coord, color: Tile) -> Result<(), String> {
        if index.x >= BOARD_SIZE && index.y >= BOARD_SIZE {
            return Err(format!("place out-of-bound: {:?}", index));
        }

        try!(match color {
            Tile::Empty => Err(format!("invalid color: {}", color)),
            _ => Ok(())
        });

        let tile = &mut self.tiles[index.y][index.x];

        try!(match *tile {
            Tile::Empty => Ok(()),
            _ => Err(format!("place on existing piece: {:?} -> {}", index, tile))
        });

        *tile = color;

        Ok(())
    }
}

impl Index<Coord> for Board {
    type Output = Tile;

    fn index(&self, index:Coord) -> &Tile {
        &self.tiles[index.y][index.x]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for line in &self.tiles {
            for tile in line {
                try!(tile.fmt(formatter));
                try!(formatter.write_str(" "));
            }
            try!(formatter.write_str("\n"));
        }

        Ok(())
    }
}

// fn get_captures(board:&Board, coord:Coord ) -> Set<Coord> {

//     Set::new()
// }

// fn get_captures(board:&Board, coord:Coord, group:Set<Coord>, color:Tile) -> Set<Coord> {
//     for neighbor in neighbors(group) {
//         if !(group.contains(neighbor) || board[coord] == color) {

//         }
//     }
// }

fn neighbors(coord:Coord) -> Vec<Coord> {
    let mut output:Vec<Coord> = Vec::new();

    for offset in &[-1, 1] {
        if coord.x + offset < BOARD_SIZE {
            output.push(Coord{ x: coord.x + offset, y: coord.y });
        }
        if coord.y + offset < BOARD_SIZE {
            output.push(Coord{ x: coord.x, y: coord.y + offset });
        }
    }
    output
}

fn main() {
    let mut board = Board::new();

    board.place(Coord{ x: 10, y: 10}, Tile::White).ok().expect("blah");
    board.place(Coord{ x: 11, y: 11}, Tile::Black).ok().expect("blah");

    print!("{}", board)
}
