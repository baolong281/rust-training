use std::env;
use std::process;
use rand::Rng;

fn main() {
	clear_screen();
	let game = Game::build(env::args()).unwrap_or_else(|e| {
		println!("Erorr {e}");
		process::exit(1);
	});
	game.render();
}

pub struct Game {
	size: usize,
	cells: Vec<Vec<u8>>,
}

impl Game {
	pub fn build(mut args: impl Iterator<Item=String>) -> Result<Game, &'static str>{
		args.next();

		let size: usize= match args.next() {
			Some(arg) => match arg.parse::<usize>() {
				Ok(arg) => {
					if arg > 64 {
						return Err("Number too large");
					}
					arg
				}
				Err(e) => return Err("Enter valid numbers")
			}
			None => 32
		};

		let population: f32 = match args.next() {
			Some(arg) => match arg.parse::<f32>() {
				Ok(arg) => {
					if arg > 1.0 {
						return Err("Invalid population size");
					}
					arg
				}
				Err(e) => return Err("Enter valid numbers")
			}
			None => 0.30
		};


		let mut cells = vec![vec![0; size]; size];
		let mut rng = rand::thread_rng();

		for i in 0..size {
			for j in 0..size {
				cells[i][j] =  if rng.gen_range(0.0..1.0) > population {
					0
				} else {
					1
				};
			}
		}

		Ok(Game { size, cells })
	}

	pub fn render(&self) {
		for row in &self.cells {
			for val in row {
				if *val == 0 {
					print!("·");
				} else {
					print!("℮")
				}
				print!(" ");
			}
			println!();
		}
	}

	pub fn counting(arr: &Vec<Vec<u8>>, i: usize, j: usize) -> u8 {
		let pos = (i, j);
		let dirs = [(-1, -1), (-1, 0), (-1, 1),
		(0, -1),           (0, 1),
		(1, -1), (1, 0), (1, 1)]; 
		
		dirs
			.iter()
			.map(|dir| {
				let pos2 = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
				let pos2 = (pos2.0 as usize, pos2.1 as usize);
				match arr.get(pos2.0) {
					Some(row) => 
					match row.get(pos2.1)  {
						Some(val) => return val,
						None => return &0,
					}
					None => return &0,
				}

			})
			.map(|val| *val)
			.sum()
		}
	pub fn step(&self) {
		for i in 0..self.size {
			for j in 0..self.size {
				let count = &Game::counting(&self.cells, i, j);

			}
		}
	}

}

///clears the screen
fn clear_screen() {
	print!("\x1B[2J\x1B[1;1H");
}

#[cfg(test)]
mod tests {
	use super::*;

	pub fn counting(arr: &Vec<Vec<u8>>, i: usize, j: usize) -> u8 {
		let pos = (i, j);
		let dirs = [(-1, -1), (-1, 0), (-1, 1),
		(0, -1),           (0, 1),
		(1, -1), (1, 0), (1, 1)]; 
		
		dirs
			.iter()
			.map(|dir| {
				let pos2 = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
				let pos2 = (pos2.0 as usize, pos2.1 as usize);
				match arr.get(pos2.0) {
					Some(row) => 
					match row.get(pos2.1)  {
						Some(val) => return val,
						None => return &0,
					}
					None => return &0,
				}

			})
			.map(|val| *val)
			.sum()
		}


	#[test]
	fn test_one() {
		let arr: Vec<Vec<u8>> =
		vec![
			vec![0, 0, 1, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![0, 1, 0, 0, 0],
			vec![0, 0, 1, 0, 0],
		];
		assert_eq!(counting(&arr, 0, 0), 1);
	}

	#[test]
	fn test_two() {
		let arr: Vec<Vec<u8>> =
		vec![
			vec![0, 0, 1, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![0, 1, 0, 0, 0],
			vec![0, 0, 1, 0, 0],
		];

		assert_eq!(counting(&arr, 0, 3), 2);
	}

	#[test]
	fn test_three() {
		let arr: Vec<Vec<u8>> =
		vec![
			vec![0, 0, 1, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![0, 1, 0, 0, 0],
			vec![0, 0, 1, 0, 0],
		];
	
		assert_eq!(counting(&arr, 3, 1), 2);
	}

	#[test]
	fn test_four() {
		let arr: Vec<Vec<u8>> =
		vec![
			vec![0, 0, 1, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![1, 0, 0, 1, 0],
			vec![0, 1, 0, 0, 0],
			vec![0, 0, 1, 0, 0],
		];
	
		assert_eq!(counting(&arr, 1, 3), 3);
	}
}

