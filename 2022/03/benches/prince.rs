use std::{fs::File, io::Read};

extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn data() -> Vec<String> {
    let mut data = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut data).unwrap();
    data.lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn part_1(data: &[&str]) {
    data.iter()
        .map(|l| {
            let len = l.len() / 2;
            let b = &l[len..];
            priority(l[0..len].chars().find(|i| b.contains(*i)).unwrap())
        }).sum::<u32>();
}

fn part_2(data: &[&str]) {
    data.chunks(3).map(|chunk| {
        let a = chunk[0]
            .chars()
            .find(|i| chunk[1].contains(*i) && chunk[2].contains(*i));
        priority(a.unwrap())
    }).sum::<u32>();
}

fn criterion_benchmark(c: &mut Criterion) {
    let source_data = data();
    let data = source_data.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    c.bench_function("day 3 - part 1", |b| b.iter(|| part_1(black_box(&data))));
    c.bench_function("day 3 - part 2", |b| b.iter(|| part_2(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn priority(c: char) -> u32 {
    let value = c as u32;
    if (97..=122).contains(&value) {
        return value - 96;
    }
    if (65..=90).contains(&value) {
        return value - 38;
    }
    panic!("Invalid item");
}

