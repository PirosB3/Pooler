
use crate::parser::parser::Token;

fn get_runnable(item: Token) -> Box<dyn Run> {
    match item {
        Token::Node(val) => val,
        Token::Number(num) => {
            let node = NumberNode{
                value: num,
            };
            Box::new(node)
        }
        _ => {
            panic!("This should never happen!")
        }
    }
}

pub fn make_node(lhs: Token, symbol: Token, rhs: Token) -> Token {
    // Token::Node
    let expr = ExpressionNode{
        symbol: symbol,
        lhs: get_runnable(lhs),
        rhs: get_runnable(rhs),
    };
    Token::Node(Box::new(expr))
}

pub trait Run {
    fn get_value(&self) -> i32;
}

struct NumberNode {
    value: i32,
}

impl Run for NumberNode {
    fn get_value(&self) -> i32 {
        self.value
    }
}

pub struct ExpressionNode {
    symbol: Token,
    rhs: Box<dyn Run>,
    lhs: Box<dyn Run>,
}

impl Run for ExpressionNode {
    fn get_value(&self) -> i32 {
        let rhs_val = self.rhs.get_value();
        let lhs_val = self.lhs.get_value();
        match self.symbol {
            Token::Minus => lhs_val - rhs_val,
            Token::Plus => lhs_val + rhs_val,
            Token::Multiply => lhs_val * rhs_val,
            Token::Divide => lhs_val / rhs_val,
            _ => {
                panic!("This should never happen")
            }
        }
    }
}