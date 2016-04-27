extern crate itertools;
extern crate time;
mod lib;

use itertools::Itertools;
use lib::astar::Graph;
use lib::astar::a_star;
use std::rc::Rc;
use time::PreciseTime;

#[derive(Copy, Clone, Debug)]
struct Coord(usize, usize);

#[derive(Clone)]
struct World {
    data: Vec<i32>,
    size: usize,
    pairs: Vec<(Coord, Coord)>,
}

fn create_pairs(size: usize) -> Vec<(Coord, Coord)> {
    let p1 = (0..size as i32).cartesian_product((0..size as i32));
    let p2 = (0..size as i32).cartesian_product((0..size as i32));
    p1.cartesian_product(p2)
        .filter(|&((y1, x1), (y2, x2))| { ((y1 - y2).abs() + (x1 - x2).abs() == 1) && y2 >= y1 && x2 >= x1 })
        .map(|((y1, x1), (y2, x2))| { (Coord(y1 as usize, x1 as usize), Coord(y2 as usize, x2 as usize)) })
        .collect()
}

impl World {
    fn new(size: usize, values: &[i32]) -> World {
        let mut data = Vec::new();
        data.extend_from_slice(values);
        World {size: size, data: data, pairs: create_pairs(size)}
    }

    fn get(&self, coord: Coord) -> i32 {
        self.data[coord.0 * self.size + coord.1]
    }

    fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{:3}", self.get(Coord(y, x)));
            }
            println!("");
        }
    }
}

type WNode = Rc<Vec<i32>>;

impl Graph for World {
    type Node = WNode;
    type Move = (Coord, Coord);

    fn is_goal(&self, node: &Self::Node) -> bool {
        for (n, &x) in node.iter().enumerate() {
            if ((n + 1) as i32) != x {
                return false;
            }
        }
        true
    }

    fn neighbors(&self, node: &Self::Node) -> Vec<(Self::Move, Self::Node)> {
        let mut res = Vec::new();
        for &(c1, c2) in &self.pairs {
            let mut new = node.as_ref().clone();
            new[c1.0 * self.size + c1.1] = node[c2.0 * self.size + c2.1];
            new[c2.0 * self.size + c2.1] = node[c1.0 * self.size + c1.1];
            res.push(((c1, c2), Rc::new(new)));
        }
        res
    }
}

fn h1(w: &World, n: &WNode) -> i32 {
    let mut tot = 0;
    for (n, &v) in n.iter().enumerate() {
        let size = w.size as i32;
        let pos = n as i32 + 1;
        tot += (v / size - pos / size).abs() + (v % size - pos % size).abs();
    }
    tot / 2
}

fn main() {
    let w = World::new(4, &[4, 6, 2, 14, 15, 8, 13, 1, 10, 5, 9, 12, 7, 11, 16, 3]);
    //let w = World::new(4, &[11, 14, 15, 6, 4, 7, 1, 5, 9, 2, 8, 3, 16, 12, 13, 10]);
    let start_time = PreciseTime::now();
    let s = Rc::new(w.data.clone());

    if let (n, Some(path)) = a_star(&w, s, h1) {
        for &((c0, c1), ref node) in &path {
            let mut nw = w.clone();
            nw.data = node.as_ref().clone();
            println!("\nSwapping ({}, {}) and ({}, {})", c0.0, c0.1, c1.0, c1.1);
            nw.print();
        }
        println!("{} grids analysed, solution has {} moves", n, path.len());
    } else {
        println!("Nope :(")
    }
    println!("\nElapsed time {}\n", start_time.to(PreciseTime::now()));
}
