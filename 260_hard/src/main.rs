use std::env;

mod map;

use map::Map;
use map::parse_input_file;

fn main() {
    let path = env::args().nth(1).expect("Missing arg");
    println!("{:?}", path);
    let maps = parse_input_file(path.as_str()).expect("File not readable");

    for map in maps {
        println!("{:?}", map.start);
        map.print();
    }
}
