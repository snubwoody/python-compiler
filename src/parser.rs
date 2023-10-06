use crate::utils::get_string;
use std::vec;
use python_compiler::{
	Token,
	TokenType::*,
	Expression::{*,self},
	DataType::*,
};

//TODO its panicking for strings
#[allow(unused_variables,unused_mut)]
pub fn parse_expressions(tokens:&Vec<Token<'_>>,file:&String) -> Vec<Expression> {
	let mut expressions:Vec<Expression> = vec![];

	let expressions_temp = expressions.clone();

	let mut grouping_expr = false;

	let mut grouping_expr_temp = vec![];

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
					let string = get_string(file, token.position[0]..token.position[1]);

					expr = LiteralExpression(IDENTIFIER(string));
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
			_ => {}
		}
		
		if grouping_expr {
			let l = &expressions_temp[expressions_temp.len()-1];
			grouping_expr_temp.push(l)
		}
	}

	return expressions;
}

fn parse_literal_expression(token:&Token,file:&String) -> Expression {
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


