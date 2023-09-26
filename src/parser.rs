use crate::utils::get_string;
use std::vec;
use python_compiler::{
	Token,
	BinaryExpr,
	TokenType::*,
};


pub fn parse_expressions(tokens:&Vec<Token<'_>>,file:&String) -> Vec<BinaryExpr<'static>>{
	let mut expressions:Vec<BinaryExpr<'_>> = vec![];
	for (index,token) in tokens.iter().enumerate() {
		
		//TODO change the impl for this because there are a lot of operators
		match token._type {
			&EQUALS => {
				let identifier = get_string(
					&file,
					tokens[index-1].position[0]..tokens[index-1].position[1]
				);
				let value = get_string(
					&file,
					tokens[index+1].position[0]..tokens[index+1].position[1]
				);
				expressions.push(BinaryExpr{left_expr:identifier,operator:&EQUALS,right_expr:value})
			},
			&DIVIDE => {
				let identifier = get_string(
					&file,
					tokens[index-1].position[0]..tokens[index-1].position[1]
				);
				let value = get_string(
					&file,
					tokens[index+1].position[0]..tokens[index+1].position[1]
				);
				expressions.push(BinaryExpr{left_expr:identifier,operator:&DIVIDE,right_expr:value})
			},
			&ADD => {
				let identifier = get_string(
					&file,
					tokens[index-1].position[0]..tokens[index-1].position[1]
				);
				let value = get_string(
					&file,
					tokens[index+1].position[0]..tokens[index+1].position[1]
				);
				expressions.push(BinaryExpr{left_expr:identifier,operator:&ADD,right_expr:value})
			},
			&MULTIPY => {
				let identifier = get_string(
					&file,
					tokens[index-1].position[0]..tokens[index-1].position[1]
				);
				let value = get_string(
					&file,
					tokens[index+1].position[0]..tokens[index+1].position[1]
				);
				expressions.push(BinaryExpr{left_expr:identifier,operator:&MULTIPY,right_expr:value})
			},
			&SUBTRACT => {
				
				let identifier = get_string(
					&file,
					tokens[index-1].position[0]..tokens[index-1].position[1]
				);
				let value = get_string(
					&file,
					tokens[index+1].position[0]..tokens[index+1].position[1]
				);
				if tokens[index+1]._type != &GREATERTHAN{
					expressions.push(BinaryExpr{left_expr:identifier,operator:&SUBTRACT,right_expr:value})
				}
			},
			_ => {}
		}
		
	}
	//expressions.iter().for_each(|expression| println!("{:?}",expression));
	return expressions;
}
