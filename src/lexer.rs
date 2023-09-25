use python_compiler::{
	Token,
	TokenRegex,
	TokenType::*
};
use regex::Regex;

pub fn tokenize(input_str:&str) -> Vec<Token>{
	
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
		TokenRegex{_type:&QUOTATION,regex_pattern:Regex::new(r#"""#).unwrap()},
		TokenRegex{_type:&DIVIDE,regex_pattern:Regex::new(r#"(\/)"#).unwrap()},
		TokenRegex{_type:&MULTIPY,regex_pattern:Regex::new(r#"(\*)"#).unwrap()},
		TokenRegex{_type:&ADD,regex_pattern:Regex::new(r#"(\+)"#).unwrap()},
		TokenRegex{_type:&SUBTRACT,regex_pattern:Regex::new(r#"(\-)"#).unwrap()},
		TokenRegex{_type:&GREATERTHAN,regex_pattern:Regex::new(r#"(\-)"#).unwrap()}
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
	
	for (_size,token) in items.into_iter().enumerate(){
		if token._type == &QUOTATION{
			if quotation_found{
				let start_position:&Vec<usize> = string_vec.first().unwrap();
				let end_postion = string_vec.last().unwrap();
				tokens_temp.push(Token { _type:&TSTRING, position:vec![start_position[0],end_postion[1]] });
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

	for (_size,i) in tokens_temp.into_iter().enumerate(){
		tokens.push(i)
	}
	//Removing the whitespace but remember to check for errors before it is removed
	tokens.sort_by(|a,b| a.position.cmp(&b.position));
	
	tokens.retain(|token| token._type != &QUOTATION && token._type != &WHITESPACE);

	let mut to_be_removed = vec![];

	tokens.iter().for_each(|token|{
		if token._type == &TSTRING{
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
