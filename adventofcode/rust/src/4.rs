use async_std::io;

struct Numbers(u32);

impl Iterator for Numbers {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            _ => {
                let d = self.0 % 10;
                self.0 /= 10;
                Some(d as Self::Item)
            }
        }
    }
}

fn process_one(num: u32) -> (u8, u8) {
    Numbers(num)
        .try_fold(
            (10, 0, 0, 0),
            |(last, repeat, mut count, mut double), curr| {
                if curr < last {
                    Some((curr, 0, count, double))
                } else if curr == last {
                    if repeat == 0 {
                        double += 1;
                        count += 1;
                    } else if repeat == 1 {
                        double -= 1;
                    }
                    Some((curr, repeat + 1, count, double))
                } else {
                    None
                }
            },
        )
        .map_or((0, 0), |(_, _, count, double)| (count, double))
}

fn check_part1(m: (u8, u8)) -> bool {
    m.0 > 0
}

fn check_part2(m: (u8, u8)) -> bool {
    m.1 > 0
}

fn process(start: u32, end: u32, f: impl Fn((u8, u8)) -> bool) -> usize {
    (start..=end).filter(|&x| f(process_one(x))).count()
}

#[async_std::main]
async fn main() -> io::Result<()> {
    println!("part 1: {}", process(307237, 769058, check_part1));
    println!("part 2: {}", process(307237, 769058, check_part2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day_4() {
        assert_eq!(true, check_part1(process_one(111111)));
        assert_eq!(false, check_part1(process_one(223450)));
        assert_eq!(false, check_part1(process_one(123789)));
        assert_eq!(false, check_part1(process_one(111121)));

        assert_eq!(false, check_part2(process_one(111111)));
        assert_eq!(false, check_part2(process_one(223450)));
        assert_eq!(true, check_part2(process_one(112233)));
        assert_eq!(false, check_part2(process_one(123444)));
        assert_eq!(false, check_part2(process_one(124444)));
        assert_eq!(true, check_part2(process_one(112222)));
        assert_eq!(true, check_part2(process_one(111122)));
        assert_eq!(false, check_part2(process_one(111112)));
    }
}
