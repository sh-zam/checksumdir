use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use walkdir::WalkDir;

use blake2::{Blake2b, Digest};
use base64;

/// Computes deterministic hash of a directory.
/// ```no_run
/// use sumdir;
/// 
/// assert_eq!("{}", sumdir::dir_hash("test-checksum"));
/// ```
pub fn dir_hash(dir_name: &str) -> String {
	let mut hasher = Blake2b::new();

	for entry in WalkDir::new(dir_name) {
		let entry = entry.unwrap();
		let file_path = entry.path();
		if file_path.is_dir() {
			continue;
		}
		hasher = file_hash(file_path, hasher);
	}
	digested(hasher)
}

fn file_hash(file_path: &Path, mut hasher: Blake2b) -> Blake2b {
	let file = File::open(file_path)
		.expect("File could not be opened");
	let mut reader = BufReader::new(file);
	loop {
		let length = {
			let buffer = reader.fill_buf().unwrap();
			hasher.input(buffer);
			buffer.len()
		};
		if length == 0 { break; }
		reader.consume(length);
	}
	hasher
}

fn digested(hasher: Blake2b) -> String {
	let digest = hasher.result();
	base64::encode(&digest)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dir_hash(){
		assert_eq!(super::dir_hash("test-checksum"),
		 "mupKycbw2LJSCieIPeOJp6NTHQY0gcbcFXIxUczmrscNcb+iqW1FCxMj7dpzYCj+UsvoXGmqLhYiBvhrgwlsyQ==");
	}
}