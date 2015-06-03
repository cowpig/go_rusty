use std::fmt;
use std::result::Result::{self, Ok, Err};
use std::ops::Index;

#[derive(Clone)]
#[derive(Copy)]
enum Tile {
    White,
    Black,
    Empty
}

type Board = [[Tile; 19]; 19];
type Coord = (usize, usize);

// impl Index<Coord> for Board {
//     fn index(&self, index:Coord) -> Tile {
//         self[index.0][index.1]
//     }
// }

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
        if (coord.0 + offset > 0) && (coord.0 + offset < 18) {
            output.push((coord.0 + offset, coord.1));
        }
        if (coord.1 + offset > 0) && (coord.1 + offset < 18) {
            output.push((coord.0, coord.1 + offset));
        }
    }
    output
}

fn main(){
    let board:&Board = &[[Tile::Empty; 19]; 19];
    // let board: Vec<Vec<Tile>> = vec![Vec::with_capacity(19), 19];
    for line in board {
        for tile in line {
            print!("{} ", tile);
        }
        print!("\n",);
    }
}
