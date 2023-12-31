use std::io::{Read,self};
use std::fs::File;
use std::env;
use std::path;
use std::ops::Range;

pub fn file_to_string(input_str:&str) -> Result<String, io::Error>{
	//TODO handle the error
	let current_dir: path::PathBuf = env::current_dir()?;
	let path = current_dir.join(&input_str);
	let mut file_contents: String = String::new();
	let mut file: File = File::open(path)?;
	file.read_to_string(&mut file_contents)?;
	return Ok(file_contents);
}

pub fn get_string(string:&String,range:Range<usize>) -> String{
	let mut word = String::new();
	let range = range;
		
	for i in range.into_iter(){word.push(string.chars().nth(i).unwrap())}
	word = String::from(word.as_str().trim_matches('"'));
	
	return word;
}

pub fn is_in_range(range_outer:Range<usize>,range_inner:Range<usize>) -> bool{
	if range_outer.start <= range_inner.start && range_outer.end >= range_inner.end {
		return true;
	}
	return false;
}