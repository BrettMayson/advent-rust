type Rules = Vec<(u32, u32)>;
type Update = Vec<u32>;
type Updates = Vec<Update>;

fn read_input(input: &str) -> (Rules, Updates) {
    let Some((rules_src, updates_src)) = input.split_once("\n\n") else {
        panic!("Invalid input");
    };
    let rules = rules_src
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('|').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    let updates = updates_src
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn matches_rules(rules: &Rules, update: &Update) -> bool {
    for (first, second) in rules {
        if !update.contains(first) || !update.contains(second) {
            continue;
        }
        let first_index = update.iter().position(|&n| n == *first).unwrap();
        let second_index = update.iter().position(|&n| n == *second).unwrap();
        if first_index > second_index {
            return false;
        }
    }
    true
}

fn count_middles(updates: &Updates) -> u32 {
    updates
        .iter()
        .fold(0, |acc, update| acc + update[update.len() / 2])
}

#[test]
fn part1_example() {
    let (rules, updates) = read_input(include_str!("example.txt"));
    let updates = updates
        .into_iter()
        .filter(|update| matches_rules(&rules, update))
        .collect();
    assert_eq!(count_middles(&updates), 143);
}

#[test]
fn part1_input() {
    let (rules, updates) = read_input(include_str!("input.txt"));
    let updates = updates
        .into_iter()
        .filter(|update| matches_rules(&rules, update))
        .collect();
    assert_eq!(count_middles(&updates), 4185);
}

fn fix_order(rules: &Rules, update: &mut Update) {
    loop {
        let mut untouched = true;
        for (first, second) in rules {
            if !update.contains(first) || !update.contains(second) {
                continue;
            }
            let first_index = update.iter().position(|&n| n == *first).unwrap();
            let second_index = update.iter().position(|&n| n == *second).unwrap();
            if first_index < second_index {
                continue;
            }
            update.swap(first_index, second_index);
            untouched = false;
        }
        if untouched {
            break;
        }
    }
}

#[test]
fn part2_example() {
    let (rules, updates) = read_input(include_str!("example.txt"));
    let updates = updates
        .into_iter()
        .filter(|update| !matches_rules(&rules, update))
        .map(|mut update| {
            fix_order(&rules, &mut update);
            update
        })
        .collect();
    assert_eq!(count_middles(&updates), 123);
}

#[test]
fn part2_input() {
    let (rules, updates) = read_input(include_str!("input.txt"));
    let updates = updates
        .into_iter()
        .filter(|update| !matches_rules(&rules, update))
        .map(|mut update| {
            fix_order(&rules, &mut update);
            update
        })
        .collect();
    assert_eq!(count_middles(&updates), 4480);
}
