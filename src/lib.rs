use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::path::Path;
use std::collections::HashSet;

use walkdir::{WalkDir, DirEntry};
use blake2::{Blake2b, Digest};
use base64;


/// Computes deterministic hash of a directory.
/// ```no_run
/// use checksumdir;
/// 
/// println!("{}", checksumdir::checksumdir("test-checksum").unwrap());
/// ```
pub fn checksumdir(dir_path: &str) -> Result<String> {
	Ok(digested(compute(dir_path, ChecksumOptions::default())?))
}

pub fn checksumdir_with_options(
		dir_path: &str, 
		opts: ChecksumOptions) 
		-> Result<String> 
{
	Ok(digested(compute(dir_path, opts)?))
}


pub struct ChecksumOptions<'a> {
	excluded: HashSet<&'a str>,
	ignore_hidden: bool,
	follow_symlinks: bool,
}

impl<'a> Default for ChecksumOptions<'a> {
	fn default() -> ChecksumOptions<'a> {
		ChecksumOptions {
			excluded: HashSet::new(),
			ignore_hidden: false,
			follow_symlinks: false,
		}
	}
}

impl<'a> ChecksumOptions<'a> {
	pub fn new(
		excluded_dirs: Vec<&'a str>,
		ignore_hidden: bool, 
		follow_symlinks: bool
		) -> Self 
	{
		let mut set = HashSet::new();
		for i in excluded_dirs {
			set.insert(i);
		}
		ChecksumOptions {
			excluded: set,
			ignore_hidden,
			follow_symlinks
		}
	}
}

fn compute(dir_path: &str, opts: ChecksumOptions) -> Result<Blake2b> {
	let mut hasher = Blake2b::new();

	let it = WalkDir::new(dir_path).follow_links(opts.follow_symlinks)
			.into_iter()
			.filter_entry(|e| {
				!(opts.ignore_hidden && is_hidden(e)) &&
				!is_in_list(&opts.excluded, e)
			})
			.filter_map(|e| e.ok())
			.filter(|e| !e.file_type().is_dir());
	
	for entry in it {
		let file_path = entry.path();
		hasher = file_hash(file_path, hasher)?;
	}

	Ok(hasher)
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

fn is_hidden(entry: &DirEntry) -> bool { 
	entry.file_name()
		.to_str()
		.map(|s| s.starts_with("."))
		.unwrap_or(false)
}

fn is_in_list<'a>(list: &HashSet<&'a str>, entry: &DirEntry) -> bool {
	list.contains(&entry.file_name().to_str().unwrap_or(""))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_checksumdir(){
		assert_eq!(checksumdir("test-checksum").unwrap(),
		 "VE1qUJBahjtfiDDTQ8uuVvl5ogEZ9q0fhQVNWKgX3Ry1KUjBm30n8OOB6cz8Y6Ut/THi9Ix7LmZ8e7ho4hRkug==");
	}

	#[test]
	fn ignore_hidden() {
		let opts = ChecksumOptions::new(vec![""], true, false);
		assert_eq!(checksumdir_with_options("test-checksum", opts).unwrap(), 
		 "mupKycbw2LJSCieIPeOJp6NTHQY0gcbcFXIxUczmrscNcb+iqW1FCxMj7dpzYCj+UsvoXGmqLhYiBvhrgwlsyQ==");
	}

	#[test]
	fn excluded_names() {
		let my_vec = vec![".foo", ".shhh"];
		let opts = ChecksumOptions::new(my_vec, false, false);
		assert_eq!(checksumdir_with_options("test-checksum", opts).unwrap(), 
		 "mupKycbw2LJSCieIPeOJp6NTHQY0gcbcFXIxUczmrscNcb+iqW1FCxMj7dpzYCj+UsvoXGmqLhYiBvhrgwlsyQ==");
	}
}
