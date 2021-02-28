use crate::parser::parser::Parser;
use std::io::{self, Read, BufRead};

mod parser;

fn read_line(input: &mut std::io::Stdin) -> String {
    print!("Enter prompt >> ");
    let mut iterator = input.lock().lines();
    let line = iterator.next().unwrap().unwrap();
    line
}

fn inner(s: &String) {
    
}

fn boxer(val: &str) -> Box<String> {
    let item = format!("Values: {}", val);
    let asd = Box::new(item);
    asd
}

fn main() {
    let values = boxer("Foo");
    println!("{}", values);

    println!("---------------------------------");

    let second_parsed = Parser::new("( ( ( 2 + 9 ) - ( 3 + 8 ) ) + 1 )");
    let val2 = second_parsed.get_value();
    println!("Final value is {}", val2);
}