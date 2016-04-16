fn print_border(size: usize, l: char, m: char, r: char) {
    print!("{}", l);
    for _ in 0..(size-1) { print!("────{}", m); }
    println!("────{}", r);
}

pub fn print_square(size: usize, data: &[i32]) {
    print_border(size, '┌', '┬', '┐');
    for y in 0..size {
        for x in 0..size {
            let c = data[y*size + x];
            if c == 0 {
                print!("│    ");
            } else {
                print!("│{:3} ", c);
            }
        }
        print!("│\n");
        if y < size - 1 {
            print_border(size, '├', '┼', '┤');
        } else {
            print_border(size, '└', '┴', '┘');
        }
    }
}
