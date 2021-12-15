use std::iter::Iterator;
use std::fs::*;
use std::io::*;

struct LanternfishSchool(Vec<u8>);

impl Iterator for LanternfishSchool {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.0.len() {
            self.0[i] = match self.0[i] {
                0 => {
                    self.0.push(8);
                    6
                }
                i => i - 1,
            };
        }
        Some(self.0.len())
    }
}

pub fn part1(raw: &str, days: usize) -> usize {
    let data: Vec<_> = raw.split(",").map(|e| e.parse::<u8>().unwrap()).collect();
    let mut fishs = LanternfishSchool(data);
    fishs.nth(days - 1).unwrap()
}

pub fn day6() {
    let reader = BufReader::new(File::open("./inputs/day6.txt").unwrap());
    let line = reader.lines().next().unwrap().unwrap();
    println!("{}", part1(&line, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(part1("3,4,3,1,2", 18) == 26);
        assert!(part1("3,4,3,1,2", 80) == 5934);
    }
}
