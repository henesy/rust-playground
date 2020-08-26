use std::io;
use std::io::BufRead;

// wc(1) utility 
// Supports counting runes, bytes, lines, and words
fn main() {
	let mut runes = 0;
	let mut bytes = 0;
	let mut lines = 0;
	let mut words = 0;
	let delim = b'\n';

	let stdin = io::stdin();
	loop {
		let mut buf = vec![];
		match stdin.lock().read_until(delim, &mut buf) {
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

	println!("lines: {}", lines);
	println!("words: {}", words);
	println!("runes: {}", runes);
	println!("bytes: {}", bytes);

}

