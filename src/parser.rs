use std::vec;
use python_compiler::{
	Token,
	BinaryExpr,
	TokenType::*,
	get_string
};


pub fn parse_expressions(tokens:&Vec<Token<'_>>,file:&String) -> Vec<BinaryExpr<'static>>{
	let mut expressions:Vec<BinaryExpr<'_>> = vec![];
	for (index,token) in tokens.iter().enumerate() {
		//println!("{:?}",size);
		if token._type == &EQUALS{
			let identifier = get_string(
				&file,
				tokens[index-1].position[0]..tokens[index-1].position[1]
			);
			let value = get_string(
				&file,
				tokens[index+1].position[0]..tokens[index+1].position[1]
			);
			expressions.push(BinaryExpr{left_expr:identifier,operator:&EQUALS,right_expr:value})
		}
		
	}
	//expressions.iter().for_each(|expression| println!("{:?}",expression));
	return expressions;
}
