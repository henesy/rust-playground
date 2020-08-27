use std::io;
use std::io::BufRead;
use std::env;
use std::process;
use std::sync::Once;

fn usage() {
	eprintln!("usage: wc [-rblw]");
	process::exit(1);
}

// wc(1) utility 
// Supports counting runes, bytes, lines, and words
fn main() {
	static START: Once = Once::new();
	let mut r = true;		// Count runes
	let mut b = true;		// Count bytes
	let mut l = true;		// Count lines
	let mut w = true;		// Count words

	/* Parse commandline arguments */ 

	let args = env::args();
	for arg in args {
		match arg.starts_with("-") {
			true => {
				// Invalid flag, this is just `-`
				if arg.len() < 2 {
					usage();
				}

				// This is a valid flag format

				// If we're using flags, don't count anything not specified
				START.call_once(|| {
					// Runs only once
					r = false;
					b = false;
					l = false;
					w = false;
				});

				let mut runes = arg.chars();

				// Discard the `-`
				runes.next();

				// Match flags
				for rune in runes {
					match rune {	
						'r' => r = true,
						'b' => b = true,
						'l' => l = true,
						'w' => w = true,

						_ => usage(),
					}
				}
			},
			false => {
				// This is not a flag - an input file instead

			},
		}
	}

	let mut runes = 0;
	let mut bytes = 0;
	let mut lines = 0;
	let mut words = 0;
	let delim = b'\n';		// Line delimiter

	/* Count from input(s) */

	let stdin = io::stdin();
	let mut br = stdin.lock();
	loop {
		let mut buf = vec![];
		match br.read_until(delim, &mut buf) {
			Ok(0) =>
				// EOF
				break,
			Ok(n) => {
				// Operate on line
				lines += 1;
				bytes += n;
				
				match String::from_utf8(buf) {
					Ok(s) => {
						runes += s.chars().count();
						words += s.split_ascii_whitespace().count();
					}

					Err(e) => panic!("err: could not read string: {}", e),
				}
			},
			Err(e) => panic!("err: could not read line: {}", e),
		}
	}

	// Emit counts
	if l {
		println!("lines: {}", lines);
	}
	if w {
		println!("words: {}", words);
	}
	if r {
		println!("runes: {}", runes);
	}
	if b {
		println!("bytes: {}", bytes);
	}
}

