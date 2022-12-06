extern crate criterion;
use std::{fs::File, io::Read};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let data = data();
    c.bench_function("day 5 - part 1", |b| b.iter(|| part_1(black_box(&data))));
    c.bench_function("day 5 - part 2", |b| b.iter(|| part_2(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn data() -> String {
    let mut data = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut data).unwrap();
    data
}

fn parse_columns(data: &str) -> (Vec<Vec<char>>, &str) {
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
    (columns, orders)
}

fn part_1(data: &str) {
    let (mut columns, orders) = parse_columns(data);
    for order in orders.lines() {
        let data = order.split(' ').map(|a| a.parse::<usize>()).filter_map(|a| a.ok()).collect::<Vec<_>>();
        for _ in 0..data[0] {
            let bin = columns[data[1] - 1].pop();
            columns[data[2] - 1].push(bin.unwrap());
        }
    }
}

fn part_2(data: &str) {
    let (mut columns, orders) = parse_columns(data);
    for order in orders.lines() {
        let data = order.split(' ').map(|a| a.parse::<usize>()).filter_map(|a| a.ok()).collect::<Vec<_>>();
        let mut stack = Vec::with_capacity(data[0]);
        for _ in 0..data[0] {
            let bin = columns[data[1] - 1].pop();
            stack.push(bin.unwrap());
        }
        stack.reverse();
        columns[data[2] - 1].append(&mut stack);
    }
}
