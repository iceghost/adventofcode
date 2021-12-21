use itertools::Itertools;
use nom::number::complete::be_u8;
use std::num::ParseIntError;
use std::ops::AddAssign;

pub mod snailfish {
    use std::collections::VecDeque;
    use std::ops::AddAssign;
    use itertools::Itertools;
    use std::str::FromStr;

    #[derive(Debug, Eq, PartialEq)]
    pub struct Data(pub Vec<Node>);

    #[derive(Debug, Eq, PartialEq)]
    pub struct Node {
        pub num: usize,
        pub nested: usize,
    }

    impl FromStr for Data {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut return_val = vec![];
            let mut nested = 0usize;
            let mut num_buffer = String::new();
            for ch in s.chars() {
                match ch {
                    '[' => nested += 1,
                    ',' => {
                        if !num_buffer.is_empty() {
                            return_val.push(Node {
                                num: num_buffer.parse::<usize>().unwrap(),
                                nested,
                            });
                            num_buffer.clear();
                        }
                    }
                    ']' => {
                        if !num_buffer.is_empty() {
                            return_val.push(Node {
                                num: num_buffer.parse::<usize>().unwrap(),
                                nested,
                            });
                            num_buffer.clear();
                        }
                        nested -= 1
                    }
                    d => num_buffer.push(d),
                }
            }
            Ok(Self(return_val))
        }
    }

    impl Data {
        pub fn explode(&mut self) -> bool {
            for i in 0..self.0.len() - 1 {
                if self.0[i].nested == self.0[i + 1].nested {
                    if self.0[i].nested >= 5 {
                        if i != 0 {
                            self.0[i - 1].num += self.0[i].num;
                        }
                        if i + 1 != self.0.len() - 1 {
                            self.0[i + 2].num += self.0[i + 1].num;
                        }
                        self.0[i] = Node {
                            num: 0,
                            nested: self.0[i].nested - 1,
                        };
                        self.0.remove(i + 1);
                        return true;
                    }
                }
            }
            false
        }
        pub fn split(&mut self) -> bool {
            for i in 0..self.0.len() {
                let mut node = &mut self.0[i];
                if self.0[i].num >= 10 {
                    self.0.insert(
                        i + 1,
                        Node {
                            num: if self.0[i].num % 2 == 0 {
                                self.0[i].num / 2
                            } else {
                                self.0[i].num / 2 + 1
                            },
                            nested: self.0[i].nested + 1,
                        },
                    );
                    self.0[i] = Node {
                        num: self.0[i].num / 2,
                        nested: self.0[i].nested + 1,
                    };
                    return true;
                }
            }
            false
        }
        pub fn reduce(&mut self) {
            loop {
                if !self.explode() {
                    if !self.split() {
                        return;
                    }
                }
            }
        }
        pub fn magnitude(self) -> usize {
            let mut stack: VecDeque<Node> = VecDeque::new();
            for node in self.0 {
                stack.push_back(node);
                let mut len = stack.len();
                while len >= 2 && stack[len - 1].nested == stack[len - 2].nested {
                    stack[len - 2] = Node {
                        num: 3 * stack[len - 2].num + 2 * stack[len - 1].num,
                        nested: stack[len - 1].nested - 1,
                    };
                    stack.pop_back();
                    len = stack.len();
                }
            }
            debug_assert_eq!(stack.len(), 1);
            stack[0].num
        }
    }
    impl AddAssign for Data {
        fn add_assign(&mut self, rhs: Self) {
            self.0.extend(rhs.0.into_iter());
            for node in &mut self.0 {
                node.nested += 1;
            }
        }
    }
}

pub fn day18() {
    let raw = std::fs::read_to_string("./inputs/day18.txt").unwrap();
    let mut lines = raw.split_terminator('\n');
    let mut num = lines.next().unwrap().parse::<snailfish::Data>().unwrap();
    for line in lines {
        num += line.parse::<snailfish::Data>().unwrap();
        num.reduce();
    }
    println!("{}", num.magnitude());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser() {
        assert_eq!(
            "[1,1]".parse::<snailfish::Data>(),
            Ok(snailfish::Data(vec![
                snailfish::Node { num: 1, nested: 1 },
                snailfish::Node { num: 1, nested: 1 }
            ]))
        );
        assert_eq!(
            "[[1,2],1]".parse::<snailfish::Data>(),
            Ok(snailfish::Data(vec![
                snailfish::Node { num: 1, nested: 2 },
                snailfish::Node { num: 2, nested: 2 },
                snailfish::Node { num: 1, nested: 1 }
            ]))
        );
    }

    #[test]
    fn explode() {
        let mut number1 = "[[[[[9,8],1],2],3],4]".parse::<snailfish::Data>().unwrap();
        assert_eq!(number1.explode(), true);
        assert_eq!(number1.explode(), false);
    }

    #[test]
    fn split() {
        let mut number = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse::<snailfish::Data>().unwrap();
        assert_eq!(number.explode(), true);
        assert_eq!(number.explode(), true);
        assert_eq!(number.explode(), false);
        assert_eq!(number.split(), true);
        assert_eq!(number.split(), true);
        assert_eq!(number.explode(), true);
    }

    #[test]
    fn add() {
        let mut number = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<snailfish::Data>().unwrap();
        number += "[1,1]".parse::<snailfish::Data>().unwrap();
        number.reduce();
        assert_eq!(Ok(number), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<snailfish::Data>());
    }

    #[test]
    fn magnitude() {
        let mut number = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse::<snailfish::Data>().unwrap();
        number.reduce();
        assert_eq!(number.magnitude(), 4140);
    }
}
