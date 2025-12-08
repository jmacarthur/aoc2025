use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input6.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut line_iterator = contents.lines().into_iter();
    let operators = vec!["+", "*"];
    let mut operator_line: Vec<&str> = vec![];
    let mut number_lines: Vec<Vec<u64>> = vec![];
    for line in line_iterator {
        let trimmed_line = line.trim();
	let tokens : Vec<&str> = trimmed_line.split_whitespace().collect();
	println!("{:?}", tokens);
	if operators.contains(&tokens[0]) {
	   println!("This is the operator line!");
	   operator_line = tokens;
	} else {
	  let number_line: Vec<u64> = tokens.into_iter().map(|x| match x.parse() {
	  Ok(i) => i,
	  Err(_) => {panic!("Unparseable number: {x}");}
	  }).collect();
	  number_lines.push(number_line);
	}
    }
    println!("{:?}", number_lines);
    let mut total = 0;
    for i in 0..operator_line.len() {
    	let column_total = match operator_line[i] {
	     "+" => number_lines.clone().into_iter().fold(0, |acc, x| acc + x[i]),
	     "*" => number_lines.clone().into_iter().fold(1, |acc, x| acc * x[i]),
	     _ => { panic!("Unknown operator {}", operator_line[i]); }	     
	};
	println!("{column_total}");
	total += column_total;
    }
    println!("{total}");
    Ok(())
}
