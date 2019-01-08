use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::path::Path;
use walkdir::WalkDir;

use blake2::{Blake2b, Digest};
use base64;

/// Computes deterministic hash of a directory.
/// ```no_run
/// use checksumdir;
/// 
/// assert_eq!("{}", checksumdir::dir_hash("test-checksum"));
/// ```
pub fn dir_hash(dir_name: &str) -> Result<String> {
	let mut hasher = Blake2b::new();

	for entry in WalkDir::new(dir_name) {
		let entry = entry?;
		let file_path = entry.path();
		if file_path.is_dir() {
			continue;
		}
		hasher = file_hash(file_path, hasher)?;
	}
	Ok(digested(hasher))
}

fn file_hash(file_path: &Path, mut hasher: Blake2b) -> Result<Blake2b> {
	let file = File::open(file_path)?;
	let mut reader = BufReader::new(file);
	loop {
		let length = {
			let buffer = reader.fill_buf()?;
			hasher.input(buffer);
			buffer.len()
		};
		if length == 0 { break; }
		reader.consume(length);
	}
	Ok(hasher)
}

fn digested(hasher: Blake2b) -> String {
	let digest = hasher.result();
	base64::encode(&digest)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dir_hash(){
		assert_eq!(super::dir_hash("test-checksum").unwrap(),
		 "mupKycbw2LJSCieIPeOJp6NTHQY0gcbcFXIxUczmrscNcb+iqW1FCxMj7dpzYCj+UsvoXGmqLhYiBvhrgwlsyQ==");
	}
}