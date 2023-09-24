use regex::Regex;
use std::io::{Read,self};
use std::fs::File;
use std::env;
use std::path;

#[derive(Clone,Debug,PartialEq)]
pub enum TokenType {
	NEWLINE,
	WHITESPACE,
	
	CLASS,
	DEF,

	//Operators
	EQUALS,

	NUMBER,
	WORD,
	STRING,

	QUOTATION
}

pub struct TokenRegex <'a> {
	pub _type:&'a TokenType,
	pub regex_pattern:Regex
}


#[derive(Clone,Debug,PartialEq)]
pub struct Token<'a> {
	pub _type:&'a TokenType,
	pub position:Vec<usize>
}

#[derive(Debug)]
pub struct BinaryExpr<'a> {
	pub left_expr:String,
	pub operator:&'a TokenType,
	pub right_expr:String
}

#[derive(Debug)]
pub enum LiteralType<'a> {
	STRINGLTR(&'a str),
	NUMBERLTR(i32)
}




pub fn file_to_string(input_str:&str) -> Result<String, io::Error>{
	//TODO handle the error
	let current_dir: path::PathBuf = env::current_dir()?;
	let path = current_dir.join(&input_str);
	let mut file_contents: String = String::new();
	let mut file: File = File::open(path)?;
	file.read_to_string(&mut file_contents)?;
	return Ok(file_contents);
}