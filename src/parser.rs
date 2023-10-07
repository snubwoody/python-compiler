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

	let mut string_vec: Vec<&str> = vec![];

	for (index,token) in tokens.iter().enumerate(){
		match token._type {
			&TIDENTIFIER |
			&TSTRING |
			&NUMBER => {
				let expr;
				if token._type == &NUMBER {
					let number = 
					get_string(file, token.position[0]..token.position[1])
					.parse::<i32>().unwrap();

					expr = LiteralExpression(INT(number));
				}
				else if token._type == &TSTRING {
					let string = get_string(file, token.position[0]..token.position[1]);

					expr = LiteralExpression(STRING(string));
				}
				else {
					let string = get_string(file, token.position[0]..token.position[1]).as_str();
					string_vec.push(string);

					expr = LiteralExpression(IDENTIFIER(string_vec.last().unwrap()));
				}
				
				if expressions.is_empty(){
					expressions.push(expr.clone());
					continue;
				}

				match expressions[expressions.len()-1].clone() {
					BinaryExpression(mut left,opr,mut right) => {

						let mut right_expr = *right;

						match right_expr {
							LiteralExpression(k) => {
								if k == DANGLINGEXPR {
									expressions.pop();
									expressions.pop();
									right_expr = expr.clone();
									expressions.push(BinaryExpression(left,opr,Box::new(right_expr)));
								}
								else {
									expressions.push(expr);
								}
							}
							_ => {
								expressions.push(expr);
								//println!("nope")
							}
						}
					},
					_ => {
						expressions.push(expr);
					}
				}
				
			},
			&ADD | 
			&EQUALS |
			&MULTIPY |
			&SUBTRACT | 
			&DIVIDE	=> 
			{
				let previous_expr = expressions[expressions.len()-1].clone();
				let next_expr = parse_literal_expression(&tokens[index+1], file);
				match previous_expr {
					BinaryExpression(x,y,z) => {}
					_ =>{
						expressions.push(
							BinaryExpression(Box::new(previous_expr),token._type.clone(),Box::new(LiteralExpression(DANGLINGEXPR)), )
						)
					}
				}
			},
			&OPENPARENTHESIS => {
				grouping_expr = true;
			}
			&CLOSEPARENTHESIS => {
				grouping_expr = false;
			}
			_ => {}
		}
		let expressions_temp: Vec<Expression<'a>> = expressions.clone();
		if grouping_expr {
			
			match expressions_temp.last() {
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

	grouping_expr_temp.iter().for_each(|t|println!("{:?}",t));

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
		_ => {return LiteralExpression(NONE);}
	}

}


