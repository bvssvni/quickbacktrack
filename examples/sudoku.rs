/*

Sodoku is a puzzle where you fill numbers 1-9 until every row, column and group contains 1-9.

For more information, see https://en.wikipedia.org/wiki/Sudoku

An interesting thing with this example is the varying performance with different algorithms
for picking the next empty slot.

*/

extern crate quickbacktrack;

use quickbacktrack::{BackTrackSolver, Puzzle, SolveSettings};

#[derive(Clone)]
pub struct Sodoku {
	pub slots: [[u8; 9]; 9],
}

impl Puzzle for Sodoku {
	type Pos = [usize; 2];
	type Val = u8;

	fn solve_simple(&mut self) {
		loop {
			let mut found_any = false;
			for y in 0..9 {
				for x in 0..9 {
					if self.slots[y][x] != 0 { continue; }
					let possible = self.possible([x, y]);
					if possible.len() == 1 {
						self.slots[y][x] = possible[0];
						found_any = true;
					}
				}
			}
			if !found_any { break; }
		}
	}

	fn set(&mut self, pos: [usize; 2], val: u8) {
		self.slots[pos[1]][pos[0]] = val;
	}

	fn remove(&mut self, other: &Sodoku) {
		for y in 0..9 {
			for x in 0..9 {
				if other.slots[y][x] != 0 {
					self.slots[y][x] = 0;
				}
			}
		}
	}

	fn print(&self) {
		println!(" ___ ___ ___");
		for y in 0..9 {
			print!("|");
			for x in 0..9 {
				let v = self.slots[y][x];
				if v == 0 {
					print!(" ");
				} else {
					print!("{}", self.slots[y][x]);
				}
				if x % 3 == 2 {
					print!("|");
				}
			}
			println!("");
			if y % 3 == 2 {
				println!(" ---+---+---");
			}
		}
	}

	fn possible(&self, pos: [usize; 2]) -> Vec<u8> {
		let mut res = vec![];
		if self.slots[pos[1]][pos[0]] != 0 {
			res.push(self.slots[pos[1]][pos[0]]);
			return res;
		}
		'next_val: for v in 1..10 {
			for x in 0..9 {
				if self.slots[pos[1]][x] == v {
					continue 'next_val;
				}
				if self.slots[x][pos[0]] == v {
					continue 'next_val;
				}
			}
			let block_x = 3 * (pos[0] / 3);
			let block_y = 3 * (pos[1] / 3);
			for y in block_y..block_y + 3 {
				for x in block_x..block_x + 3 {
					if self.slots[y][x] == v {
						continue 'next_val;
					}
				}
			}
			res.push(v);
		}
		return res;
	}

	fn is_solved(&self) -> bool {
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 { return false; }
			}
		}
		return true;
	}
}

impl Sodoku {

	pub fn find_empty(&self) -> Option<[usize; 2]> {
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 {
					return Some([x, y]);
				}
			}
		}
		return None;
	}

	pub fn find_min_empty(&self) -> Option<[usize; 2]> {
		let mut min = None;
		let mut min_pos = None;
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 {
					let possible = self.possible([x, y]);
					if min.is_none() || min.unwrap() > possible.len() {
						min = Some(possible.len());
						min_pos = Some([x, y]);
					}
				}
			}
		}
		return min_pos;
	}

	pub fn find_freq_empty(&self) -> Option<[usize; 2]> {
		// Find the frequency of each numbers.
		let mut freq = [0; 9];
		let mut mask: [[u16; 9]; 9] = [[0; 9]; 9];
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 {
					let possible = self.possible([x, y]);
					for p in &possible {
						freq[(*p - 1) as usize] += 1;
						mask[y][x] |= 1 << (*p - 1);
					}
				}
			}
		}

		// Find the number with least frequency, but bigger than 0.
		let mut min_freq = None;
		for i in 0..9 {
			if freq[i] > 0 && (min_freq.is_none() || freq[i] < freq[min_freq.unwrap()]) {
				min_freq = Some(i);
			}
		}
		// println!("TEST {:?} {:?}", freq, min_freq);
		let min_freq = if let Some(i) = min_freq {
			i
		} else {
			return self.find_empty();
		};

		for y in 0..9 {
			for x in 0..9 {
				let bit = 1 << min_freq;
				if self.slots[y][x] == 0 && (mask[y][x] & bit == bit) {
					return Some([x, y]);
				}
			}
		}
		return self.find_empty();
	}
}

fn main() {
	let x = example2();
	x.print();

	let settings = SolveSettings::new()
		.solve_simple(false)
		.debug(true)
		.difference(true)
		.sleep_ms(500)
	;
	let solver = BackTrackSolver::new(x, settings);
	// Try `find_empty` and `find_freq_empty` for comparison.
	let difference = solver.solve(|s| s.find_min_empty())
		.expect("Expected solution");
	println!("Difference:");
	difference.print();
}

pub fn example1() -> Sodoku {
	Sodoku {
		slots: [
			[0, 4, 1, 0, 9, 0, 2, 0, 0],
			[9, 2, 6, 5, 0, 0, 1, 0, 0],
			[0, 0, 0, 1, 0, 0, 3, 0, 6],
			[6, 3, 0, 0, 4, 0, 0, 8, 9],
			[7, 0, 0, 0, 0, 0, 0, 0, 1],
			[1, 5, 0, 0, 8, 0, 0, 2, 7],
			[2, 0, 9, 0, 0, 7, 0, 0, 0],
			[0, 0, 5, 0, 0, 8, 9, 1, 2],
			[0, 0, 3, 0, 1, 0, 7, 5, 0],
		]
	}
}

pub fn example2() -> Sodoku {
	Sodoku {
		slots: [
			// [8, 3, 0, 0, 0, 0, 7, 0, 0],
			// [0, 0, 6, 0, 3, 4, 0, 2, 0],
			// [4, 7, 0, 9, 0, 0, 0, 6, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 3, 4, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],

			[9, 6, 0, 0, 5, 0, 0, 8, 7],
			[2, 0, 0, 0, 0, 0, 0, 0, 6],
			[7, 1, 0, 0, 2, 0, 0, 4, 5],

			[0, 2, 0, 0, 0, 9, 0, 7, 8],
			[0, 4, 0, 6, 1, 0, 5, 0, 0],
			[0, 0, 8, 0, 0, 0, 0, 1, 3],
		]
	}
}
