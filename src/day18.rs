use itertools::Itertools;

pub mod snailfish {
    use std::ops::AddAssign;
    use std::str::FromStr;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Data(pub Vec<Node>);

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
                let first = self.0[i];
                let second = self.0[i + 1];
                if first.nested == second.nested {
                    if first.nested >= 5 {
                        if i != 0 {
                            self.0[i - 1].num += first.num;
                        }
                        if i + 1 != self.0.len() - 1 {
                            self.0[i + 2].num += second.num;
                        }
                        self.0[i] = Node {
                            num: 0,
                            nested: first.nested - 1,
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
                let Node { num, nested } = self.0[i];
                if num >= 10 {
                    self.0.insert(
                        i + 1,
                        Node {
                            num: if num % 2 == 0 { num / 2 } else { num / 2 + 1 },
                            nested: nested + 1,
                        },
                    );
                    self.0[i].num /= 2;
                    self.0[i].nested += 1;
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
        pub fn magnitude(&self) -> usize {
            use std::collections::VecDeque;
            let mut stack: VecDeque<Node> = VecDeque::new();
            for node in &self.0 {
                stack.push_back(*node);
                while stack.len() >= 2 {
                    let Node {
                        num: left_num,
                        nested: left_nested,
                    } = stack[stack.len() - 2];
                    let Node {
                        num: right_num,
                        nested: right_nested,
                    } = stack[stack.len() - 1];
                    if left_nested == right_nested {
                        stack.pop_back();
                        *stack.back_mut().unwrap() = Node {
                            num: 3 * left_num + 2 * right_num,
                            nested: left_nested - 1,
                        };
                        continue;
                    }
                    break;
                }
            }
            debug_assert_eq!(stack.len(), 1, "stack: {:?}", stack);
            let num = stack[0].num;
            num
        }
    }
    impl AddAssign for Data {
        fn add_assign(&mut self, rhs: Self) {
            self.0.extend(rhs.0.into_iter());
            for node in &mut self.0 {
                node.nested += 1;
            }
            self.reduce()
        }
    }
}

fn part1(raw: &str) {
    let mut lines = raw.split_terminator('\n');
    let mut num = lines.next().unwrap().parse::<snailfish::Data>().unwrap();
    for line in lines {
        num += line.parse::<snailfish::Data>().unwrap();
    }
    println!("{}", num.magnitude());
}

fn part2(raw: &str) -> usize {
    let mut lines = raw.split_terminator('\n');
    lines
        .map(|line| line.parse::<snailfish::Data>().unwrap())
        .permutations(2)
        .map(|mut x| {
            let mut x1 = x.pop().unwrap();
            let x2 = x.pop().unwrap();
            x1 += x2;
            x1.magnitude()
        })
        .max()
        .unwrap()
}

pub fn day18() {
    let raw = std::fs::read_to_string("./inputs/day18.txt").unwrap();
    println!("{}", part2(&raw));
}

#[cfg(test)]
mod tests {
    use super::snailfish::*;
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
        let mut number = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
            .parse::<snailfish::Data>()
            .unwrap();
        assert_eq!(number.explode(), true);
        assert_eq!(number.explode(), true);
        assert_eq!(number.explode(), false);
        assert_eq!(number.split(), true);
        assert_eq!(number.split(), true);
        assert_eq!(number.explode(), true);
    }

    #[test]
    fn add() {
        let mut number = "[[[[4,3],4],4],[7,[[8,4],9]]]"
            .parse::<snailfish::Data>()
            .unwrap();
        number += "[1,1]".parse::<snailfish::Data>().unwrap();
        number.reduce();
        assert_eq!(
            Ok(number),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<snailfish::Data>()
        );
    }

    #[test]
    fn magnitude() {
        let mut number = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            .parse::<snailfish::Data>()
            .unwrap();
        // number.reduce();
        assert_eq!(number.magnitude(), 4140);
    }

    #[test]
    fn max() {
        let raw = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(part2(raw), 3993);
    }
}
