mod lexer;
mod parser;
use python_compiler::file_to_string;
use python_compiler::{
	RegisterValue::*,
	Register,
	Token
};
use parser::parse_expressions;
use lexer::tokenize;



#[allow(unused_variables,unused_mut)]
fn main() {
	let file_tokens: Vec<Token<'_>>;
	let mut rg1 = Register::new();
	let mut rg2 = Register::new();
	let mut rg3 = Register::new();
	let mut rg4 = Register::new();

	rg1.mov(RINT(10));
	assert_eq!(rg1.get_int().unwrap(),&10);

	rg1.add(10);
	assert_eq!(rg1.get_int().unwrap(),&20);


    match file_to_string("src/main.py") {

		Ok(file_contents) => {
			file_tokens = tokenize(&file_contents);
			
			parse_expressions(&file_tokens, &file_contents);
			file_tokens.iter().for_each(|token| println!("{:?}",token))			
		}
		Err(error) => {
			println!("{}",error);
		}
	}
	
}

//TODO generate expressions





