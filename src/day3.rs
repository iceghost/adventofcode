use std::fs;
use std::ops;

#[derive(Debug)]
struct Counter(u32, u32);

impl Counter {
    fn add_zero(&mut self) {
        self.0 += 1;
    }
    fn add_one(&mut self) {
        self.1 += 1;
    }
}

pub fn day3() {
    let content = fs::read_to_string("./inputs/day3.txt").unwrap();
    let mut lines = content.split("\r\n");
    let mut arr: Vec<Counter> = lines
        .next()
        .unwrap()
        .chars()
        .map(|b| b.to_digit(2).unwrap())
        .map(|d| if d == 0 { Counter(1, 0) } else { Counter(0, 1) })
        .collect();
    for line in lines {
        line.chars()
            .map(|b| b.to_digit(2).unwrap())
            .zip(&mut arr)
            .for_each(|(d, counter)| {
                if d == 0 {
                    counter.add_zero()
                } else {
                    counter.add_one()
                }
            });
    }
    let result1: String = arr
        .iter()
        .map(|counter| if counter.0 < counter.1 { '0' } else { '1' })
        .collect();
    let result2: String = arr
        .iter()
        .map(|counter| if counter.0 > counter.1 { '0' } else { '1' })
        .collect();
    // println!("{}", result1);
    // println!("{}", result2);
    let dec1 = u64::from_str_radix(&result1, 2).unwrap();
    let dec2 = u64::from_str_radix(&result2, 2).unwrap();
    println!("{} * {} = {}", dec1, dec2, dec1 * dec2);
}
