use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut data = String::new();
        let mut file = File::open("./03/input.txt").unwrap();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let items = data.lines().filter(|l| !l.is_empty());
    let value = items
        .clone()
        .map(|l| {
            let items = l.chars().map(Item).collect::<Vec<_>>();
            (
                items[0..items.len() / 2].to_vec(),
                items[(items.len() / 2)..].to_vec(),
            )
        })
        .fold(0, |acc, (mut left, right)| {
            left.retain(|i| right.contains(i));
            left.dedup();
            assert_eq!(left.len(), 1);
            acc + left[0].priority()
        });
    println!("Part One: {}", value);
    let value = items
        .clone()
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |acc, chunk| {
            let (a, b, c) = (chunk[0], chunk[1], chunk[2]);
            let mut a = a.chars().collect::<Vec<_>>();
            let b = b.chars().collect::<Vec<_>>();
            let c = c.chars().collect::<Vec<_>>();
            a.retain(|i| b.contains(i) && c.contains(i));
            a.dedup();
            assert_eq!(a.len(), 1);
            acc + Item(a[0]).priority()
        });
    println!("Part Two: {}", value);
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Item(char);

impl Item {
    pub fn priority(&self) -> u32 {
        let value = self.0 as u32;
        if (97..=122).contains(&value) {
            return value - 96;
        }
        if (65..=90).contains(&value) {
            return value - 38;
        }
        panic!("Invalid item");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_priority() {
        assert_eq!(Item('a').priority(), 1);
        assert_eq!(Item('z').priority(), 26);
        assert_eq!(Item('A').priority(), 27);
        assert_eq!(Item('Z').priority(), 52);
    }
}
