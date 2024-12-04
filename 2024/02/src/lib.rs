fn read_levels(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_safe(level: &[i32], tolerance: i32) -> bool {
    if tolerance == 0 {
        return check(level).is_none();
    }
    for i in 0..level.len() {
        let mut variation = level.to_vec();
        variation.remove(i);
        if check(&variation).is_none() {
            return true;
        }
    }
    false
}

fn check(level: &[i32]) -> Option<usize> {
    let increasing = level[0] < level[1];
    for i in 1..level.len() {
        if level[i - 1] == level[i] {
            return Some(i);
        }
        // abs difference must be at most 3
        if (level[i - 1] - level[i]).abs() > 3 {
            return Some(i);
        }
        // must be increasing or decreasing across the level
        if (level[i - 1] < level[i]) != increasing {
            return Some(i);
        }
    }
    None
}

#[test]
fn part1_example() {
    let levels = read_levels(include_str!("example.txt"));
    assert_eq!(
        levels.into_iter().filter(|level| is_safe(level, 0)).count(),
        2
    );
}

#[test]
fn part1_input() {
    let levels = read_levels(include_str!("input.txt"));
    assert_eq!(
        levels.into_iter().filter(|level| is_safe(level, 0)).count(),
        472
    );
}

#[test]
fn part2_example() {
    let levels = read_levels(include_str!("example.txt"));
    assert_eq!(
        levels.into_iter().filter(|level| is_safe(level, 1)).count(),
        4
    );
}

#[test]
fn part2_input() {
    let levels = read_levels(include_str!("input.txt"));
    assert_eq!(
        levels.into_iter().filter(|level| is_safe(level, 1)).count(),
        520
    );
}
