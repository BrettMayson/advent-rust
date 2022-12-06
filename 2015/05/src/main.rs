use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut file = File::open("./05/input.txt").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let nice_words = data
        .lines()
        .filter(|word| !word.is_empty())
        .filter(|word| nice_1(word))
        .count();
    println!("Part One: {}", nice_words);
    let nice_words = data
        .lines()
        .filter(|word| !word.is_empty())
        .filter(|word| nice_2(word))
        .count();
    println!("Part Two: {}", nice_words);
}

fn nice_1(word: &str) -> bool {
    for illegal in ["ab", "cd", "pq", "xy"] {
        if word.contains(illegal) {
            return false;
        }
    }
    let vowels = word.chars().filter(|&c| "aeiou".contains(c)).count();
    if vowels < 3 {
        return false;
    }
    word.chars()
        .into_iter()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|w| w[0] == w[1])
}

fn nice_2(word: &str) -> bool {
    let chars = word.chars().into_iter().collect::<Vec<_>>();
    let mut found = false;
    'search: for i in 0..chars.len() - 1 {
        let pair = &chars[i..i + 2];
        if chars[i + 2..].windows(2).any(|w| w == pair) {
            found = true;
            break 'search;
        }
    }
    if !found {
        return false;
    }

    word.chars()
        .into_iter()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|w| w[0] == w[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_1() {
        assert!(nice_1("ugknbfddgicrmopn"));
        assert!(nice_1("aaa"));
        assert!(!nice_1("jchzalrnumimnmhp"));
        assert!(!nice_1("haegwjzuvuyypxyu"));
        assert!(!nice_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_nice_2() {
        assert!(nice_2("qjhvhtzxzqqjkmpb"));
        assert!(nice_2("xxyxx"));
        assert!(!nice_2("uurcxstgmygtbstg"));
        assert!(!nice_2("ieodomkazucvgmuy"));
    }
}
