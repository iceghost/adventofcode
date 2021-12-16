use std::fs;
use std::io;

pub fn part1(vec: &mut Vec<isize>) -> isize {
    vec.sort_unstable();
    let median = vec[vec.len() / 2];
    vec.iter().map(|x| (x - median).abs()).sum()
}

pub fn part2(vec: &mut Vec<isize>) -> isize {
    vec.sort_unstable();
    let median = vec[vec.len() / 2];
    let mean = (vec.iter().sum::<isize>() + (vec.len() as isize / 2)) / (vec.len() as isize);
    vec.iter().map(|x| sum_from_one((x - mean).abs())).sum()
}

fn sum_from_one(x: isize) -> isize {
    x * (x + 1) / 2
}

pub fn day7() {
    let raw = fs::read_to_string("./inputs/day7.txt").unwrap();
    let mut vec: Vec<_> = raw
        .split(',')
        .map(|word| word.parse::<isize>().unwrap())
        .collect();
    // println!("{}", part1(&mut vec));
    println!("{}", part2(&mut vec));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(&mut vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
        assert_eq!(part2(&mut vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168);
    }
}
