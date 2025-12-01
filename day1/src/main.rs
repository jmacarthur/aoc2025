use std::fs::File;
use std::io::prelude::*;

fn parse_token(text: &str) -> i32 {
   let absval: i32 = match text[1..].parse() {
     Ok(i) => i,
     Err(_) => { panic!("Unreadable numeric field {}", text); }
   };
 
   match text.chars().nth(0) {
     Some('L') => -absval,
     Some('R') => absval,
     Some(_) => { panic!("Unreadable field {}", text); },
     None  => { panic!("No char?"); }
   }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut token_iterator = contents.split_whitespace();
    let mut list1 = Vec::new();

    while let Some(s) = token_iterator.next() {
    	  list1.push(parse_token(s));
    }

    println!("{:?}", list1);
    println!("Hello, world!");
    Ok(())
}
