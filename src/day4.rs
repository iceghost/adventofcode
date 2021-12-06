use std::cmp::*;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::Sum;

#[derive(Debug)]
struct Board(Vec<Vec<Cell>>, u8);

#[derive(Debug, PartialEq, Eq, Ord)]
struct Cell(u32, u8);

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
        match self.1.partial_cmp(&other.1) {
            Some(Ordering::Equal) => self.0.partial_cmp(&other.0).map(|c| c.reverse()),
            x => x,
        }
    }
}

impl Board {
    fn new<'a, It>(it: &mut It) -> Option<Board>
    where
        It: Iterator<Item = String>,
    {
        let mut board: Vec<Vec<Cell>> = vec![];
        // blank line
        it.next();
        // next five lines
        for _ in 0..5 {
            let row: Vec<Cell> = it
                .next()?
                .split_ascii_whitespace()
                .map(|dd| Cell(dd.parse::<u32>().unwrap(), 0))
                .collect();
            board.push(row);
        }
        Some(Board(board, 0))
    }

    fn mark(&mut self, num: u32) {
        self.1 += 1;
        for row in &mut self.0 {
            if let Some(cell) = row.into_iter().find(|x| x.0 == num) {
                cell.1 = self.1;
                break;
            }
        }
    }

    fn is_ok(&self) -> Option<Cell> {
        for row in &self.0 {
            if row.into_iter().all(|cell| cell.1 != 0) {
                return Some(row.into_iter().fold(Cell(0, 0), |mut acc, cell| {
                    acc.0 += cell.0;
                    acc.1 = max(acc.1, cell.1);
                    acc
                }));
            }
        }
        for j in 0..self.0[0].len() {
            let mut col = (&self.0).into_iter().map(|row| &row[j]);
            if col.all(|cell| cell.1 != 0) {
                return Some(col.fold(Cell(0, 0), |mut acc, cell| {
                    acc.0 += cell.0;
                    acc.1 = min(acc.1, cell.1);
                    acc
                }));
            }
        }
        None
    }
}

// struct BoardParser<'a, It>(&'a mut It)
// where
//     It: Iterator<Item = String>;

pub fn day4() {
    let reader = BufReader::new(fs::File::open("./inputs/day4.txt").unwrap());
    let mut lines = reader.lines().map(|v| v.unwrap());
    let line = lines.next().unwrap();
    let numbers: Vec<_> = line
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let mut max_result = Cell(0, 0);
    while let Some(mut board) = Board::new(&mut lines) {
        for &num in &numbers {
            board.mark(num);
            if let Some(result) = board.is_ok() {
                println!("{:?}", result);
                max_result = min(max_result, result);
                break;
            }
        }
    }
}
