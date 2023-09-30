use crate::utils::get_string;
use std::vec;
use python_compiler::{
	Token,
	BinaryExpr,
	TokenType::*,
	ExpressionType::*,
	Expression,
	DataType::*
};


pub fn parse_expressions(tokens:&Vec<Token<'_>>,file:&String) -> Vec<Expression<'static>>{
	let mut expressions:Vec<Expression<'_>> = vec![];
	//parse_grouping_expression(&mut expressions, tokens, file);
	parse_literal_expression(&mut expressions, tokens, file);
	parse_binary_expression(&mut expressions,tokens,file);

	return expressions;
}


fn parse_grouping_expression(expressions:&mut Vec<Expression<>>,tokens:&Vec<Token<>>,file:&String){
}

fn parse_literal_expression(expressions:&mut Vec<Expression<>>,tokens:&Vec<Token<>>,file:&String){
	for (_index,token) in tokens.iter().enumerate() {		
		match token._type {
			&TSTRING => {
				let data = get_string(
					&file,
					token.position[0]..token.position[1]
				);
				expressions.push(Expression::new(LiteralExpression(STRING(data)), token.position.clone()))
			},
			&NUMBER => {
				let data = get_string(
					&file,
					token.position[0]..token.position[1]
				);
				expressions.push(Expression::new(LiteralExpression(INT(data.parse::<i32>().unwrap())), token.position.clone()))
			},
			&TIDENTIFIER => {
				let data = get_string(
					&file,
					token.position[0]..token.position[1]
				);
				expressions.push(Expression::new(LiteralExpression(IDENTIFIER(data)), token.position.clone()))
			},
			_ => {}
		}
	}
	
}

fn parse_binary_expression(expressions:&mut Vec<Expression<>>,tokens:&Vec<Token<>>,file:&String){

	for (index,token) in tokens.iter().enumerate() {
		
		//TODO change the impl for this because there are a lot of operators
		match token._type {
			&EQUALS => {
				let position = &token.position;
				for (index,expression) in expressions.iter().enumerate(){
					if expression.position > position.clone() {
						println!("{:?}, item: {:?}", token._type, get_string(file, expression.position[0]..expression.position[1]));

						break;
					}
					
				}
			},
			_ => {}
		}
		
	}
	
}