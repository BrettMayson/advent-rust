#[derive(Debug)]
pub enum Instruction {
    Mul(i64, i64),
    EnableMul,
    DisableMul,
}

#[allow(clippy::while_let_on_iterator)]
fn find_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut buffer = String::new();
    let mut left = String::new();
    let mut right = String::new();
    let mut stream = input.chars();
    while let Some(char) = stream.next() {
        buffer.push(char);
        if char == '(' {
            'outer: {
                if buffer.ends_with("mul(") {
                    // read numbers into left until comma, reset if not comma
                    while let Some(char) = stream.next() {
                        if char.is_ascii_digit() {
                            left.push(char);
                        } else if char == ',' {
                            break;
                        } else {
                            break 'outer;
                        }
                    }
                    // read numbers into right until closing parenthesis, reset if not closing parenthesis
                    while let Some(char) = stream.next() {
                        if char.is_ascii_digit() {
                            right.push(char);
                        } else if char == ')' {
                            instructions.push(Instruction::Mul(
                                left.parse().unwrap(),
                                right.parse().unwrap(),
                            ));
                            break;
                        } else {
                            break;
                        }
                    }
                    buffer.clear();
                }
            }
            left.clear();
            right.clear();
        } else if char == ')' {
            if buffer.ends_with("do()") {
                instructions.push(Instruction::EnableMul);
            } else if buffer.ends_with("don't()") {
                instructions.push(Instruction::DisableMul);
            }
            buffer.clear();
        }
    }
    instructions
}

fn execute(instructions: &[Instruction], part: usize) -> i64 {
    instructions
        .iter()
        .fold(
            (0, 0, true),
            |(i, mul, enabled), instruction| match instruction {
                Instruction::Mul(left, right) => {
                    if enabled || part == 1 {
                        (i + 1, mul + left * right, enabled)
                    } else {
                        (i + 1, mul, enabled)
                    }
                }
                Instruction::EnableMul => (i + 1, mul, true),
                Instruction::DisableMul => (i + 1, mul, false),
            },
        )
        .1
}

#[test]
fn part1_example() {
    let instructions = find_instructions(include_str!("example1.txt"));
    assert_eq!(execute(&instructions, 1), 161);
}

#[test]
fn part1_input() {
    let instructions = find_instructions(include_str!("input.txt"));
    assert_eq!(execute(&instructions, 1), 165_225_049);
}

#[test]
fn part2_example() {
    let instructions = find_instructions(include_str!("example2.txt"));
    assert_eq!(execute(&instructions, 2), 48);
}

#[test]
fn part2_input() {
    let instructions = find_instructions(include_str!("input.txt"));
    assert_eq!(execute(&instructions, 2), 108_830_766);
}
