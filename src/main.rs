use std::io;
use std::u8;
use std::u64;
use std::env;
use std::io::prelude::*;
//use std::fs::File;

fn read_file<T: Read>(fd: &mut T) -> io::Result<Vec<u8>> {
	let mut contents = Vec::<u8>::new();

	match fd.read_to_end(&mut contents) {
		Ok(_) => {
			Ok(contents)
		}
		Err(e) => {
			Err(e)
		}
	}
}

fn write_file<T: Write>(fd: &mut T, output: &[u8]) -> io::Result<()> {
	fd.write_all(output)
}

fn print_usage(program_name: String) {
	println!("{} <replacement index> <replacement byte 1> ... <replacement byte n>", program_name);
}

fn main() -> std::io::Result<()> {
	let mut args: Vec<String> = env::args().collect();
	let replace: u64;
	let mut replacement: Vec<u8> = Vec::<u8>::new();
	let mut input: io::Stdin = io::stdin();
	let mut output: io::Stdout = io::stdout();
	let mut contents: Vec<u8>;

	if args.len() <= 2 {
		print_usage(args[0].clone());
		return Err(io::Error::new(io::ErrorKind::Other,
		                          "Not enough parameters."));
	}

	replace = match u64::from_str_radix(&args[1], 16) {
		Ok(v) => v,
		Err(_) => return Err(io::Error::new(io::ErrorKind::Other,
		                          "Error parsing replacement index parameter."))
	};

	/*
	  From now on we are only going to consider the replacement
		content parameters.
	 */
	args = args[2..].to_vec();

	for (i, r) in args.iter().enumerate() {
		let rv: u8 = match u8::from_str_radix(r, 16) {
			Ok(v) => v,
			Err(_) => return Err(io::Error::new(io::ErrorKind::Other,
		                       format!("Error parsing {}th replacement content parameter.", i+1)))
		};
		replacement.push(rv);
	}

	contents = read_file(&mut input).unwrap();

	/*
	  TODO: Make sure that the replacement fits within the content.
	 */

	for i in (replace .. replace + (replacement.len() as u64)) {
		contents[i as usize] = replacement.pop().unwrap();
	}

	write_file(&mut output, &contents)

	/*
	  Closing input and output is not necessary because io::File's 
		are implicitly closed when they go out of scope.
	 */
}
