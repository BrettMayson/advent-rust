use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Blue,
    Red,
    Green,
}

impl Color {
    fn from_str(s: &str) -> Self {
        match s {
            "blue" => Self::Blue,
            "red" => Self::Red,
            "green" => Self::Green,
            _ => panic!("Invalid color: {}", s),
        }
    }
}

struct Bag(HashMap<Color, u32>);

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn from_str(s: &str) -> Self {
        let (game, hands) = s.split_once(": ").unwrap();
        let id = game.split_once(' ').unwrap().1.parse().unwrap();
        let hands = hands.split("; ").map(|hands| {
            let mut cubes = hands.split(", ");
            let mut map = HashMap::new();
            while let Some((count, color)) = cubes.next().map(|s| s.split_once(' ').unwrap()) {
                map.insert(Color::from_str(color), count.parse().unwrap());
            }
            Hand(map)
        }).collect();
        Self { id, hands }
    }

    fn possible_with_bag(&self, bag: &Bag) -> bool {
        self.hands.iter().all(|hand| {
            hand.0.iter().all(|(color, count)| {
                bag.0.get(color).unwrap_or(&0) >= count
            })
        })
    }

    /// returns the highest count of each color from all hands
    fn min_cube(&self) -> HashMap<Color, u32> {
        let mut min = HashMap::new();
        for hand in &self.hands {
            for (color, count) in hand.0.clone() {
                let entry = min.entry(color).or_insert(count);
                if *entry < count {
                    *entry = count;
                }
            }
        }
        min
    }
}

#[derive(Clone, Debug)]
struct Hand(HashMap<Color, u32>);

fn part_1(data: &str) -> u32 {
    let bag = Bag(vec![(Color::Blue, 14), (Color::Red, 12), (Color::Green, 13)].into_iter().collect());
    data.lines()
        .map(|line| {
            Game::from_str(line)
        })
        .filter_map(|game| game.possible_with_bag(&bag).then_some(game.id))
        .sum::<u32>()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 8);
    assert_eq!(part_1(include_str!("input.txt")), 2486);
}

fn part_2(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            Game::from_str(line).min_cube().values().product::<u32>()
        })
        .sum::<u32>()
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286);
    assert_eq!(part_2(include_str!("input.txt")), 87984);
}
