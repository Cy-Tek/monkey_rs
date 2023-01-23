use std::any::Any;

use crate::{lexer::Lexer, parser::Parser, ast::{LetStatement, Node}};

fn test_let_statement(statement: Box<dyn Any>, name: &str) {
	let let_statement = statement.downcast::<LetStatement>().unwrap();
	assert_eq!(let_statement.token_literal(), "let");
	assert_eq!(let_statement.name.value, name);
	assert_eq!(let_statement.name.token_literal(), name);
}

#[test]
fn test_let_statements() {
    let input = r###"
let x = 5;
let y = 10;
let foobar = 838383;
"###;

	let l = Lexer::new(input.to_owned());
	let mut parser = Parser::new(l);
	let program = parser.parse_program();

	assert_eq!(program.statements.len(), 3, "program.statements should have 3 valid results");

	let tests = vec![
		"x",
		"y",
		"foobar"
	];

	tests.into_iter().zip(program.statements.into_iter()).for_each(|(expected, statement)| {
		test_let_statement(statement, expected);
	})
}
