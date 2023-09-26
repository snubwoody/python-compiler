use regex::Regex;
use DataType::*;


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

#[derive(Debug)]
pub enum StackError {
	AddressEmpty
}
#[derive(Debug)]
pub struct BinaryExpr<'a> {
	pub left_expr:String,
	pub operator:&'a TokenType,
	pub right_expr:String
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

	/*
		pub fn out(&self){
		match self.value {
			Some()
		}
	} 
	*/
}

#[derive(PartialEq,Debug)]
pub struct DataStack {
	pub data:Vec<DataType>
}

impl DataStack {
	pub fn new() -> Self {
		DataStack { data:Vec::new() }
	}

	pub fn _push(&mut self,value:DataType) {
		self.data.push(value);
		
	}
	
	pub fn get(&self,address:usize) -> Result<DataType,StackError> {
		let stack = &self.data;
		if address < stack.len(){
			Ok(stack[address].clone())
		}
		else {
			println!("Error: Stack address is empty");
			Err(StackError::AddressEmpty)
		}
	}

}
