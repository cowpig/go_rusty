use std::fmt;
use std::result::Result::{self, Ok, Err};
use std::ops::Index;
use std::collections::HashSet;


const BOARD_SIZE: usize = 19;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile 
{
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

    fn place(&mut self, coord: Coord, color: Tile) -> Result<(), String> {
        println!("Placing stone at: ({:?})", coord);

        if coord.x >= BOARD_SIZE && coord.y >= BOARD_SIZE {
            return Err(format!("place out-of-bound: {:?}", coord));
        }

        try!(match color {
            Tile::Empty => Err(format!("invalid color: {}", color)),
            _ => Ok(())
        });

        try!(match self[coord.clone()] {
            Tile::Empty => Ok(()),
            _ => Err(format!("place on existing piece: {:?} -> {}", coord, self[coord.clone()]))
        });

        if self.get_captures(coord.clone(), color.clone()).is_some() {
            return Err(format!("Suicidal move!"));
        }

        self.tiles[coord.y][coord.x] = color;

        for neighbor in neighbors(coord.clone()) {
            if let Some(mut captures) = self.get_captures(
                    neighbor, if color==Tile::Black {Tile::White} else {color}) {
                for capture in captures {
                    self.tiles[capture.y][capture.x] = Tile::Empty;
                }
            }
        }

        println!("Resulting board:\n({})", self);
        Ok(())
    }

    fn get_captures(&self, coord:Coord, color:Tile) -> Option<HashSet<Coord>> {
        let mut group:HashSet<Coord> = HashSet::new();
        let mut to_check:Vec<Coord> = Vec::new();
        group.insert(coord.clone());
        for neighbor in neighbors(coord.clone()) {
            to_check.push(neighbor);
        }

        // println!("starting with {:?}, to_check: {:?}", group, to_check);
        
        while let Some(coord_to_check) = to_check.pop() {
            for neighbor in neighbors(coord_to_check.clone()) {
                if self[coord_to_check.clone()] == Tile::Empty {
                    // println!("returning None");
                    return None;
                }
                if !group.contains(&neighbor) && self[coord_to_check.clone()] == color {
                    to_check.push(neighbor);
                    group.insert(coord_to_check.clone());
                }
            }
        }
        // println!("returning {:?}", group);
        Some(group)
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

fn neighbors(coord:Coord) -> Vec<Coord> {
    let mut output:Vec<Coord> = Vec::new();

    if coord.x > 0 { output.push(Coord{ x: coord.x - 1, y: coord.y     }) }
    if coord.y > 0 { output.push(Coord{ x: coord.x,     y: coord.y - 1 }) }

    if coord.x < BOARD_SIZE - 1 { output.push(Coord{ x: coord.x + 1, y: coord.y     }) }
    if coord.y < BOARD_SIZE - 1 { output.push(Coord{ x: coord.x,     y: coord.y + 1 }) }

    output
}

fn main() {
    let mut board = Board::new();

    board.place(Coord{ x: 10, y: 10}, Tile::White).ok().expect("blah");
    board.place(Coord{ x: 11, y: 10}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 9, y: 10}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 10, y: 9}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 10, y: 11}, Tile::Black).ok().expect("blah");
    board.place(Coord{ x: 10, y: 10}, Tile::White).err().expect("shoulda yelled at u");
}
