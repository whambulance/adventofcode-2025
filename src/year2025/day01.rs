//! # Secret Entrance

use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    let direction = input.bytes().filter(|&b| b.is_ascii_uppercase());
    let amount = input.iter_signed::<i32>();
    direction.zip(amount).map(|(d, a)| if d == b'R' { a } else { -a }).collect()
}

pub fn part1(input: &Vec<i32>) -> i32 {
    let mut initial = 50;
    let mut count = 0;

    for int in input.iter() {
        initial += int;
        count += if initial % 100 == 0 { 1 } else { 0 };
    }

    count
}

pub fn part2(input: &Vec<i32>) -> i32 {
    let mut initial = 1050;
    let mut count = 0;

    for int in input.iter() {
        for _ in 0..int.abs() {
            if *int > 0 {
                initial += 1;
            } else {
                initial -= 1;
            }
            count += if initial % 100 == 0 { 1 } else { 0 };
        }
    }

    count
}