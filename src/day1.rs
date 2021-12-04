use std::fs;

pub fn day1() -> Option<()> {
    let content = fs::read_to_string("inputs/day1.txt").ok()?;
    let it = content.split('\n').map(|s| s.parse::<i32>().unwrap());
    let it_lead = it.clone().skip(3);
    let count = it.zip(it_lead).map(|(x, y)| y - x).filter(|x| *x > 0).count();
    print!("{}", count);
    None
}