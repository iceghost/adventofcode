use std::iter::Iterator;
use std::fs::*;
use std::io::*;

struct SizeCache(Vec<usize>);

impl SizeCache {
    fn new() -> Self {
        //  -8 -7 -6 -5 -4 -3 -2 -1  0
        SizeCache(vec![1, 1, 1, 1, 1, 1, 1, 1, 1])
    }

    fn get(&mut self, index: &mut isize) -> usize {
        // normalize
        *index += 8;
        while *index as usize >= self.0.len() {
            self.0.push(self.0[*index as usize - 7] + self.0[*index as usize - 9]);
        }
        self.0[*index as usize]
    }
}

struct LanternfishSchool {
    data: Vec<u8>,
    day: usize,
    sizeCache: SizeCache,
}

impl LanternfishSchool {
    fn new(data: Vec<u8>) -> Self {
        Self { data, day: 0, sizeCache : SizeCache::new() }
    }
}

impl Iterator for LanternfishSchool {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut size = 0;
        for fish in &self.data {
            size += self.sizeCache.get(&mut (self.day as isize - *fish as isize));
        }
        self.day += 1;
        Some(size)
    }
}

pub fn part1(raw: &str, days: usize) -> usize {
    let data: Vec<_> = raw.split(",").map(|e| e.parse::<u8>().unwrap()).collect();
    let mut fishs = LanternfishSchool::new(data);
    fishs.nth(days).unwrap()
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
        assert!(part1("3,4,3,1,2", 256) == 26984457539);
    }
}
