use pest::error::Error;
use pest::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "bab.pest"]
pub struct Grammar;

#[derive(Debug)]
pub enum Core<'a> {
    Array(Vec<Core<'a>>),
    HashMap(&'a str, Regex),
}

pub fn lexer(file: &str) -> Result<Core, Error<Rule>> {
    fn parse_element(token: pest::iterators::Pair<Rule>) -> Core {
        match token.as_rule() {
            Rule::grammar => Core::Array(token.into_inner().map(parse_element).collect()),
            Rule::law => Core::Array(token.into_inner().map(parse_element).collect()),
            Rule::var => Core::Array(token.into_inner().map(parse_element).collect()),
            Rule::attributes => Core::Array(token.into_inner().map(parse_element).collect()),
            Rule::attribute => {
                let mut tokens = token.into_inner();
                let name = tokens.next().unwrap().as_str();
                let value = tokens.next().unwrap().as_str();
                Core::HashMap(name, Regex::new(&format!("{}", value)).unwrap())
            }
            Rule::name => unreachable!(),
            Rule::value => unreachable!(),
            Rule::WHITESPACE => unreachable!(),
        }
    }

    let grammar_tree = Grammar::parse(Rule::grammar, &file)?.next().unwrap();
    Ok(parse_element(grammar_tree))
}

#[test]
fn test_lexer_1() {
    let test_file = fs::read_to_string("tests/lexer.1").expect("cannot read file");
    let core: Core = lexer(&test_file).expect("unsuccessful parse");
    assert_eq!(
        format!("{:?}", core),
        "Array([Array([Array([HashMap(\"lemma\", Hello), HashMap(\"lemma\", World)])])])"
    );
}
