use fancy_regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn iterator_to_i64(iterator: &mut std::str::Split<char>) -> i64 {
    let strval = match iterator.next() {
        Some(s) => s,
        None => {
            panic!("Iterator had no values");
        }
    };
    match strval.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unparseable value {}", strval);
        }
    }
}

fn scan_range(range_array: &Vec<&str>, re: &Regex) -> i64 {
    let mut total = 0;
    for range_unclean in range_array {
        let range = range_unclean.trim();
        let mut lower_upper_iterator = range.split('-');
        let lower: i64 = iterator_to_i64(&mut lower_upper_iterator);
        let higher: i64 = iterator_to_i64(&mut lower_upper_iterator);
        println!("Scanning range {} to {}", lower, higher);
        for i in lower..=higher {
            if re.is_match(&i.to_string()).unwrap() {
                println!("{}", i);
                total += i;
            }
        }
    }
    total
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let range_array: Vec<&str> = contents.split(',').collect();

    let re_part1 = Regex::new(r"^([1-9][0-9]*)(\1)$").unwrap();
    let re_part2 = Regex::new(r"^([1-9][0-9]*)(\1)+$").unwrap();

    let total_part1 = scan_range(&range_array, &re_part1);
    let total_part2 = scan_range(&range_array, &re_part2);

    println!("Total for part 1 is: {}", total_part1);
    println!("Total for part 2 is: {}", total_part2);

    Ok(())
}
