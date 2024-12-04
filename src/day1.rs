#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut l1: Vec<usize> = vec![];
    let mut l2: Vec<usize> = vec![];
    let mut i = 0;

    for line in input.lines() {
        let a =line.split(' ').collect::<Vec<&str>>();
        l1.push(a[0].parse().unwrap());
        l2.push(a[3].parse().unwrap());

        i+=1;
    }
    l1.sort();
    l2.sort();

    let mut sum = 0;
    for i in 0..l1.len() {
        sum += l1[i].abs_diff(l2[i]);
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut l1: Vec<usize> = vec![];
    let mut l2: Vec<usize> = vec![];
    let mut i = 0;
    for line in input.lines() {
        let a =line.split(' ').collect::<Vec<&str>>();
        l1.push(a[0].parse().unwrap());
        l2.push(a[3].parse().unwrap());

        i+=1;
    }
    l2.sort_by(|a, b| a.cmp(&b));

    let mut sum = 0;
    for i in 0..l1.len() {
        sum += l1[i] * l2.iter().filter(|e| **e == l1[i]).count();
    }

    sum
}