use std::fs::File;
use std::io::prelude::*;

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            None
        } else {
            Some(
                self.rows[TryInto::<usize>::try_into(y).unwrap()]
                    [TryInto::<usize>::try_into(x).unwrap()],
            )
        }
    }
    fn set(&mut self, x: i32, y: i32, newsymbol: char) {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            return;
        }
        let row = TryInto::<usize>::try_into(y).unwrap();
        let col = TryInto::<usize>::try_into(x).unwrap();
        self.rows[row][col] = newsymbol;
    }
    fn width(&self) -> i32 {
        self.rows[0].len().try_into().unwrap()
    }
    fn height(&self) -> i32 {
        self.rows.len().try_into().unwrap()
    }
}



fn main() -> std::io::Result<()> {
    let mut file = File::open("input4.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let line_iterator = contents.split_whitespace();

    let mut grid: Vec<Vec<char>> = vec!();
    for s in line_iterator {
        grid.push(s.chars().collect());
    }

    println!("{:?}", grid);
    let gridwidth = grid[0].len();
    let gridheight = grid.len();
    println!("{gridwidth}, {gridheight}");
    let directions: Vec<(i32, i32)> = [ (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1), (1,1) ].to_vec();

    let mut grid2 = Grid { rows: grid };
    let mut total_removals = 0;
    loop {
	let mut removal_list: Vec<(i32, i32)> = vec![];
	for y in 0i32..grid2.height() {
	    for x  in 0i32..grid2.width() {
		if grid2.get(x,y) == Some('@') {
		    let mut neighbours = 0;
		    for (dx, dy) in &directions {
			let v = grid2.get((x+dx).try_into().unwrap(), (y+dy).try_into().unwrap());

			match v {
			      Some('@') => neighbours += 1,
			      _ => {}
			}

		    }
		    if neighbours < 4 {
		       removal_list.push((x, y));
		    }

		}
	    }
	}
	println!("Total moveable rolls: {}", removal_list.len());
	if removal_list.len() == 0 {
	break;
	}
	total_removals += removal_list.len();
	for (x,y) in removal_list {
	    grid2.set(x,y,'.');
	}
    }
    println!("Total removals: {total_removals}");
    Ok(())
}
