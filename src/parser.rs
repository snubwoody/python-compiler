use crate::utils::get_string;
use std::vec;
use python_compiler::{
	Token,
	TokenType::*,
	Expression::{*,self},
	BinaryExpr,
	DataType::*,
};

//TODO Without spaces binary expressions are tokenized as a word instead so FIX that
#[allow(unused_variables,unused_mut)]
pub fn parse_expressions(tokens:&Vec<Token<'_>>,file:&String) -> Vec<Expression> {
	//parse_grouping_expression(&mut expressions, tokens, file);
	let mut expressions:Vec<Expression> = vec![];

	for (index,token) in tokens.iter().enumerate(){
		match token._type {
			&NUMBER => {
				let number = 
					get_string(file, token.position[0]..token.position[1])
					.parse::<i32>().unwrap();

				let expr = LiteralExpression(INT(number));
				
				if expressions.is_empty(){
					expressions.push(expr.clone());
					continue;
				}

				match expressions[expressions.len()-1].clone() {
					BinaryExpression(mut val) => {
						//println!("yurr {:?}", expressions[expressions.len()-1].clone());
						let right_expr = *val.right_expr;
						match right_expr {
							LiteralExpression(k) => {
								if k == DANGLINGEXPR {
									expressions.pop();
									expressions.pop();
									val.right_expr = Box::new(expr.clone());
									expressions.push(BinaryExpression(val));
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
					BinaryExpression(v) => {}
					_ =>{
						expressions.push(
							BinaryExpression(
								BinaryExpr { 
									left_expr: Box::new(previous_expr), 
									operator: token._type.clone(), 
									right_expr: Box::new(LiteralExpression(DANGLINGEXPR)), 
								}
							)
						)
					}
				}
			}
			_ => {}
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


