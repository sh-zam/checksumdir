use std::fs;
use std::io::Read;
use blake2::{Blake2b, Digest};
use walkdir::WalkDir;
use base64;

pub fn traverse(dir_name: &str) {
	let mut hasher = Blake2b::new();
	let mut content = String::new();

	for entry in WalkDir::new(dir_name) {
		let entry = entry.unwrap();
		let file_path = entry.path();
		if file_path.is_dir() {
			continue;
		}
		fs::File::open(file_path.to_string_lossy().to_string())
			.unwrap()
			.read_to_string(&mut content)
			.unwrap();
		hasher.input(&content[..]);
		content.clear();
	}
	let digest = hasher.result();
	println!("{}",base64::encode(&digest));
}

#[cfg(test)]
mod tests {
    #[test]
    fn demo_traverse(){
		super::traverse("src");
	}
}