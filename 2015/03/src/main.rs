use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let data = {
        let mut file = File::open("./03/input.txt").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let mut x = 0;
    let mut y = 0;
    let mut houses = HashMap::new();
    data.chars().for_each(|c| {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        let key = (x, y);
        let count = houses.entry(key).or_insert(0);
        *count += 1;
    });
    println!(
        "Part One: {}",
        houses.values().into_iter().filter(|&&c| c > 0).count()
    );
    x = 0;
    y = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut houses = HashMap::new();
    data.chars().collect::<Vec<_>>().chunks(2).for_each(|c| {
        // Santa
        match c[0] {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        let key = (x, y);
        let count = houses.entry(key).or_insert(0);
        *count += 1;

        if c.len() == 1 {
            return;
        }
        // Robo-Santa
        match c[1] {
            '^' => y2 += 1,
            'v' => y2 -= 1,
            '>' => x2 += 1,
            '<' => x2 -= 1,
            _ => (),
        }
        let key = (x2, y2);
        let count = houses.entry(key).or_insert(0);
        *count += 1;
    });
    println!(
        "Part Two: {}",
        houses.values().into_iter().filter(|&&c| c > 0).count()
    );
}
