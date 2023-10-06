use python_compiler::{
	Token,
	TokenRegex,
	TokenType::*,
};
use regex::Regex;
use crate::utils::is_in_range;


pub fn tokenize(input_str:&str) -> Vec<Token>{
	//TODO problems with identifiers
	let mut tokens: Vec<Token<'_>> = vec![];

	let regex_patterns: Vec<TokenRegex> = vec![
		TokenRegex{_type:&CLASS,regex_pattern:Regex::new(r"(class)").unwrap()},
		TokenRegex{_type:&WHITESPACE,regex_pattern:Regex::new(r"\s").unwrap()},
		TokenRegex{_type:&DEF,regex_pattern:Regex::new(r"(def)").unwrap()},
		TokenRegex{_type:&NEWLINE,regex_pattern:Regex::new(r"\n").unwrap()},
		TokenRegex{_type:&EQUALS,regex_pattern:Regex::new(r"(=)").unwrap()},
		TokenRegex{_type:&NUMBER,regex_pattern:Regex::new(r"\d").unwrap()},
		TokenRegex{_type:&TIDENTIFIER,regex_pattern:Regex::new(r#"[a-z][^\n\d\s]+"#).unwrap()},
		TokenRegex{_type:&TSTRING,regex_pattern:Regex::new(r#"("+.+")"#).unwrap()},
		TokenRegex{_type:&DIVIDE,regex_pattern:Regex::new(r#"(\/)"#).unwrap()},
		TokenRegex{_type:&MULTIPY,regex_pattern:Regex::new(r#"(\*)"#).unwrap()},
		TokenRegex{_type:&ADD,regex_pattern:Regex::new(r#"(\+)"#).unwrap()},
		TokenRegex{_type:&SUBTRACT,regex_pattern:Regex::new(r#"(\-)"#).unwrap()},
		TokenRegex{_type:&GREATERTHAN,regex_pattern:Regex::new(r#"(\-)"#).unwrap()},
		TokenRegex{_type:&OPENPARENTHESIS,regex_pattern:Regex::new(r#"\("#).unwrap()},
		TokenRegex{_type:&CLOSEPARENTHESIS,regex_pattern:Regex::new(r#"\)"#).unwrap()}
	];
	
	for i in regex_patterns {
		let _matches = i.regex_pattern.find_iter(&input_str);
		for regex_match in _matches{
			tokens.push(Token{ _type: &i._type, position: vec![regex_match.start(),regex_match.end()]});
		}
	}

	let mut to_be_removed = vec![];
	
	tokens.retain(|token| token._type != &QUOTATION && token._type != &WHITESPACE);

	let tokens_temp = tokens.clone();

	tokens_temp.iter().for_each(|token| 
		if token._type == &TSTRING {
			tokens_temp.iter().for_each(|t|
			if is_in_range(token.position[0]..token.position[1],t.position[0]..t.position[1]) && 
				t._type == &TIDENTIFIER {
				to_be_removed.push(t);
				//println!("{:?}",t._type)
			}
		)
		}
	);
	
	tokens.sort_by(|a,b| a.position.cmp(&b.position));

	tokens.retain(|token| !to_be_removed.contains(&token));

	return tokens;
}

