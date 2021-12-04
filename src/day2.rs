use std::fs;

enum Direction {
  Up,
  Down,
  Forward,
}

struct Instruction {
  dir: Direction,
  dist: i32,
}

pub fn day2() -> Result<(), ParseError> {
  let content = fs::read_to_string("./inputs/day2.txt").map_err(|_| ParseError::IO)?;
  let lines = content.split("\n");
  let mut dx: i32 = 0;
  let mut dy: i32 = 0;
  let mut aim: i32 = 0;
  for line in lines {
    let Instruction { dir, dist } = parse_line(line)?;
    match dir {
      Direction::Forward => {
        dx += dist;
        dy += dist * aim;
      }
      Direction::Up => {
        aim -= dist;
      }
      Direction::Down => {
        aim += dist;
      }
    };
  }
  println!("{}", dx * dy);
  Ok(())
}

#[derive(Debug)]
pub enum ParseError {
  Direction,
  Int,
  IO,
}

fn parse_line(line: &str) -> Result<Instruction, ParseError> {
  let raw_dir = &line[..line.len() - 2];
  let raw_dist = &line[line.len() - 1..];
  // println!("|{}|{}|", raw_dir, raw_dist);
  let dist = raw_dist.parse::<i32>().map_err(|_| ParseError::Int)?;
  match raw_dir {
    "forward" => Ok(Instruction {
      dir: Direction::Forward,
      dist,
    }),
    "up" => Ok(Instruction {
      dir: Direction::Up,
      dist,
    }),
    "down" => Ok(Instruction {
      dir: Direction::Down,
      dist,
    }),
    _ => Err(ParseError::Direction),
  }
}
