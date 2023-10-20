use crate::utils::get_string;
use std::vec;
use python_compiler::{
	Token,
	TokenType::*,
	Expression::{*,self},
	DataType::*,
};

#[allow(unused_variables,unused_mut)]
pub fn parse_expressions<'a>(tokens:&Vec<Token<'_>>,file:&String) -> Vec<Expression<'a>> {
	let mut expressions:Vec<Expression<'a>> = vec![] ;

	let mut grouping_expr: bool = false;

	let mut grouping_expr_temp: Vec<Expression<'a>> = vec![];

	let mut string_vec: Vec<String> = vec![];

	for (index,token) in tokens.iter().enumerate(){
		match token._type {

			&TIDENTIFIER |
			&TSTRING |
			&NUMBER => {
				let expr: Expression<'_> = parse_literal_expression(token, file);
				
				if expressions.is_empty(){
					expressions.push(expr.clone());
					continue;
				}
			},
			&ADD | 
			&EQUALS |
			&MULTIPY |
			&SUBTRACT | 
			&DIVIDE	=> 
			{
				match expressions.last().unwrap() {
					BinaryExpression(..) => {},
					_ => {
						expressions.pop();
					}
				}
				
				expressions.push(
					parse_binary_expression(
						token.clone(), 
						file,
						expressions.clone(),
						parse_literal_expression(&tokens[index-1], file),
						index,
						tokens
					)
				);
				
				
			},
			&OPENPARENTHESIS => {
				grouping_expr = true;
			}
			&CLOSEPARENTHESIS => {
				grouping_expr = false;
			}
			_ => {}
		}
		
		if grouping_expr {
			
			match expressions.last() {
				Some(val) => {
					let k = val.clone();
					grouping_expr_temp.push(k);
				},
				None => {
					println!("it is empty")
				}
			}

			
		}
	}

	//grouping_expr_temp.iter().for_each(|t|println!("{:?}",t));

	return expressions;
}

fn parse_literal_expression<'a>(token:&Token,file:&String) -> Expression<'a> {
	match token._type {
		&NUMBER => {
			let number = 
				get_string(file, token.position[0]..token.position[1])
				.parse::<i32>().unwrap();
			return LiteralExpression(INT(number))
		},
		&TSTRING => {
			let string = get_string(file, token.position[0]..token.position[1]);
			return LiteralExpression(STRING(string));
		},
		&TIDENTIFIER => {
			let string = get_string(file, token.position[0]..token.position[1]);
			return  LiteralExpression(IDENTIFIER(string));
		}
		_ => {return LiteralExpression(NONE);}
	}

}

fn parse_binary_expression<'a>(
	token:Token,
	file:&String,
	_expressions:Vec<Expression<'a>>,
	prev_expr:Expression<'a>,
	index:usize,
	tokens:&Vec<Token>
) -> Expression<'a> {
	//TODO account for multiple binary expressions
	let next_expr = parse_literal_expression(&tokens[index+1], file);
	BinaryExpression(Box::new(prev_expr),token._type.clone(), Box::new(next_expr))
}
