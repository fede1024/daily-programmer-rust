use std::collections::HashMap;
use std::iter::FromIterator;

struct Pos {
    x: i32,
    y: i32,
}

fn create_map() -> HashMap<char, Pos> {
    let iter = "123456789.0".chars().enumerate()
        .map(|(n, c)| { (c, Pos{ x: (n as i32) % 3, y: (n as i32) / 3 } ) });

    HashMap::<char, Pos>::from_iter(iter)
}

fn dist(p1: &Pos, p2: &Pos) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    ((dx*dx + dy*dy) as f32).sqrt()
}

fn main() {
    let map = create_map();
    let seq = "219.45.143.143";

    let tot = seq.chars().zip(seq.chars().skip(1))
        .fold(0f32, |sum, (p, c)| { sum + dist(map.get(&p).unwrap(), map.get(&c).unwrap()) });

    println!("Tot: {}", tot);
}
