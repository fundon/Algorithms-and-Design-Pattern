use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use std::iter::Iterator;

#[derive(Debug)]
enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

#[derive(Debug, Clone, Hash, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // let a = ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt() as i32;
        // let b = ((other.x.pow(2) + other.y.pow(2)) as f32).sqrt() as i32;
        let a = self.x.abs() + self.y.abs();
        let b = other.x.abs() + other.y.abs();
        Some(a.cmp(&b))
    }
}

impl Eq for Coordinate {}

fn process(input: Vec<Vec<Direction>>) -> (i32, i32) {
    let coordinates = input
        .iter()
        .map(|v| {
            let mut set = Vec::new();
            let t = v
                .iter()
                .scan(Coordinate { x: 0, y: 0 }, |a, e| {
                    let mut x = 0;
                    let mut y = 0;
                    match *e {
                        Direction::Up(n) => {
                            y = n;
                        }
                        Direction::Right(n) => {
                            x = n;
                        }
                        Direction::Down(n) => {
                            y = -n;
                        }
                        Direction::Left(n) => {
                            x = -n;
                        }
                    }

                    if x != 0 {
                        let v = x.abs();
                        let d = x / v;
                        for i in 1..=v {
                            let c = Coordinate {
                                x: a.x + i * d,
                                y: a.y,
                            };
                            set.push(c);
                        }
                        a.x += x;
                    } else if y != 0 {
                        let v = y.abs();
                        let d = y / v;
                        for i in 1..=v {
                            let c = Coordinate {
                                x: a.x,
                                y: a.y + i * d,
                            };
                            set.push(c);
                        }
                        a.y += y;
                    }

                    Some(*a)
                })
                .collect::<Vec<Coordinate>>();

            drop(t);

            set
        })
        .collect::<Vec<Vec<Coordinate>>>();

    let a = &coordinates[0];
    let b = &coordinates[1];
    let ah: HashSet<_> = a.iter().cloned().collect();
    let bh: HashSet<_> = b.iter().cloned().collect();

    let mut marks: Vec<Coordinate> = ah
        .intersection(&bh)
        .collect::<HashSet<&Coordinate>>()
        .iter()
        .map(|x| **x)
        .collect();

    // dbg!(&marks);

    let mut r = (0, 0);

    r.1 = marks
        .iter()
        .map(|c| {
            a.into_iter()
                .collect::<Vec<&Coordinate>>()
                .iter()
                .position(|x| *x == c)
                .unwrap()
                + 1
                + b.into_iter()
                    .collect::<Vec<&Coordinate>>()
                    .iter()
                    .position(|x| *x == c)
                    .unwrap()
                + 1
        })
        .min()
        .unwrap() as i32;

    marks.sort();

    if let Some(c) = marks.first() {
        r.0 = c.x.abs() + c.y.abs();
    }

    r
}

fn parse<S: AsRef<str>>(line: S) -> Vec<Direction> {
    line.as_ref()
        .trim()
        .split(',')
        .map(|s| {
            let (d, n) = s.split_at(1);
            let n = n
                .parse::<i32>()
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
                .unwrap();
            match d.chars().nth(0).unwrap() {
                'U' => Direction::Up(n),
                'R' => Direction::Right(n),
                'D' => Direction::Down(n),
                'L' => Direction::Left(n),
                _ => unimplemented!(),
            }
        })
        .collect()
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let file = File::open("inputs/3").await?;
    let mut lines = BufReader::new(file).lines();
    let mut input = Vec::new();

    while let Some(line) = lines.next().await {
        input.push(parse(line?));
    }

    let r = process(input);
    println!("Its distance is {}", r.0);
    println!("A total of {} steps for the first mark", r.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day_3() {
        assert_eq!(
            (6, 30),
            process(vec![parse("R8,U5,L5,D3"), parse("U7,R6,D4,L4")])
        );

        assert_eq!(
            (159, 610),
            process(vec![
                parse("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                parse("U62,R66,U55,R34,D71,R55,D58,R83")
            ])
        );

        assert_eq!(
            (135, 410),
            process(vec![
                parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ])
        );
    }
}
