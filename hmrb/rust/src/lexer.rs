use pest::Parser;
use pest::error::Error;
use std::fs;

#[derive(Parser)]
#[grammar = "bab.pest"]
pub struct Grammar;

enum Core<'a> {
    Array(Vec<Core<'a>>),
    String(&'a str),
}

fn lex(file: &str) -> Result<Core, Error<Rule>> {
    fn parse_element(token: pest::iterators::Pair<Rule>) -> Core {
        match token.as_rule() {
            Rule::grammar => {
                println!("GRAMMAR {}", token.as_str());
                Core::Array(token.into_inner().map(parse_element).collect())}
                ,
            Rule::law => {
                println!("LAW {}", token.as_str());
                Core::Array(token.into_inner().map(parse_element).collect())
            },
            Rule::var => {
                println!("LAW {}", token.as_str());
                Core::Array(token.into_inner().map(parse_element).collect())
            },
            Rule::attributes => {
                println!("ATTRS {}", token.as_str());
                Core::Array(token.into_inner().map(parse_element).collect())
            },
            Rule::attribute => {
                println!("ATTR {}", token.as_str());
                Core::Array(token.into_inner().map(parse_element).collect())
            },
            Rule::name => {
                println!("NAME {}", token.as_str());
                Core::String(token.as_str())
            },
            Rule::value => {
                println!("VALUE {}", token.as_str());
                Core::String(token.as_str())
            },
            Rule::WHITESPACE => unreachable!()
        }
    }

    let grammar_tree = Grammar::parse(Rule::grammar, &file)?.next().unwrap();
    Ok(parse_element(grammar_tree))
}


#[test]
fn test_lexer() {
    let test_file = fs::read_to_string("tests/lexer.1").expect("cannot read file");
    let core: Core = lex(&test_file).expect("unsuccessful parse");
    assert_eq!(true, true);
}
