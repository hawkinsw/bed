use std::io;
use std::u8;
use std::u64;
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn read_file<T: Read>(fd: &mut T) -> io::Result<Vec<u8>> {
	let mut contents = Vec::<u8>::new();

	match fd.read_to_end(&mut contents) {
		Ok(s) => {
			println!("Read {} bytes.", s);
			Ok(contents)
		}
		Err(e) => {
			println!("Read failure: {}", e);
			Err(e)
		}
	}
}

fn print_usage() {
}

fn main() -> std::io::Result<()> {
	let mut args: Vec<String> = env::args().collect();
	let replace: u64;
	let mut replacement: Vec<u8> = Vec::<u8>::new();

	if args.len() <= 2 {
		print_usage();
		return Err(io::Error::new(io::ErrorKind::Other,
		                          "Not enough parameters."));
	}

	replace = match u64::from_str_radix(&args[1], 16) {
		Ok(v) => v,
		Err(e) => return Err(io::Error::new(io::ErrorKind::Other,
		                          "Error parsing parameter."))
	};

	args = args[2..].to_vec();

	for r in args.iter() {
		let rv: u8 = match u8::from_str_radix(r, 16) {
			Ok(v) => v,
			Err(e) => return Err(io::Error::new(io::ErrorKind::Other,
		                          "Error parsing parameter."))
		};
		println!("Pushing {}", rv);
		replacement.push(rv);
	}

	println!("Replace {} with ...", replace);
	let mut fd = io::stdin();
	let mut contents = read_file(&mut fd).unwrap();

	println!("Old:");
	for i in contents.iter() {
		println!("{}", i);
	}

	for i in (replace .. replace + (replacement.len() as u64)) {
		contents[i as usize] = replacement.pop().unwrap();
		println!("i: {}", i);
	}

	println!("New:");
	for i in contents.iter() {
		println!("{}", i);
	}

	/*
	  Closing _fd_ is not necessary because io::File's are
		implicitly closed when they go out of scope.
	 */
	Ok(())
}
