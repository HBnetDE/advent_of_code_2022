fn main() {
    for i in (0..15).step_by(3) {
        println!("{i}")
    }

    let input = include_str!("../input");
    round_one(input);
    round_two(input);
}

fn round_one(input: &str) {
    let answer: u32 = input
        .lines()
        .map(|l| Rucksack::new(l))
        .map(|r| Priority::from(r.duplicate_item()))
        .map(|p| p.0)
        .sum();

    println!("Sum of priorities: {answer}");
}

fn round_two(input: &str) {
    let rucksacks: Vec<_> = input.lines().map(|l| Rucksack::new(l)).collect();

    let mut groups: Vec<RucksackGroup> = Vec::new();

    for i in (0..rucksacks.len()).step_by(3) {
        groups.push(RucksackGroup {
            first: rucksacks[i].clone(),
            second: rucksacks[i + 1].clone(),
            third: rucksacks[i + 2].clone(),
        });
    }

    let answer: u32 = groups
        .iter()
        .map(|g| Priority::from(g.badge()))
        .map(|p| p.0)
        .sum();

    println!("Sum of priorities: {answer}");
}

#[derive(Clone)]
struct Rucksack<'a> {
    content: &'a str,
}

struct RucksackGroup<'a> {
    first: Rucksack<'a>,
    second: Rucksack<'a>,
    third: Rucksack<'a>,
}

impl<'a> Rucksack<'a> {
    fn new(content: &'a str) -> Self {
        Rucksack { content }
    }
    fn first_compartment(&self) -> &str {
        let (first, _) = self.content.split_at(self.content.len() / 2);
        first
    }
    fn second_compartment(&self) -> &str {
        let (_, second) = self.content.split_at(self.content.len() / 2);
        second
    }
    fn duplicate_item(&self) -> char {
        for c in self.second_compartment().chars() {
            if self.first_compartment().contains(c) {
                return c;
            }
        }
        unreachable!()
    }
}

impl<'a> RucksackGroup<'a> {
    fn new(rucksacks: Vec<Rucksack<'a>>) -> Self {
        Self {
            first: rucksacks[0].clone(),
            second: rucksacks[1].clone(),
            third: rucksacks[2].clone(),
        }
    }
    fn badge(&self) -> char {
        for c in self.first.content.chars() {
            if self.second.content.contains(c) && self.third.content.contains(c) {
                return c;
            }
        }
        unreachable!()
    }
}

struct Priority(u32);

impl From<char> for Priority {
    fn from(c: char) -> Self {
        // Use Acii to transform to numbers
        match c {
            'a'..='z' => Self(u32::from(c) - 96),
            'A'..='Z' => Self(u32::from(c) - 38),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_correctly() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(rucksack.first_compartment(), "vJrwpWtwJgWr");
        assert_eq!(rucksack.second_compartment(), "hcsFMMfFFhFp")
    }
    #[test]
    fn finds_duplicate() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(rucksack.duplicate_item(), 'p');
    }
    #[test]
    fn priority() {
        assert_eq!(1, Priority::from('a').0);
        assert_eq!(26, Priority::from('z').0);
        assert_eq!(27, Priority::from('A').0);
        assert_eq!(52, Priority::from('Z').0);
    }
    #[test]
    fn group_priority() {
        let group_1 = RucksackGroup::new(vec![
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::new("PmmdzqPrVvPwwTWBwg"),
        ]);
        assert_eq!(Priority::from(group_1.badge()).0, 18);
    }
}
