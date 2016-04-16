extern crate itertools;
extern crate time;

mod print;

use itertools::Itertools;
use std::cell::Cell;
use time::PreciseTime;

#[derive(Copy, Clone)]
struct Coord(usize, usize);

fn create_tile_positions(size: i32) -> Vec<(Coord, Coord)> {
    let p1 = (0..size).cartesian_product((0..size));
    let p2 = (0..size).cartesian_product((0..size));
    p1.cartesian_product(p2)
        .filter(|&((y1, x1), (y2, x2))| { (y1 - y2).abs() + (x1 - x2).abs() == 1 })
        .map(|((y1, x1), (y2, x2))| { (Coord(y1 as usize, x1 as usize), Coord(y2 as usize, x2 as usize)) })
        .collect()
}

struct Square {
    data: Vec<i32>,
    size: usize,
    sum: i32,
    used: Vec<bool>,
}

enum Eval {
    Done,
    Doable,
    Wrong
}

impl Square {
    fn new (size: usize) -> Square {
        let sum = size * (size * size + 1) / 2;
        Square{
            data: vec![0; size * size],
            size: size,
            sum: sum as i32,
            used: vec![false; sum],
        }
    }

    fn get(&self, c: Coord) -> i32 {
        self.data[c.0 * self.size + c.1]
    }

    fn set(&mut self, c: Coord, val: i32) {
        self.data[c.0 * self.size + c.1] = val;
    }

    // Return the number of empty cells in a valid square,
    // or -1 if the square is not valid.
    fn eval(&self) -> Eval {
        let mut tot_zero = 0;
        let mut d = (0, 0);
        let add_tile = |y: usize, x: usize, stat: &mut (i32, i32)| {
            stat.0 += self.get(Coord(y, x));
            stat.1 += if self.get(Coord(y, x)) == 0 { 1 } else { 0 };
        };
        let cant_fill = |(tot, _): (i32, i32)| -> bool {
            self.used[(self.sum - tot) as usize]
        };
        let wrong_values = |(tot, zeroes): (i32, i32)| {
            tot > self.sum || (tot != self.sum && zeroes == 0)
        };
        for y in 0..self.size {
            let mut r = (0, 0);
            let mut c = (0, 0);
            for x in 0..self.size {
                add_tile(y, x, &mut r);
                add_tile(x, y, &mut c);
                if y == x {
                    add_tile(y, y, &mut d);
                }
            }
            if wrong_values(r) || wrong_values(c) ||
                (r.1 == 1 && cant_fill(r)) || (c.1 == 1 && cant_fill(c)) {
                return Eval::Wrong;
            }
            tot_zero += r.1;
        }
        if  wrong_values(d) || (d.1 == 1 && cant_fill(d)) {
            return Eval::Wrong;
        }
        if tot_zero == 0 {
            Eval::Done
        } else {
            Eval::Doable
        }
    }

    fn print(&self) {
        print::print_square(self.size, &self.data);
    }
}

fn find_magic(positions: &[(Coord, Coord)], tiles: &[Tile], square: &mut Square) -> bool {
    let set = |s: &mut Square, c1: Coord, c2: Coord, tile: &Tile| {
        s.set(c1, tile.a);
        s.set(c2, tile.b);
        s.used[tile.a as usize] = true;
        s.used[tile.b as usize] = true;
    };
    let unset = |s: &mut Square, c1: Coord, c2: Coord, tile: &Tile| {
        s.set(c1, 0);
        s.set(c2, 0);
        s.used[tile.a as usize] = false;
        s.used[tile.b as usize] = false;
    };
    for &(c1, c2) in positions {
        if square.get(c1) != 0 || square.get(c2) != 0 {
            continue
        }
        for tile in tiles {
            if tile.used.get() { continue }
            tile.used.set(true);
            set(square, c1, c2, tile);
            match square.eval() {
                Eval::Done   => return true,
                Eval::Doable => if find_magic(positions, tiles, square) {
                                    return true;
                                },
                Eval::Wrong  => {},
            };
            unset(square, c1, c2, tile);
            tile.used.set(false);
        }
    }
    false
}

struct Tile {
    a: i32,
    b: i32,
    used: Cell<bool>,
}

fn create_tiles(input: &[(i32, i32)]) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = input.iter()
        .map(|&(a, b)| Tile { a:a, b:b, used: Cell::new(false) }).collect();
    tiles.sort_by(|i1: &Tile, i2: &Tile| (i1.a + i1.b).cmp(&(i2.a + i2.b)));
    tiles.reverse();
    tiles
}

fn main() {
    let inputs = vec![
        (4, vec![(1, 7), (2, 8), (3, 13), (4, 6), (5, 11), (9, 15), (10, 16), (12, 14)]),
        (4, vec![(1, 8), (2, 13), (3, 14), (4, 7), (5, 11), (6, 12), (9, 16), (10, 15)]),
        (4, vec![(1, 9), (2, 10), (3, 6), (4, 14), (5, 11), (7, 15), (8, 16), (12, 13)]),
        (4, vec![(1, 14), (2, 8), (3, 13), (4, 15), (5, 11), (6, 9), (7, 12), (10, 16)]),
        // (6, vec![(1, 33), (2, 20), (3, 32), (4, 22), (5, 14), (6, 35), (7, 25), (8, 19),
        //          (9, 18), (10, 26), (11, 31), (12, 16), (13, 29), (15, 17), (21, 34),
        //          (23, 30), (24, 28), (27, 36)])
    ];

    for (size, input) in inputs {
        println!("Solving {}x{}...", size, size);
        let start_time = PreciseTime::now();
        let positions = create_tile_positions(size as i32);
        let mut square = Square::new(size);
        if find_magic(&positions, create_tiles(&input).as_slice(), &mut square) {
            square.print();
        } else {
            println!("Square not found :(")
        }
        println!("Elapsed time {}\n", start_time.to(PreciseTime::now()));
    }
}
