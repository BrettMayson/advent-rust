use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut file = File::open("./02/input.txt").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let data = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split('x');
            let l = parts.next().unwrap().parse::<u32>().unwrap();
            let w = parts.next().unwrap().parse::<u32>().unwrap();
            let h = parts.next().unwrap().parse::<u32>().unwrap();
            let present = Present::new(l, w, h);
            let paper = present.surface_area() + present.smallest_side();
            let ribbon = present.ribbon();
            (paper, ribbon)
        })
        .fold((0, 0), |(paper, ribbon), (p, r)| (paper + p, ribbon + r));
    println!("Part One: {}", data.0);
    println!("Part Two: {}", data.1);
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(length: u32, width: u32, height: u32) -> Present {
        Present {
            length,
            width,
            height,
        }
    }

    fn surface_area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn smallest_side(&self) -> u32 {
        let mut sides = vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        sides.sort();
        sides[0]
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn ribbon(&self) -> u32 {
        let mut sides = vec![self.length, self.width, self.height];
        sides.sort();
        2 * sides[0] + 2 * sides[1] + self.volume()
    }
}
