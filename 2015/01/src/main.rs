use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut file = File::open("./01/input.txt").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let mut floor = 0;
    let mut pos = 0;
    let mut p2 = false;
    for char in data.chars() {
        pos += 1;
        if char == '(' {
            floor += 1;
        } else if char == ')' {
            floor -= 1;
        }
        if !p2 && floor == -1 {
            println!("Part Two: {}", pos);
            p2 = true;
        }
    }
    println!("Part One: {}", floor);
}
