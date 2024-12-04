type Grid = Vec<Vec<char>>;

fn read_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_word_in_grid(grid: &Grid, word: &str) -> usize {
    let mut count = 0;

    let word_len = word.len();
    let mut directions = Vec::new();
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            directions.push((dr, dc));
        }
    }

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            for &(dr, dc) in &directions {
                if check_word(grid, word, r, c, dr, dc, word_len) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_word(
    grid: &Grid,
    word: &str,
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
    word_len: usize,
) -> bool {
    let mut row = r as isize;
    let mut col = c as isize;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..word_len {
        if row < 0 || row >= rows || col < 0 || col >= cols {
            return false;
        }
        if grid[row as usize][col as usize] != word.chars().nth(i).unwrap() {
            return false;
        }
        row += dr;
        col += dc;
    }

    true
}

#[test]
fn part1_example() {
    let grid = read_grid(include_str!("example.txt"));
    assert_eq!(find_word_in_grid(&grid, "XMAS"), 18);
}

#[test]
fn part1_input() {
    let grid = read_grid(include_str!("input.txt"));
    assert_eq!(find_word_in_grid(&grid, "XMAS"), 2644);
}

fn find_crosses(grid: &Grid) -> Vec<(char, char, char, char)> {
    let mut crosses = Vec::new();
    for r in 1..grid.len() - 1 {
        for c in 1..grid[0].len() - 1 {
            if grid[r][c] == 'A' {
                crosses.push((
                    grid[r - 1][c - 1],
                    grid[r - 1][c + 1],
                    grid[r + 1][c - 1],
                    grid[r + 1][c + 1],
                ));
            }
        }
    }
    crosses
}

fn is_valid_cross(cross: &(char, char, char, char)) -> bool {
    // the list must contain 2 M and 2 S
    let vec = [cross.0, cross.1, cross.2, cross.3];
    let m = vec.iter().filter(|&&x| x == 'M').count();
    let s = vec.iter().filter(|&&x| x == 'S').count();
    if m != 2 || s != 2 {
        return false;
    }
    // check for MAM or SAS
    if cross.0 == cross.3 || cross.1 == cross.2 {
        return false;
    }
    true
}

fn count_crosses(grid: &Grid) -> usize {
    find_crosses(grid)
        .into_iter()
        .filter(is_valid_cross)
        .count()
}

#[test]
fn part2_example() {
    let grid = read_grid(include_str!("example.txt"));
    assert_eq!(count_crosses(&grid), 9);
}

#[test]
fn part2_input() {
    let grid = read_grid(include_str!("input.txt"));
    assert_eq!(count_crosses(&grid), 1952);
}
