

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input.split("mul(")
        .skip(1)
        .filter_map(|part| part.split(")").next())
        .filter_map(|part| {
            let vals: Vec<&str> = part.split(',').collect();
            if vals.len() == 2 {
                if let (Ok(val1), Ok(val2)) = (vals[0].parse::<i32>(), vals[1].parse::<i32>()) {
                    return Some(val1 * val2);
                }
            }
            None
        })
        .sum::<i32>() as usize
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum = 0;
    let mut en = true;

    for line in input.split("mul(").skip(1) {
        let line = line.replace("don't()", "ß098763456784239").replace("do()", "gtjhgcnertzg4356tz");
        for part in line.split(")") {
            if part.contains("ß098763456784239") {
                en = false;
            } else if part.contains("gtjhgcnertzg4356tz") {
                en = true;
            } else if en {
                let vals: Vec<&str> = part.split(',').collect();
                if vals.len() == 2 {
                    if let (Ok(val1), Ok(val2)) = (vals[0].parse::<i32>(), vals[1].parse::<i32>()) {
                        sum += val1 * val2;
                    }
                }
            }
        }
    }

    sum as usize
}


/*
#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut sum= 0;

    let a: Vec<&str> = input.split("mul(").collect();
    let b: Vec<Vec<&str>> = a.iter().map(|e| e.split(")").collect()).collect();

    for e in b {
        for f in e {
            let vals: Vec<&str> = f.split(",").collect();
            if vals.len() == 2 && vals[0].parse::<i32>().is_ok() && vals[1].parse::<i32>().is_ok() {
                sum += vals[0].parse::<i32>().unwrap() * vals[1].parse::<i32>().unwrap();
            }
        }
    }

    sum as usize
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum= 0;

    let a: Vec<&str> = input.split("mul(").collect();
    let b: Vec<String> = a.iter().map(|e| e.replace("don't()", "ß098763456784239")).collect();
    let h: Vec<String> = b.iter().map(|e| e.replace("do()", "gtjhgcnertzg4356tz")).collect();
    let c: Vec<Vec<&str>> = h.iter().map(|e| e.split(")").collect()).collect();

    let mut en = true;
    
    for e in c {
        for f in e {
            if f.contains("ß098763456784239") {
                en = false;
            }
            if f.contains("gtjhgcnertzg4356tz") {
                en = true;
            }
            let vals: Vec<&str> = f.split(",").collect();
            if en && vals.len() == 2 && vals[0].parse::<i32>().is_ok() && vals[1].parse::<i32>().is_ok() {
                sum += vals[0].parse::<i32>().unwrap() * vals[1].parse::<i32>().unwrap();
            }
        }
    }

    sum as usize
} */