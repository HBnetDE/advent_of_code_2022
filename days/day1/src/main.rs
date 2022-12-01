use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("./input").unwrap().read_to_string(&mut input);

    let elves = parse_input(input);

    let mut highest = 0;
    let mut highest_index = 0;

    for (index, elf) in elves.iter().enumerate() {
        if elf.0 > highest {
            highest = elf.0;
            highest_index = index;
        }
    }

    println!("{highest}");

    let mut amounts: Vec<_> = elves.iter().map(|e| e.0).collect();
    amounts.sort();
    amounts.reverse();

    println!("{}", amounts[0] + amounts[1] + amounts[2])
}

#[derive(Debug, PartialEq)]
struct Elf(u64);

fn parse_input(input: String) -> Vec<Elf> {
    let mut out = vec![];

    let mut counter = 0;

    for line in input.lines() {
        if let Ok(num) = line.parse::<u64>() {
            counter += num;
        } else {
            out.push(Elf(counter));
            counter = 0;
        }
    }

    if counter > 0 {
        out.push(Elf(counter))
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let elves = super::parse_input(
            r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#
            .to_owned(),
        );

        assert_eq!(
            vec![Elf(6000), Elf(4000), Elf(11000), Elf(24000), Elf(10000)],
            elves
        )
    }
}
