use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut data = String::new();
        let mut file = File::open("./01/input.txt").unwrap();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let mut max = data
        .split("\n\n")
        .map(|e| {
            e.split('\n')
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();
    println!("Part One: {}", max.iter().max().unwrap());
    max.sort();
    max.reverse();
    println!("Part Two: {}", max[0] + max[1] + max[2]);
}
