use std::collections::VecDeque;

use itertools::Itertools;

fn part1(raw: &str) -> usize {
    let mut lines = raw.lines();
    let enhancer = lines.next().unwrap().parse::<ImageEnhanceStr>().unwrap();
    lines.next();
    let raw = lines.join("\n");
    let image = raw.parse::<Image>().unwrap();
    let image = image.enhance(&enhancer);
    let image = image.enhance(&enhancer);
    image.count_lit()
}

fn part2(raw: &str) -> usize {
    let mut lines = raw.lines();
    let enhancer = lines.next().unwrap().parse::<ImageEnhanceStr>().unwrap();
    lines.next();
    let raw = lines.join("\n");
    let mut image = raw.parse::<Image>().unwrap();
    for _ in 0..50 {
        image = image.enhance(&enhancer);
    }
    image.count_lit()
}

pub fn day20() {
    let raw = std::fs::read_to_string("inputs/day20.txt").unwrap();
    // println!("{}", part1(&raw));
    println!("{}", part2(&raw));
}

pub struct ImageEnhanceStr {
    data: Vec<bool>,
}

impl std::str::FromStr for ImageEnhanceStr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = Vec::with_capacity(512);
        vec.extend(s.chars().flat_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        }));
        Ok(ImageEnhanceStr::new(vec))
    }
}

impl ImageEnhanceStr {
    fn new(data: Vec<bool>) -> Self {
        assert_eq!(data.len(), 512);
        Self { data }
    }

    fn at(&self, index: usize) -> bool {
        self.data[index]
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Image {
    data: VecDeque<VecDeque<bool>>,
    width: usize,
    height: usize,
    infinite_lit: bool,
}

impl std::str::FromStr for Image {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect::<VecDeque<_>>()
            })
            .collect::<VecDeque<_>>();

        Ok(Self::new(data))
    }
}

impl Image {
    pub fn new(data: VecDeque<VecDeque<bool>>) -> Self {
        let width = data[0].len();
        let height = data.len();
        assert_eq!(width, height);
        Self {
            data,
            width,
            height,
            infinite_lit: false,
        }
    }

    fn expand(&mut self) {
        for line in &mut self.data {
            line.push_back(self.infinite_lit);
            line.push_front(self.infinite_lit);
        }
        let mut empty_row = self.data[0].clone();
        for x in &mut empty_row {
            *x = self.infinite_lit;
        }
        self.data.push_back(empty_row.clone());
        self.data.push_front(empty_row);
        self.width += 2;
        self.height += 2;
    }

    fn get(&self, x: isize, y: isize) -> bool {
        if let Some(row) = self.data.get(y as usize) {
            if let Some(elem) = row.get(x as usize) {
                return *elem;
            }
        }
        self.infinite_lit
    }

    fn output_at(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;
        let pixels = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let bits_string = pixels
            .into_iter()
            .map(|(x, y)| self.get(x, y))
            .map(|b| if b { '1' } else { '0' })
            .collect::<String>();
        usize::from_str_radix(&bits_string, 2).unwrap()
    }

    fn enhance(mut self, enhancer: &ImageEnhanceStr) -> Self {
        self.expand();
        let mut new_self = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = self.output_at(x, y);
                new_self.data[y][x] = enhancer.at(pos);
            }
        }
        new_self.infinite_lit = if self.infinite_lit {
            enhancer.at(511)
        } else {
            enhancer.at(0)
        };
        new_self
    }

    fn count_lit(&self) -> usize {
        assert!(!self.infinite_lit);
        self.data.iter().map(|row| row.iter().filter(|&&x| x).count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";

    #[test]
    fn image_algorithm() {
        let result = EX.parse::<ImageEnhanceStr>().unwrap();
        assert_eq!(result.at(34), true);
    }

    const IMG: &str = "#..#.
#....
##..#
..#..
..###";

    #[test]
    fn image() {
        let result = IMG.parse::<Image>().unwrap();
        assert_eq!(result.output_at(2, 2), 34);
        assert_eq!(result.output_at(0, 0), 18);
    }

    #[test]
    fn enhance() {
        let enhancer = EX.parse::<ImageEnhanceStr>().unwrap();
        let image = IMG.parse::<Image>().unwrap();
        let image = image.enhance(&enhancer);
        const RES: &str = ".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.";

        let result_image = RES.parse::<Image>().unwrap();

        assert_eq!(image, result_image);

        let image = image.enhance(&enhancer);

        assert_eq!(image.count_lit(), 35);
    }

    const FULL: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###

";

    #[test]
    fn full() {
        assert_eq!(part1(FULL), 35);
    }
}
