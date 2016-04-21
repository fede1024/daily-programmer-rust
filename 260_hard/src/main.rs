use std::env;

mod map;

use map::Map;
use map::Tile;
use map::parse_input_file;


fn find_path(map: &mut Map, depth: i32) -> Option<Vec<(char, i32, i32)>> {
    if depth == 0 {
        return None
    }
    for &(m, dy, dx) in &[('↑', -1, 0), ('↓', 1, 0), ('←', 0, -1), ('→', 0, 1)] {
        let head = map.head;
        let y = map.head.0 + dy;
        let x = map.head.1 + dx;
        if let Some(t) = map.get(y, x) {
            if t == Tile::Pit || t == Tile::Snake {
                continue;
            }
            if t == Tile::Food {
                map.food -= 1;
                if map.food == 0 {
                    return Some(vec![(m, y, x)]);
                }
            }
            map.head.0 = y;
            map.head.1 = x;
            map.set(y, x, Tile::Snake);
            if let Some(mut res) = find_path(map, depth - 1) {
                res.push((m, y, x));
                return Some(res);
            }
            map.set(y, x, t);
            if t == Tile::Food {
                map.food += 1;
            }
            map.head = head;
        }
    }
    None
}

fn main() {
    let path = env::args().nth(1).expect("Missing arg");
    let maps = parse_input_file(path.as_str()).expect("File not readable");

    for (n, m) in maps.iter().enumerate() {
        let map = m.clone();
        println!("\nMap: {}", n);
        map.print();
        println!("{}x{}: {} (h: {:?})", map.height, map.width, map.food, map.head);

        for d in 0..(map.height * map.width) {
            let mut map = map.clone();
            let res = find_path(&mut map, d);
            if let Some(mut path) = res {
                path.reverse();
                map.print_with_path(path);
                break;
            }
        }
    }
}
