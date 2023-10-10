use std::{env, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	let file = fs::read_to_string(&args[1]).expect("UNABLE TO READ FILE");

	let mut elves: Vec<Vec<u64>> = Vec::new();
	let mut elf: Vec<u64> = Vec::new();

	for elem in file.lines() {
		if elem == "" {
			elves.push(elf.clone());

			elf = Vec::new();
		} else {
			elf.push(elem.parse().unwrap());
		}
	}

	let mut heft = 0;
	let mut phattest = 0;
	for (idx, e) in elves.iter().enumerate() {
		let sum: u64 = e.iter().sum();

		if sum > heft {
			phattest = idx+1;
			heft = sum;
		}
	}

    let cal_count: u64 = elves[phattest-1].iter().sum();
	println!("The most Caloric Elf is: {phattest}");
    println!("\tCarrying {cal_count} Calories");
}
