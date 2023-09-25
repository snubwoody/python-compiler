use regex::Regex;
use std::io::{Read,self};
use std::fs::File;
use std::env;
use std::path;
use RegisterValue::*;
use std::ops::Range;


#[derive(Clone,Debug,PartialEq)]
pub enum TokenType {
	NEWLINE,
	WHITESPACE,
	
	CLASS,
	DEF,

	//Operators
	EQUALS,
	MULTIPY,
	DIVIDE,
	ADD,
	SUBTRACT,

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

#[derive(PartialEq, Eq,Debug)]
pub enum RegisterValue{
	RINT(i32),
	RSTRING(String)
}
#[derive(PartialEq, Eq)]
pub struct Register{
	pub value:Option<RegisterValue>
}

//TODO theres A LOT of potential undefined behaviour here FIX IT
impl Register {
	pub fn new() -> Self{
		Register { value:None }
	}

	pub fn get_int(&self) -> Option<&i32> {
		match &self.value {
			Some(RINT(int)) => {
				Some(int)
			},
			Some(RSTRING(_)) => {
				None
			},
			None => {
				println!("reg is empty");
				None
			}
		}
	}

	pub fn get_string(&self) -> Option<&String> {
		match &self.value {
			Some(RINT(_)) => {
				None
			},
			Some(RSTRING(string)) => {
				Some(string)
			},
			None => {
				println!("reg is empty");
				None
			}
		}
	}

	pub fn mov(&mut self,value:RegisterValue) {
		self.value = Option::Some(value);
	}

	pub fn add(&mut self,value:i32) {
		self.value = Option::Some(RINT(self.get_int().unwrap()+value));
	}
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

pub fn get_string(string:&String,range:Range<usize>) -> String{
	let mut word = String::new();
	let range = range;
		
	for i in range.into_iter(){
		word.push(string.chars().nth(i).unwrap())
	}
	return word;
}