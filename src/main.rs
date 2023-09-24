use python_compiler::file_to_string;
use regex::Regex;
use std::{vec, collections::binary_heap, ops::Range};
mod lib;
use lib::*;
use TokenType::*;
use LiteralType::*;
mod codegen;
use  codegen::code_gen;


fn main() {
	let file_tokens: Vec<Token<'_>>;

    match file_to_string("src/main.py") {

		Ok(file_contents) => {
			file_tokens = tokenize(&file_contents);
			
			//generate_ast(&file_tokens,&file_contents);
			parse_expressions(&file_tokens, &file_contents);
			//get_string()
			/*
			for (size,token) in file_tokens.iter().enumerate(){
				println!("{:?} , {:?}",token._type,token.position)
			}
			*/
		}
		Err(error) => {
			println!("{}",error);
		}
	}
	
}

//TODO generate expressions

fn tokenize(input_str:&str) -> Vec<Token>{
	
	let mut tokens: Vec<Token<'_>> = vec![];
	//&CLASS,Regex::new(r"(class)").unwrap()
	let regex_patterns: Vec<TokenRegex> = vec![
		TokenRegex{_type:&CLASS,regex_pattern:Regex::new(r"(class)").unwrap()},
		TokenRegex{_type:&WHITESPACE,regex_pattern:Regex::new(r"\s").unwrap()},
		TokenRegex{_type:&DEF,regex_pattern:Regex::new(r"(def)").unwrap()},
		TokenRegex{_type:&NEWLINE,regex_pattern:Regex::new(r"\n").unwrap()},
		TokenRegex{_type:&EQUALS,regex_pattern:Regex::new(r"(=)").unwrap()},
		TokenRegex{_type:&NUMBER,regex_pattern:Regex::new(r"\d").unwrap()},
		TokenRegex{_type:&WORD,regex_pattern:Regex::new(r"\w+[^(\n\s)]+\w").unwrap()},
		TokenRegex{_type:&QUOTATION,regex_pattern:Regex::new(r#"""#).unwrap()}
	];
	
	for i in regex_patterns {
		let _matches = i.regex_pattern.find_iter(&input_str);
		for regex_match in _matches{
			tokens.push(Token{ _type: &i._type, position: vec![regex_match.start(),regex_match.end()]})
		}
	}
	
	tokens.sort_by(|a,b| a.position.cmp(&b.position));

	let items = tokens.clone();

	let mut quotation_found = false;
	let mut string_vec = vec![];
	let mut tokens_temp = vec![];
	
	for (size,token) in items.into_iter().enumerate(){
		if token._type == &QUOTATION{
			if quotation_found{
				let start_position:&Vec<usize> = string_vec.first().unwrap();
				let end_postion = string_vec.last().unwrap();
				tokens_temp.push(Token { _type:&STRING, position:vec![start_position[0],end_postion[1]] });
				string_vec.clear();
			}
			quotation_found = !quotation_found;
			continue;
		}
		if quotation_found{
			//println!("{:?}",token._type);
			string_vec.push(token.position)
		}
	}

	for (size,i) in tokens_temp.into_iter().enumerate(){
		tokens.push(i)
	}
	//Removing the whitespace but remember to check for errors before it is removed
	tokens.sort_by(|a,b| a.position.cmp(&b.position));
	
	tokens.retain(|token| token._type != &QUOTATION && token._type != &WHITESPACE);

	let mut to_be_removed = vec![];

	tokens.iter().for_each(|token|{
		if token._type == &STRING{
			let range = &token.position;
			tokens.iter().for_each(|item| {
				if (range.contains(&item.position[0]) || range.contains(&item.position[1])) && item != token{
					to_be_removed.push(item.clone());
				} 
			})
		}
	});

	tokens.retain(|token| !to_be_removed.contains(token));

	return tokens;
}


fn parse_expressions(tokens:&Vec<Token<'_>>,file:&String) -> Vec<BinaryExpr<'static>>{
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
	expressions.iter().for_each(|expression| println!("{:?}",expression));
	return expressions;
}


fn get_string(string:&String,range:Range<usize>) -> String{
	let mut word = String::new();
	let range = range;
		
	for i in range.into_iter(){
		word.push(string.chars().nth(i).unwrap())
	}
	return word;
}