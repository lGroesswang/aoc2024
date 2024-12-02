#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let rows: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for row in rows {
        let vals: Vec<i32> = row.split_whitespace().map(|e| e.parse().unwrap()).collect();
        if is_valid(&vals) {
            sum += 1;
        }
    }

    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let rows: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for row in rows {
        let vals: Vec<i32> = row.split_whitespace().map(|e| e.parse().unwrap()).collect();
        if is_valid(&vals) || can_be_valid_by_removing_one(&vals) {
            sum += 1;
        }
    }

    sum
}

fn is_valid(vals: &[i32]) -> bool {
    if vals.windows(2).all(|w| (w[0] - w[1]).abs() <= 3 && (w[0] - w[1]).abs() != 0) {
        return true;
    }
    false
}

fn can_be_valid_by_removing_one(vals: &[i32]) -> bool {
    for i in 0..vals.len() {
        let mut z = vals.to_vec();
        z.remove(i);
        if is_valid(&z) {
            return true;
        }
    }
    false
}
