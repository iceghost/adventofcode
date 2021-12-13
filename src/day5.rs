extern crate itertools;
use itertools::*;
use std::fs::*;
use std::io::*;
use std::prelude::rust_2021::*;

#[derive(Clone, Debug)]
struct Line {
    from: (u32, u32),
    to: (u32, u32),
}

impl Line {
    fn new(line: &str) -> Self {
        let splits: Vec<&str> = line.split(" -> ").collect();
        let parse_pair = |split: &str| -> (u32, u32) {
            let mut nums: Vec<u32> = split
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            (nums[0], nums[1])
        };
        Self {
            from: parse_pair(splits[0]),
            to: parse_pair(splits[1]),
        }
    }
}

#[derive(Debug, Clone)]
struct HorizontalLine {
    y: u32,
    x_from: u32,
    x_to: u32,
}

impl HorizontalLine {
    fn new(line: &Line) -> Option<Self> {
        if line.from.1 != line.to.1 {
            None
        } else {
            let (x_from, x_to) = if line.from.0 < line.to.0 {
                (line.from.0, line.to.0)
            } else {
                (line.to.0, line.from.0)
            };

            Some(Self {
                y: line.from.1,
                x_from,
                x_to,
            })
        }
    }

    fn overlap((hline1, hline2): (&HorizontalLine, &HorizontalLine)) -> Vec<(u32, u32)> {
        if hline1.y == hline2.y && hline1.x_from <= hline2.x_to && hline2.x_from <= hline1.x_to {
            let mut quad = vec![hline1.x_from, hline1.x_to, hline2.x_from, hline2.x_to];
            quad.sort();
            (quad[1]..quad[2] + 1)
                .map(|x| (x, hline1.y))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
struct VerticalLine {
    x: u32,
    y_from: u32,
    y_to: u32,
}

impl VerticalLine {
    fn new(line: &Line) -> Option<Self> {
        if line.from.0 != line.to.0 {
            None
        } else {
            let (y_from, y_to) = if line.from.1 < line.to.1 {
                (line.from.1, line.to.1)
            } else {
                (line.to.1, line.from.1)
            };
            Some(Self {
                x: line.from.0,
                y_from,
                y_to,
            })
        }
    }

    fn overlap((vline1, vline2): (&VerticalLine, &VerticalLine)) -> Vec<(u32, u32)> {
        if vline1.x == vline2.x && vline1.y_from <= vline2.y_to && vline2.y_from <= vline1.y_to {
            let mut quad = vec![vline1.y_from, vline1.y_to, vline2.y_from, vline2.y_to];
            quad.sort();
            (quad[1]..quad[2])
                .map(|y| (vline1.x, y))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }
}

pub fn day5() {
    let reader = BufReader::new(File::open("./inputs/day5.txt").unwrap());
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut hlines: Vec<HorizontalLine> = vec![];
    let mut vlines: Vec<VerticalLine> = vec![];
    for line in lines {
        let line = Line::new(&line);
        if let Some(h_line) = HorizontalLine::new(&line) {
            hlines.push(h_line);
            continue;
        } else if let Some(v_line) = VerticalLine::new(&line) {
            vlines.push(v_line);
            continue;
        }
    }
    let mut list: Vec<(u32, u32)> = iproduct!((&hlines).into_iter(), (&vlines).into_iter())
        .map(intersect)
        .filter_map(|x| x)
        .collect();
    println!("{:?}", hlines.len());
    for i in 0..hlines.len() {
        for j in i + 1..hlines.len() {
            list.append(&mut HorizontalLine::overlap((&hlines[i], &hlines[j])));
        }
    }
    for i in 0..vlines.len() {
        for j in i + 1..vlines.len() {
            list.append(&mut VerticalLine::overlap((&vlines[i], &vlines[j])));
        }
    }
    list = list.into_iter().unique().collect();
    println!("hlines: {:?}", list.len())
}

fn intersect((hline, vline): (&HorizontalLine, &VerticalLine)) -> Option<(u32, u32)> {
    if (hline.x_from as i32 - vline.x as i32) * (hline.x_to as i32 - vline.x as i32) <= 0
        && (vline.y_from as i32 - hline.y as i32) * (vline.y_to as i32 - hline.y as i32) <= 0
    {
        Some((vline.x, hline.y))
    } else {
        None
    }
}
