use crate::parser::expression::make_node;
use crate::parser::expression::Run;
use std::fmt::Display;
use std::fmt;
use std::convert::TryInto;

// mod expression;

pub struct Parser<'a> {
    tokens: &'a str,
    root_node: Token,
}

pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftBracket,
    RightBracket,
    Number(i32),
    Node(Box<dyn Run>)
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let val: String = match self {
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::LeftBracket => "(".to_string(),
            Token::RightBracket => ")".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Divide => "/".to_string(),
            Token::Number(n) => format!("{}", n),
            Token::Node(_) => format!("<NODE>"),
        };
        f.debug_struct("Token").field("value", &val).finish()

    }
}

// 3 - 4 - 9 - 12
impl<'a> Parser<'a> {
    fn flatten(tokens: &mut Vec<Token>) {
        println!("{:?}", tokens);

        let mut last_left: i32 = -1;
        let mut current_right: i32 = -1;
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::LeftBracket => {
                    last_left = i.try_into().unwrap();
                }

                Token::RightBracket => {
                    assert!(last_left >= 0);
                    current_right = i.try_into().unwrap();
                    println!("Left: {}, Right: {}", last_left, current_right);
                    assert_eq!(current_right - last_left, 4);
                    break;
                }

                _ => {}
            }
        }

        println!("Current right: {}", current_right);
        tokens.remove(current_right.try_into().unwrap());
        let rhs = tokens.remove((current_right - 1).try_into().unwrap());
        let symbol = tokens.remove((current_right - 2).try_into().unwrap());
        let lhs = tokens.remove((current_right - 3).try_into().unwrap());
        tokens.remove((current_right - 4).try_into().unwrap());

        // Make node
        println!("{:?} {:?} {:?}", &lhs, &symbol, &rhs);

        let new_node = make_node(lhs, symbol, rhs);
        tokens.insert((current_right - 4).try_into().unwrap(), new_node);

        println!("{:?}", tokens);
    }

    pub fn get_value(&self) -> i32 {
        match &self.root_node {
            Token::Node(val) => {
                val.get_value()
            }
            Token::Number(n) => {
                *n
            }
            _ => panic!("Invalid statement")
        }
    }

    pub fn new(body: &'a str) -> Self {
        let formatted = format!("{}", body);
        let splits = formatted.split(" ");

        let mut tokens: Vec<Token> = Vec::new();
        for split in splits {
            match split {
                "+" => tokens.push(Token::Plus),
                "-" => tokens.push(Token::Minus),
                "*" => tokens.push(Token::Multiply),
                "/" => tokens.push(Token::Divide),
                "(" => tokens.push(Token::LeftBracket),
                ")" => tokens.push(Token::RightBracket),
                val => {
                    let an_int = val.parse::<i32>().expect(
                        &format!("Value must be an integer, but it was {}", val)
                    );
                    tokens.push(Token::Number(an_int))
                }
            }
        }

        // Until tokens list is 1, continue to flatten
        while tokens.len() > 1 {
            Self::flatten(&mut tokens);
        }

        Parser {
            tokens: body,
            root_node: tokens.pop().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let p = Parser::new("( 1 + 9 )");
        assert_eq!(p.get_value(), 10);
    }

    #[test]
    fn test_no_expression() {
        let p = Parser::new("15");
        assert_eq!(p.get_value(), 15);
    }

    #[test]
    fn test_nested() {
        let p = Parser::new("( ( ( 2 + 9 ) - ( 3 + 8 ) ) + 1 )");
        assert_eq!(p.get_value(), 1);
    }

    #[test]
    fn test_division() {
        let p = Parser::new("( ( 1 + 9 ) / 2 )");
        assert_eq!(p.get_value(), 5);
    }
}