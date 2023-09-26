mod lexer;
mod parser;
mod utils;
use python_compiler::{
	DataType::*,
	Register,
	Token,
	DataStack,
	BinaryExpr
};
use utils::file_to_string;
use parser::parse_expressions;
use lexer::tokenize;



#[allow(unused_variables,unused_mut)]
fn main() {
	let file_tokens: Vec<Token<'_>>;
	let mut expressions:Vec<BinaryExpr<'_>> = Vec::new();
	let mut rg1 = Register::new();
	let mut rg2 = Register::new();
	let mut rg3 = Register::new();
	let mut rg4 = Register::new();

	let mut stack:DataStack = DataStack::new();
	
    match file_to_string("src/main.py") {

		Ok(file_contents) => {
			file_tokens = tokenize(&file_contents);
			expressions = parse_expressions(&file_tokens, &file_contents);

			file_tokens.iter().for_each(|token| println!("{:?}",token))
		}
		Err(error) => {
			println!("{}",error);
		}
	}

	
}






