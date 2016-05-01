extern crate regex;
extern crate time;

use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::collections::HashSet;
use time::PreciseTime;

#[derive(Copy, Clone, Debug)]
enum Tile {
    Island(i32, i32),
    Water,
    Bridge,
}

#[derive(Clone)]
struct Map {
    data: Vec<Tile>,
    bridges: Vec<(i32, i32, i32)>,
    height: i32,
    i_counter: i32,
    width: i32,
}

impl Map {
    fn new(width: i32, height: i32) -> Map {
        Map{width: width, height: height, data: vec![Tile::Water; (width * height) as usize],
        bridges: Vec::new(), i_counter: 0}
    }

    fn get(&self, y: i32, x: i32) -> Tile {
        self.data[(y * self.width + x) as usize]
    }

    fn set(&mut self, y: i32, x: i32, tile: Tile) {
        self.data[(y * self.width + x) as usize] = tile;
    }

    fn add_island(&mut self, y: i32, x: i32, n: i32) {
        let i = self.i_counter;
        self.set(y, x, Tile::Island(i, n));
        self.i_counter += 1;
    }

    fn iter_islands(&self) -> IterIslands {
        IterIslands(&self, 0)
    }

    fn build_bridge(&mut self, y: i32, x: i32, ny: i32, nx: i32, weight: i32) {
        for iy in (y+1)..ny {
            self.set(iy, x, Tile::Bridge);
        }
        for ix in (x+1)..nx {
            self.set(y, ix, Tile::Bridge);
        }
        if let Tile::Island(id, w) = self.get(y, x) {
            self.set(y, x, Tile::Island(id, w - weight));
        }
        if let Tile::Island(id, w) = self.get(ny, nx) {
            self.set(ny, nx, Tile::Island(id, w - weight));
        }
    }
}

struct IterIslands<'a>(&'a Map, i32);

impl<'a> Iterator for IterIslands<'a> {
    type Item = ((i32, i32), (i32, i32));

    fn next(&mut self) -> Option<Self::Item> {
        while self.1 < (self.0.width * self.0.height) {
            let y = self.1 / self.0.width;
            let x = self.1 % self.0.width;
            self.1 += 1;
            if let Tile::Island(id, w) = self.0.get(y, x) {
                return Some(((y, x), (id, w)));
            }
        }
        None
    }
}

fn parse_maps(path: &str) -> Result<Vec<Map>, io::Error> {
    let size_re = Regex::new(r"(\d+)x(\d+)").unwrap();
    let island_re = Regex::new(r"island\((\d+), *(\d+), *(\d+)\)\.").unwrap();
    let f = try!(File::open(path));
    let reader = BufReader::new(f);
    let mut current_map: Option<Map> = None;
    let mut maps: Vec<Map> = Vec::new();

    for line in reader.lines() {
        let line = try!(line);
        if let Some(cap) = size_re.captures(line.as_str()) {
            if let Some(map) = current_map {
                maps.push(map);
            }
            let width = cap.at(1).unwrap().parse::<i32>().unwrap();
            let height = cap.at(1).unwrap().parse::<i32>().unwrap();
            current_map = Some(Map::new(width, height));
            continue;
        }
        if let Some(ref mut map) = current_map {
            for cap in island_re.captures_iter(line.as_str()) {
                let x = cap.at(1).unwrap().parse::<i32>().unwrap();
                let y = map.height - 1 - cap.at(2).unwrap().parse::<i32>().unwrap();
                let n = cap.at(3).unwrap().parse::<i32>().unwrap();
                map.add_island(y, x, n);
            }
        }
    }
    if let Some(map) = current_map {
        maps.push(map);
    }
    Ok(maps)
}

fn find_bridges(m: &Map, y: i32, x: i32) -> Vec<(i32, i32, i32, i32)> {
    let mut cx;
    let mut cy;
    let mut bridges = Vec::new();

    let (_, w) = match m.get(y, x) {
        Tile::Island(id, w) => (id, w),
        _                   => return bridges,
    };

    for &(dy, dx) in &[(1, 0), (0, 1)] {
        cy = y + dy;
        cx = x + dx;
        while cy >= 0 && cy < m.height && cx >= 0 && cx < m.width {
            match m.get(cy, cx) {
                Tile::Water             => { cy += dy; cx += dx },
                Tile::Bridge            => { break },
                Tile::Island(n_id, n_w) => {
                    if w >= 1 && n_w >= 1 {
                        bridges.push((cy, cx, 1, n_id));
                    }
                    if w >= 2 && n_w >= 2 {
                        bridges.push((cy, cx, 2, n_id));
                    }
                    break;
                }
            };
        }
    }
    bridges
}

fn solved(map: &Map) -> bool {
    for (_, (_, w)) in map.iter_islands() {
        if w != 0 {
            return false;
        }
    }
    true
}

fn subsets<T>(v: &[T]) -> Vec<Vec<T>>
    where T: Copy + Clone
{
    let mut res = Vec::new();

    if v.len() == 0 {
        res.push(Vec::new());
        return res;
    }

    for sub in subsets(&v[1..]) {
        let mut new = sub.clone();
        res.push(sub);
        new.push(v[0]);
        res.push(new);
    }
    res
}

fn find_solution(map: &Map) -> Option<Vec<(i32, i32, i32)>> {
    if solved(map) {
        return Some(map.bridges.clone())
    }
    for ((y, x), (id, w)) in map.iter_islands() {
        let bridges = find_bridges(&map, y, x);
        if bridges.is_empty() {
            continue;
        }
        let mut found_valid = false;
        for bridge_set in subsets(bridges.as_slice()) {
            let mut island_set = HashSet::new();
            let mut n_map = map.clone();
            let mut sum = 0;
            for &(by, bx, bw, bid) in &bridge_set {
                if island_set.contains(&(by, bx)) {
                    break;
                } else {
                    island_set.insert((by, bx));
                }
                n_map.build_bridge(y, x, by, bx, bw);
                n_map.bridges.push((id, bid, bw));
                sum += bw;
            }
            if sum == w {
                found_valid = true;
                if let Some(bridges) = find_solution(&n_map) {
                    return Some(bridges);
                }
            }
        }
        if !found_valid {
            break
        }
    }
    None
}

fn main() {
    let path = env::args().nth(1).expect("Missing arg");
    let maps = parse_maps(path.as_str()).expect("File not readable");

    for (n, map) in maps.iter().enumerate() {
        println!("Solving map {} ({}x{}):", n+1, map.width, map.height);
        let start_time = PreciseTime::now();
        if let Some(bridges) = find_solution(map){
            println!("Total bridges {}:", bridges.len());
            for (id1, id2, w) in bridges {
                println!("> bridge from {} to {}, width {}", id1, id2, w);
            }
            println!("Elapsed time {}\n", start_time.to(PreciseTime::now()));
        } else {
            println!("No solution found");
        }
    }
}
