use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut data = String::new();
        let mut file = File::open("./04/input.txt").unwrap();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let p1: u32 = data.lines().filter(|l| !l.is_empty()).map(|l| {
        l.split_once(',').map(|(a, b)| {
            let (astart, aend) = a.split_once('-').map(|(a, b)| {
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                (a, b)
            }).unwrap();
            let (bstart, bend) = b.split_once('-').map(|(a, b)| {
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                (a, b)
            }).unwrap();
            ((astart <= bstart && bend <= aend) || (bstart <= astart && aend <= bend)) as u32
        }).unwrap()
    }).sum();
    println!("Part One {}", p1);
    let p2: u32 = data.lines().filter(|l| !l.is_empty()).map(|l| {
        l.split_once(',').map(|(a, b)| {
            let (astart, aend) = a.split_once('-').map(|(a, b)| {
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                (a, b)
            }).unwrap();
            let (bstart, bend) = b.split_once('-').map(|(a, b)| {
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                (a, b)
            }).unwrap();
            ((astart <= bstart && aend >= bstart) || (bstart <= astart && bend >= astart)) as u32
        }).unwrap()
    }).sum();
    println!("Part Two {}", p2);
}
