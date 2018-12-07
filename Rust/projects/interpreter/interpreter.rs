
// Author: Kyle Sargent
// CSC 333: HW5
// Due: 12/03/18 
//
//
// In this file you will implement a parser
// for the following grammar:
//
//     expr   -> term   [ (‘+’ | ‘-’) term   ]*
//     term   -> factor [ (‘*’ | ‘/’) factor ]*
//     factor -> NumberLiteral | ‘(‘ expr ‘)’
//



//
// Tokens (lexems) recognized by the scanner
//

#[derive(Debug, Clone, Copy)]
enum Token {
    Plus, Minus, Times, Divide,
    LeftParen, RightParen,
    NumberLiteral(f64),
}


//
// The scanner object implements regular expressions
// for recognizing the tokens found in the enum above
//

#[derive(Debug)]
struct Scanner {
    input: Vec<char>,
    cursor: usize,
}


impl Scanner {

    // A function for creating scanners from strings
    fn new(input_string: String) -> Scanner {
        Scanner {
            input: input_string.chars().collect(),
            cursor: 0
        }
    }

    // Check to see if the entire input string has been read
    fn input_remaining(&self) -> bool {
        self.cursor < self.input.len()
    }

    // Parse a number literal: Digit+ ('.' Digit*)?
    // Numbers start with a digit and can optionally include a decimal
    // followed by zero or more digits
    fn parse_number_literal(&mut self, digit: char) -> Token {
        let mut number = String::new();

        // The first Digit was skipped in get_next_token
        // Digit
        number.push(digit);

        // Digit*
        while self.input_remaining() && self.input[self.cursor].is_digit(10) {
            number.push(self.input[self.cursor]);
            self.cursor += 1;
        }

        // ('.' Digit*)?
        if self.input_remaining() && self.input[self.cursor] == '.' {
            number.push(self.input[self.cursor]);
            self.cursor += 1;

            // Digit*
            while self.input_remaining() && self.input[self.cursor].is_digit(10) {
                number.push(self.input[self.cursor]);
                self.cursor += 1;
            }
        }

        // The parse will never fail given the above regular expression
        // (We always have at least a one digit number)
        Token::NumberLiteral(number.parse::<f64>().unwrap())
    }

    // Grab the next token from the input string
    fn get_next_token(&mut self) -> Option<Token> {

        // Skip whitespace
        while self.input_remaining() && self.input[self.cursor].is_whitespace() {
            self.cursor += 1;
        }

        // Check for end of input
        if !self.input_remaining() {
            return None;
        }

        // Advance past the current character
        let current_char = self.input[self.cursor];
        self.cursor += 1;

        // Recognize (and return) the current token
        match current_char {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Times),
            '/' => Some(Token::Divide),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            d @ '0' ... '9' => Some(self.parse_number_literal(d)),
            _ => None
        }
    }
}


//
// An enum for abstract syntax tree nodes
//
// TODO: fill in the enum; you will need five different
//       variants, and they must be named:
//       - Addition
//       - Subtraction
//       - Multiplication
//       - Division
//       - Number

#[derive(Debug)]
enum ASTNode {
    Addition(Box<ASTNode>, Box<ASTNode>),
    Subtraction(Box<ASTNode>, Box<ASTNode>),
    Multiplication(Box<ASTNode>, Box<ASTNode>),
    Division(Box<ASTNode>, Box<ASTNode>),
    Number(f64),
}


//
// A parser object that creates an abstract syntax tree
// by continually requesting tokens from the scanner
//

#[derive(Debug)]
struct Parser {
    scanner: Scanner,
    token: Option<Token>,
}


impl Parser {

    // A function for creating new parsers
    fn new(input_string: String) -> Parser {
        Parser {
            scanner: Scanner::new(input_string),
            token: None
        }
    }

    // Advanced past the current token
    fn advance_token(&mut self) {
        self.token = self.scanner.get_next_token();

        //
        // The following lines can be uncommented for debugging purposes
        //

        // match self.token {
        //     Some(tok) => println!("  {:?}", tok),
        //     None => (),
        // }
    }

    // The top-level call to start parsing
    //
    // The parse functions return a Result object:
    //      Ok(ASTNode) is returned if the parse is successful
    //      Err(String) is returned if there is a parse error
    //
    fn parse(&mut self) -> Result<ASTNode, String> {

        // Advance to first token
        self.advance_token();

        // Start recognizing
        // See: https://doc.rust-lang.org/book/second-edition/
        //              ch09-02-recoverable-errors-with-result.html
        // for a discussion on the Result type and the question mark operator
        let ast = self.expr()?;

        // Return an error if the input is not empty aftering a completed parse
        match self.scanner.input_remaining() || self.token.is_some() {
            true => Err(String::from("Not all input tokens were consumed.")),
            false => Ok(ast)
        }
    }

    // expr -> term [ (‘+’ | ‘-’) term ]*
    fn expr(&mut self) -> Result<ASTNode, String> {
        // TODO this function must be implemented
        let mut ast = self.term()?;
        loop {
            match self.token {
                Some(Token::Plus) => {
                    self.advance_token();
                    let rhs = self.term()?;
                    ast = ASTNode::Addition(Box::new(ast), Box::new(rhs));
                },
                Some(Token::Minus) => {
                    self.advance_token();
                    let rhs = self.term()?;
                    ast = ASTNode::Subtraction(Box::new(ast), Box::new(rhs))
                },
                _ => break
            }
        }
        Ok(ast)
    }

    // term -> factor [ (‘*’ | ‘/’) factor ]*
    fn term(&mut self) -> Result<ASTNode, String> {
        // TODO this function must be implemented
        let mut ast = self.factor()?;
        loop {
            match self.token {
                Some(Token::Times) => {
                    self.advance_token();
                    let rhs = self.factor()?;
                    ast = ASTNode::Multiplication(Box::new(ast), Box::new(rhs));
                },
                Some(Token::Divide) => {
                    self.advance_token();
                    let rhs = self.factor()?;
                    ast = ASTNode::Division(Box::new(ast), Box::new(rhs))
                },
                _ => break
            }
        }
        Ok(ast)
    }

    // factor -> NumberLiteral | ‘(‘ expr ‘)’
    fn factor(&mut self) -> Result<ASTNode, String> {
       match self.token {
           Some(Token::NumberLiteral(val)) =>{
                self.advance_token();
                Ok(ASTNode::Number(val))
           },
           Some(Token::LeftParen) => {
               self.advance_token();
               let mut ast = self.expr()?;
               match self.token {
                   Some(Token::RightParen) =>{
                    self.advance_token();
                    Ok(ast)
                   },
                   _ => Err(String::from("Expected NumberLiteral or RightParen"))
               }
           }
            _ => Err(format!("Expected NumberLiteral or LeftParen, Found: {:?}", self.token))
       }
    }
}   

fn main() {

    use std::io;

    // Ask the user for an arithmetic expression
    println!("Please provide an arithmetic expression to parse:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Could not read input.");
    input_string = String::from(input_string.trim());

    // Parse the input and print the result
    let mut parser = Parser::new(input_string);
    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        Err(msg) => println!("ERROR: {}", msg)
    }
}
