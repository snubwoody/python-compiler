mod lexer;
mod parser;
use python_compiler::{file_to_string, BinaryExpr};
use python_compiler::{
	DataType::*,
	Register,
	Token,
	TokenType::*,
	DataStack,
};
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

	let mut stack:DataStack<'_> = DataStack::new();
	
	stack._push(&INT(1));
	stack.get(0);

    match file_to_string("src/main.py") {

		Ok(file_contents) => {
			file_tokens = tokenize(&file_contents);
			expressions = parse_expressions(&file_tokens, &file_contents);
		}
		Err(error) => {
			println!("{}",error);
		}
	}

	expressions.iter().for_each(|expression|
		match expression.operator {
				&ADD => {
					rg2.mov(INT(expression.left_expr.parse::<i32>().unwrap()));
					rg2.add(expression.right_expr.parse::<i32>().unwrap());
				},
				_ => {
				}
		}
			
	);

	assert_eq!(rg2.get_int().unwrap(),&7);
	
	
}






