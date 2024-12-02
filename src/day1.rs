use hashbrown::{hash_map::Entry, HashMap};

pub fn part1(input: &str) -> u64 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    input.lines().for_each(|line| {
        let mut it = line.split_whitespace();
        let l: i64 = it.next().unwrap().parse().unwrap();
        left.push(l);
        let r: i64 = it.next().unwrap().parse().unwrap();
        right.push(r);
    });

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).unsigned_abs())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut left = Vec::new();
    let mut right = HashMap::new();
    input.lines().for_each(|line| {
        let mut it = line.split_whitespace();
        let l: u64 = it.next().unwrap().parse().unwrap();
        left.push(l);
        let r: u64 = it.next().unwrap().parse().unwrap();
        *right.entry(r).or_insert(0_u64) += 1;
    });
    left.iter()
        .map(|n| match right.entry(*n) {
            Entry::Occupied(e) => n * e.get(),
            Entry::Vacant(_) => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::get_input;

    const DAY: i16 = 1;

    #[test]
    fn part1_test() {
        let input = get_input(DAY).unwrap();
        println!("Answer for day {DAY} part 1: {}", part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = get_input(DAY).unwrap();
        println!("Answer for day {DAY} part 2: {}", part2(&input));
    }
}
