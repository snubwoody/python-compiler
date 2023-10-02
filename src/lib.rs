use regex::Regex;
use DataType::*;


//TODO refactor this to only be an enum not a whole struct
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
pub enum DataType {
	INT(i32),
	STRING(String),
	BOOL(bool),
	IDENTIFIER(String),
	NONE,
	DANGLINGEXPR
}

impl DataType {
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
pub enum Expression{
	BinaryExpression(BinaryExpr),
	LiteralExpression(DataType)
}

#[derive(Debug,Clone,PartialEq)]
pub struct BinaryExpr{
	pub left_expr:Box<Expression>,
	pub operator:TokenType,
	pub right_expr:Box<Expression>
}


#[derive(PartialEq, Eq,Clone)]
pub struct Register{
	pub value:Option<DataType>
}

//TODO theres A LOT of potential undefined behaviour here FIX IT
impl Register {
	pub fn new() -> Self{
		Register { value:None }
	}

	pub fn mov(&mut self,value:DataType) {
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

