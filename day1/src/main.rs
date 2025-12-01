use std::fs::File;
use std::io::prelude::*;

fn parse_token(text: &str) -> i32 {
    let absval: i32 = match text[1..].parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable numeric field {}", text);
        }
    };

    match text.chars().next() {
        Some('L') => -absval,
        Some('R') => absval,
        Some(_) => {
            panic!("Unreadable field {}", text);
        }
        None => {
            panic!("No char?");
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let token_iterator = contents.split_whitespace();
    let mut list1 = Vec::new();

    for s in token_iterator {
        list1.push(parse_token(s));
    }

    let mut position: i32 = 50;
    let mut zero_hits: i32 = 0;
    for i in list1.iter() {
        position = (position + i) % 100;
        if position == 0 {
            zero_hits += 1;
        }
    }
    println!("Zero passes (part 1): {}", zero_hits);
    position = 50;
    zero_hits = 0;
    for i in list1.iter() {
        let direction = i.signum();
        let mut amount = i.abs();
        while amount > 0 {
            position = (position + direction) % 100;
            if position == 0 {
                zero_hits += 1;
            }
            amount -= 1;
        }
    }
    println!("Zero passes (part 2): {}", zero_hits);
    println!("Hello, world!");
    Ok(())
}
