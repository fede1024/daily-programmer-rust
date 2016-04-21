use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    FormatError(String),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::IoError(err)
    }
}

impl From<String> for ParseError {
    fn from(err: String) -> ParseError {
        ParseError::FormatError(err)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Food,
    Empty,
    Pit,
    Snake,
}

#[derive(Clone)]
pub struct Map {
    data: Vec<Tile>,
    pub head: (i32, i32),
    pub width: i32,
    pub height: i32,
    pub food: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Map { data: vec![], width: width, height: height, head: (0, 0), food: 0 }
    }

    pub fn get(&self, y: i32, x: i32) -> Option<Tile> {
        if y >= 0 && y < self.height && x >= 0 && x < self.width {
            Some(self.data[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, y: i32, x: i32, tile: Tile) {
        self.data[(y * self.width + x) as usize] = tile;
    }

    pub fn print(&self) {
        self.print_with_path(vec![]);
    }

    pub fn print_with_path(&self, path: Vec<(char, i32, i32)>) {
        let path_set: HashMap<(i32, i32), char> = path.iter().map(|&(c, y, x)| ((y, x), c)).collect();
        let print_border = || {
            print!("+");
            for _ in 0..self.width { print!("-"); }
            println!("+");
        };
        print_border();
        for y in 0..self.height {
            print!("|");
            for x in 0..self.width {
                let c = match self.get(y, x).unwrap() {
                    Tile::Empty => ' ',
                    Tile::Snake => 's',
                    Tile::Food => '*',
                    Tile::Pit => 'O',
                };
                let x = path_set.get(&(y, x)).unwrap_or(&c);
                print!("{}", x);
            }
            println!("|");
        }
        print_border();
    }
}

fn parse_maps(lines: Vec<String>) -> Result<Vec<Map>, ParseError> {
    let mut maps = Vec::new();
    let mut map = Map::new(0, 0);

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() < 1 {
            continue
        }
        if chars[0] == '+' {
            if line.as_str().contains("map") {
                map = Map::new((chars.len() - 2) as i32, 0);
            } else {
                let h = map.data.iter().position(|&t| t == Tile::Snake).unwrap_or(0) as i32;
                map.head = (h / map.width, h % map.width);
                map.food = map.data.iter().filter(|&t| *t == Tile::Food).count() as i32;
                maps.push(map.clone());
            }
            continue;
        }
        for c in chars {
            match c {
                '|' => continue,
                ' ' => map.data.push(Tile::Empty),
                's' => map.data.push(Tile::Snake),
                '*' => map.data.push(Tile::Food),
                'O' => map.data.push(Tile::Pit),
                x   => return Err(ParseError::FormatError(format!("Unknown char {}", x))),
            };
        }
        map.height += 1;
    }

    Ok(maps)
}

fn get_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let f = try!(File::open(path));
    let reader = BufReader::new(f);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(try!(line));
    }
    Ok(lines)
}

pub fn parse_input_file(path: &str) -> Result<Vec<Map>, ParseError> {
    let lines = try!(get_lines(path));
    parse_maps(lines)
}
