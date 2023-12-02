fn part_1(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|d| d.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| (digits.first().unwrap() * 10) + digits.last().unwrap())
        .sum::<u32>()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
    assert_eq!(part_1(include_str!("input.txt")), 55172);
}

fn part_2(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut numbers: Vec<u32> = Vec::with_capacity(8);
            let mut index = 0;
            loop {
                if index == line.len() {
                    break numbers.first().unwrap() * 10 + numbers.last().unwrap();
                }
                if (bytes[index]).is_ascii_digit() {
                    numbers.push(line.chars().nth(index).unwrap().to_digit(10).unwrap());
                    index += 1;
                    continue;
                }
                let bytes = &bytes[index..];
                index += match bytes {
                    [b'o', b'n', b'e', ..] => {
                        numbers.push(1);
                        2
                    }
                    [b't', b'w', b'o', ..] => {
                        numbers.push(2);
                        2
                    }
                    [b't', b'h', b'r', b'e', b'e', ..] => {
                        numbers.push(3);
                        4
                    }
                    [b'f', b'o', b'u', b'r', ..] => {
                        numbers.push(4);
                        3
                    }
                    [b'f', b'i', b'v', b'e', ..] => {
                        numbers.push(5);
                        3
                    }
                    [b's', b'i', b'x', ..] => {
                        numbers.push(6);
                        2
                    }
                    [b's', b'e', b'v', b'e', b'n', ..] => {
                        numbers.push(7);
                        4
                    }
                    [b'e', b'i', b'g', b'h', b't', ..] => {
                        numbers.push(8);
                        4
                    }
                    [b'n', b'i', b'n', b'e', ..] => {
                        numbers.push(9);
                        3
                    }
                    _ => 1,
                }
            }
        })
        .sum::<u32>()
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("oneight"), 18);
    assert_eq!(part_2("ninetwone"), 91);
    assert_eq!(part_2("nine32twone"), 91);
    assert_eq!(part_2(include_str!("input.txt")), 54925);
}
