use regex::Regex;
use DataType::*;

//TODO Without spaces binary expressions are tokenized as a word instead so FIX that
//TODO letters are not parsed as identifiers

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
	TIDENTIFIER,

	QUOTATION,
	GREATERTHAN,
	OPENPARENTHESIS,
	CLOSEPARENTHESIS
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
pub enum DataType<'a> {
	INT(i32),
	STRING(String),
	BOOL(bool),
	IDENTIFIER(String),
	PLACEHOLDER(&'a str),
	NONE,
	DANGLINGEXPR
}

impl<'a> DataType<'a> {
    pub fn get_int(&self) -> Option<i32> {
        match self {
            INT(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            STRING(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            BOOL(value) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum StackError {
	AddressEmpty
}

#[derive(Debug,Clone,PartialEq)]
pub enum Expression<'a>{
	BinaryExpression(Box<Expression<'a>>,TokenType,Box<Expression<'a>>),
	LiteralExpression(DataType<'a>),
	GroupingExpression(Box<Vec<Expression<'a>>>)
}

#[derive(PartialEq, Eq,Clone)]
pub struct Register<'a>{
	pub value:Option<DataType<'a>>
}

//TODO theres A LOT of potential undefined behaviour here FIX IT
impl<'a> Register<'a> {
	pub fn new() -> Self{
		Register { value:None }
	}

	pub fn mov(&'a mut self,value:DataType<'a>) {
		self.value = Option::Some(value);
	}

	pub fn add(&mut self,value:i32) {
		let register = self.clone();
		self.value = Option::Some(INT(register.value.unwrap().get_int().unwrap() + value))
	}

	pub fn out(&self){
		let data = self.value.clone();
		
		match data {
			Some(INT(val)) => {
				println!("{:?}",val)
			},
			Some(STRING(val)) => {
				println!("{:?}",val)
			},
			Some(BOOL(val)) => {
				println!("{:?}",val)
			},
			_ => {
				//TODO change this to just return an error 
				println!("the register is empty")
			}
		}
	} 
	
}

#[derive(PartialEq,Debug)]
pub struct DataStack<'a> {
	pub data:Vec<DataType<'a>>
}

impl<'a> DataStack<'a> {
	pub fn new() -> Self {
		DataStack { data:Vec::new() }
	}

	pub fn _push(&'a mut self,value:DataType<'a>) {
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

