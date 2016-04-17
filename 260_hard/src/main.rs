use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn get_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let f = try!(File::open(path));
    let reader = BufReader::new(f);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(try!(line));
    }
    Ok(lines)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Food,
    Empty,
    Pit,
    Snake,
}

#[derive(Clone)]
struct Map {
    data: Vec<Tile>,
    start: (i32, i32),
    width: i32,
    height: i32,
    food: i32,
}

impl Map {
    fn new(width: i32, height: i32) -> Map {
        Map { data: vec![], width: width, height: height, start: (0, 0), food: 0 }
    }

    fn get(&self, y: i32, x: i32) -> Option<Tile> {
        if y >= 0 && y < self.height && x >= 0 && x < self.width {
            Some(self.data[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    fn set(&mut self, y: i32, x: i32, tile: Tile) {
        self.data[(y * self.width + x) as usize] = tile;
    }

    fn print(&self) {
        print!("+");
        for _ in 0..self.width { print!("-"); }
        println!("+");
        for y in 0..self.height {
            print!("|");
            for x in 0..self.width {
                let c = match self.get(y, x).unwrap() {
                    Tile::Empty => ' ',
                    Tile::Snake => 's',
                    Tile::Food => '*',
                    Tile::Pit => 'O',
                };
                print!("{}", c);
            }
            println!("|");
        }
        print!("+");
        for _ in 0..self.width { print!("-"); }
        println!("+");
    }
}

fn parse_maps(lines: Vec<String>) -> Result<Vec<Map>, String> {
    let mut maps = Vec::new();
    let mut map = Map::new(0, 0);

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars[0] == '+' {
            if line.as_str().contains("map") {
                map = Map::new((chars.len() - 2) as i32, 0);
            } else {
                let n = map.data.iter().position(|&t| t == Tile::Snake).unwrap_or(0) as i32;
                map.start = (n / map.width, n % map.width);
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
                x   => return Err(format!("Unknown char {}", x)),
            };
        }
        map.height += 1;
    }

    Ok(maps)
}

fn main() {
    let path = env::args().nth(1).expect("Missing arg");
    println!("{:?}", path);
    let lines = get_lines(path.as_str()).expect("File not readable");
    let maps = parse_maps(lines);

    for map in maps.unwrap() {
        println!("{:?}", map.start);
        map.print();
    }
}
