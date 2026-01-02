//! # Gift Shop

use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(",")
        .map(|pair| {
            let (a, b) = pair.split_once("-").expect("unable to parse");
            (
                a.parse::<i64>().expect(format!("unable to convert a {a} to int").as_str()),
                b.parse::<i64>().expect(format!("unable to convert b {b} to int").as_str()),
            )
        })
        .collect()
}

pub fn part1(input: &Vec<(i64, i64)>) -> i64 {
    let mut count = 0;
    
    for (first, second) in input {
        for num in first.clone()..=second.clone() {
            let str = num.to_string();
            
            if str.len() % 2 != 0 {
                continue;
            }

            let (first_half, second_half) = str.split_at(str.len() / 2);
            if first_half == second_half {
                count += num;
            }
        }
    }

    count
}

pub fn part2(input: &Vec<(i64, i64)>) -> u32 {
    let mut count: u32 = 0;

    for (first, second) in input {
        for num in first.clone()..=second.clone() {
            let str = num.to_string();
            let mut values: HashSet<u32> = HashSet::new();

            // println!("str {str} / {}", str.len() % 2);
            
            // if str.len() % 2 != 0 {
            //     continue;
            // }

            // println!("{}", str.len() / 2);

            for midpoint in 1..=(str.len() / 2) {
                if str.len() % midpoint != 0 {
                    continue;
                }

                // println!("midpoint {midpoint}, str {str}");

                
                let substr = &str[..midpoint];
                // println!("checking: {str} with {substr}");
                let repeated = substr.repeat(str.len() / midpoint);
                
                if repeated == str {
                    // count += num as u32;
                    values.insert(num as u32);
                }
            }

            for value in values {
                println!("adding {num}");
                count += value;
            }
        }
    }

    count
}
