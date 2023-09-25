use regex::Regex;
use std::io::{Read,self};
use std::fs::File;
use std::env;
use std::path;
use DataType::*;
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
	TSTRING,

	QUOTATION,
	GREATERTHAN
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

#[derive(PartialEq, Eq,Debug,Clone)]
pub enum DataType {
	INT(i32),
	STRING(String),
	BOOL(bool)
}

pub enum StackError {
	AddressEmpty
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

#[derive(PartialEq, Eq)]
pub struct Register{
	pub value:Option<DataType>
}

//TODO theres A LOT of potential undefined behaviour here FIX IT
impl Register {
	pub fn new() -> Self{
		Register { value:None }
	}

	pub fn get_int(&self) -> Option<&i32> {
		match &self.value {
			Some(INT(int)) => {
				Some(int)
			},
			None => {
				println!("reg is empty");
				None
			},
			_ => {
				None
			}
		}
	}

	pub fn mov(&mut self,value:DataType) {
		self.value = Option::Some(value);
	}

	pub fn add(&mut self,value:i32) {
		self.value = Option::Some(INT(self.get_int().unwrap()+value));
	}
}

#[derive(PartialEq,Debug)]
pub struct DataStack<'a> {
	pub data:Vec<&'a DataType>
}

impl<'a> DataStack<'a>  {
	pub fn new() -> Self {
		DataStack { data:Vec::new() }
	}

	pub fn _push(&'a mut self,value:&'a DataType) {
		self.data.push(value);
		
	}
	
	pub fn get(&self,address:usize) -> Result<&DataType,StackError> {
		let stack = &self.data;
		if address < stack.len(){
			Ok(stack[address])
		}
		else {
			println!("Error: Stack address is empty");
			Err(StackError::AddressEmpty)
		}
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