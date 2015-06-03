use std::fmt;
use std::result::Result::{self, Ok, Err};

#[derive(Clone)]
#[derive(Copy)]
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
        );
        Ok(())
    }
}

fn main(){
    let board = &[[Tile::Empty; 19]; 19];
    // let board: Vec<Vec<Tile>> = vec![Vec::with_capacity(19), 19];
    for line in board {
        for tile in line {
            print!("{} ", tile);
        }
        print!("\n",);
    }
}
