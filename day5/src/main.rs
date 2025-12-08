use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

fn parse_field(iter: Option<&str>) -> u64 {
    let s = match iter {
        Some(field_string) => field_string,
        None => { panic!("Field expected, none found"); }
    };
    match s.parse() {
        Ok(i) => i,
        Err(_) => { panic!("Unparseable number: {:?}",s); }
    }
}            

fn is_inrange(val: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (lower, higher) in ranges {
        if val >= *lower && val <= *higher {
            return true;
        }
    }
    false
}

fn reduce_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_ranges: Vec<(u64, u64)> = vec![];
    // Collapse ranges
    let mut index = 0;
    loop {
        if index >= ranges.len()-1 {
            if index < ranges.len() {
                new_ranges.push(ranges[index]);
            }
            break;
        }
        let (l1, h1) = ranges[index];
        let (l2, h2) = ranges[index+1];
        if l2 <= h1 && l2 >= l1 {
           index += 1;
           new_ranges.push((l1, max(h1, h2)));
        } else {
           new_ranges.push((l1, h1));
        }
        index += 1;
    }
    new_ranges
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input5.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut line_iterator = contents.lines().into_iter();
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];
    let mut processing_ranges: bool = true;

    for line in line_iterator {
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            processing_ranges = false;
            continue;
        }
        if processing_ranges {
            let mut fields = trimmed_line.split("-");
            let lower = parse_field(fields.next());
            let higher = parse_field(fields.next());
            println!("Range: {lower}-{higher}");
            ranges.push((lower, higher));

        } else {            
            println!("Ingredient: {line}");
            ingredients.push(match trimmed_line.parse() {
                Ok(i) => i,
                Err(_) => { panic!("Unparseable number: {:?}",trimmed_line); }
            });
        }        
    }

    let mut new_ranges = ranges;
    // Sort ranges
    new_ranges.sort_by(|(l1, h1), (l2, h2)| l1.cmp(&l2));

    loop {
        let old_len = new_ranges.len();
        new_ranges = reduce_ranges(&mut new_ranges);
        if new_ranges.len() == old_len {
            break;
        } else {
            println!("Reduced ranges by {}", old_len - new_ranges.len());
        }
    }

    println!("Final ranges: {:?}", new_ranges);
    let mut fresh_count = 0;
    for val in ingredients {
        if is_inrange(val, &new_ranges) {
            fresh_count += 1;
        }
    };

    let mut possible_fresh_count = 0;
    for (lower, higher) in new_ranges {
        possible_fresh_count += (higher - lower + 1);
    }


    println!("In total, {fresh_count} ingredients are fresh.");
    println!("{possible_fresh_count} ingredients could be fresh.");

    Ok(())
}
