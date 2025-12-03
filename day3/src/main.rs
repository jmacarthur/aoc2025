use std::fs::File;
use std::io::prelude::*;

fn highest_first_digit(nums: &[char], start_position: usize, ignore_end: usize) -> (char, usize) {
    let mut maximum = '0';
    let mut max_index = 0;
    for (index, digit) in nums.iter().enumerate().take(nums.len() - ignore_end).skip(start_position) {
        if *digit > maximum {
            maximum = *digit;
            max_index = index;
        }
    }
    (maximum, max_index)
}

fn find_joltage(line: &str, fields: usize) -> i64 {
    let nums: Vec<char> = line.chars().collect();
    let mut digits: Vec<char> = vec![];
    let mut next_position = 0;
    for i in 0..fields {
        let (digit, position) = highest_first_digit(&nums, next_position, fields - 1 - i);
        next_position = position + 1;
        digits.push(digit);
    }
    let num_string = digits.into_iter().collect::<String>();
    match num_string.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unparseable value {}", num_string);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input3.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let token_iterator = contents.split_whitespace();
    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for s in token_iterator {
        total_part1 += find_joltage(s, 2);
        total_part2 += find_joltage(s, 12);
    }

    println!("Part 1 total: {}", total_part1);
    println!("Part 2 total: {}", total_part2);

    Ok(())
}
