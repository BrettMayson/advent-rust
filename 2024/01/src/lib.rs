fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(" ");
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.last().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    assert_eq!(left.len(), right.len());
    (left, right)
}

fn difference(left: &[i32], right: &[i32]) -> i32 {
    let mut diff = 0;
    for i in 0..left.len() {
        diff += (left[i] - right[i]).abs();
    }
    diff
}

fn occurences(needle: i32, haystack: &[i32]) -> usize {
    haystack.iter().filter(|&&x| x == needle).count()
}

fn similarity(left: &[i32], right: &[i32]) -> usize {
    left.iter()
        .map(|&x| x as usize * occurences(x, right))
        .sum()
}

#[test]
fn part1_example() {
    let (left, right) = get_lists(include_str!("example.txt"));
    assert_eq!(difference(&left, &right), 11);
}

#[test]
fn part1_input() {
    let (left, right) = get_lists(include_str!("input.txt"));
    assert_eq!(difference(&left, &right), 1830467);
}

#[test]
pub fn part2_example() {
    let (left, right) = get_lists(include_str!("example.txt"));
    assert_eq!(similarity(&left, &right), 31);
}

#[test]
pub fn part2_input() {
    let (left, right) = get_lists(include_str!("input.txt"));
    assert_eq!(similarity(&left, &right), 26674158);
}
