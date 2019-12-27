use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use std::io::{Error, ErrorKind};
use std::iter::Iterator;

fn process(input: &mut Vec<u32>) -> Vec<u32> {
    let l = input.len();
    let mut i = 0;

    while i < l {
        let n = input[i];

        if (n == 1 || n == 2) && (i + 3 < l) {
            let a = input[i + 1] as usize;
            let b = input[i + 2] as usize;
            let p = input[i + 3] as usize;

            if a < l && b < l && p < l {
                if n == 1 {
                    input[p] = input[a] + input[b];
                } else {
                    input[p] = input[a] * input[b];
                }
            }

            i += 3;
        } else if n == 99 {
            break;
        } else {
            i += 1;
        }
    }

    input.to_owned()
}

fn wrap_process(input: &mut Vec<u32>, a: u32, b: u32) -> Vec<u32> {
    input[1] = a;
    input[2] = b;

    // println!("input = {:?}\n", input);

    process(input)
}

fn answer(noun: u32, verb: u32) -> u32 {
    100 * noun + verb
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let file = File::open("inputs/2").await?;
    let mut lines = BufReader::new(file).lines();
    let mut input = Vec::new();

    if let Some(line) = lines.next().await {
        input.append(
            &mut line?
                .trim()
                .split(',')
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
                        .unwrap()
                })
                .collect(),
        );
    }

    println!("part 1 = {}", wrap_process(&mut input.clone(), 12, 2)[0]);

    for noun in 0..=99 {
        for verb in 0..=99 {
            if 19_690_720 == wrap_process(&mut input.clone(), noun, verb)[0] {
                println!(
                    "noun: {}, verb: {}, part 2 = {}",
                    noun,
                    verb,
                    answer(noun, verb)
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day_2() {
        assert_eq!(2, process(&mut vec![1, 0, 0, 0, 99])[0]);

        let v = process(&mut vec![2, 3, 0, 3, 99]);
        assert_eq!(2, v[0]);
        assert_eq!(6, v[3]);

        let v = process(&mut vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(2, v[0]);
        assert_eq!(9801, v[5]);

        let v = process(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(v, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
