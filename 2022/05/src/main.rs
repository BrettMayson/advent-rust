use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut data = String::new();
        let mut file = File::open("./05/input.txt").unwrap();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let (stacks, orders) = data.split_once("\n\n").unwrap();
    let mut stacks = stacks.lines().collect::<Vec<_>>();
    stacks.reverse();
    let mut stack_iter = stacks.iter();
    let mut columns = vec![Vec::with_capacity(50); 9];
    stack_iter.next().unwrap();
    for line in stack_iter {
        for i in 0..9 {
            columns[i].push(line.chars().nth(i * 4 + 1).unwrap());
        }
    }
    for i in 0..9 {
        columns[i].retain(|i| *i != ' ');
    }
    println!("{:?}", columns);

    let mut part1_columns = columns.clone();

    for order in orders.lines() {
        let data = order.split(' ').map(|a| a.parse::<usize>()).filter_map(|a| a.ok()).collect::<Vec<_>>();
        for _ in 0..data[0] {
            let bin = part1_columns[data[1] - 1].pop();
            part1_columns[data[2] - 1].push(bin.unwrap());
        }
    }

    print!("Part One: ");
    for i in 0..9 {
        print!("{}", part1_columns[i][part1_columns[i].len() - 1]);
    }
    println!();

    let mut part2_columns = columns.clone();

    for order in orders.lines() {
        let data = order.split(' ').map(|a| a.parse::<usize>()).filter_map(|a| a.ok()).collect::<Vec<_>>();
        let mut stack = Vec::with_capacity(data[0]);
        for _ in 0..data[0] {
            let bin = part2_columns[data[1] - 1].pop();
            stack.push(bin.unwrap());
        }
        stack.reverse();
        part2_columns[data[2] - 1].append(&mut stack);
    }

    print!("Part Two: ");
    for i in 0..9 {
        print!("{}", part2_columns[i][part2_columns[i].len() - 1]);
    }
    println!();
}
