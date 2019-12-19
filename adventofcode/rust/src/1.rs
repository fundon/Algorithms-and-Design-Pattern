use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use std::io::{Error, ErrorKind};
use std::iter::Iterator;

struct Fuel(u32);

impl Iterator for Fuel {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = fuel(self.0);
        if res > 0 {
            self.0 = res;
            Some(res)
        } else {
            None
        }
    }
}

fn fuel(i: u32) -> u32 {
    (i / 3).saturating_sub(2)
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let file = File::open("inputs/1").await?;
    let mut lines = BufReader::new(file).lines();
    let mut part_one = 0;
    let mut part_two = 0;

    while let Some(line) = lines.next().await {
        let mass = line?
            .parse::<u32>()
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

        part_one += fuel(mass);
        part_two += Fuel(mass).into_iter().sum::<u32>();
    }

    println!("part 1: {}", part_one);
    println!("part 2: {}", part_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(2, fuel(12));
        assert_eq!(2, fuel(14));
        assert_eq!(654, fuel(1969));
        assert_eq!(33583, fuel(100756));

        assert_eq!(2, Fuel(12).into_iter().sum::<u32>());
        assert_eq!(2, Fuel(14).into_iter().sum::<u32>());
        assert_eq!(966, Fuel(1969).into_iter().sum::<u32>());
        assert_eq!(50346, Fuel(100756).into_iter().sum::<u32>());
    }
}
