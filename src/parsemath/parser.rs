use super::{
    ast::Node, 
    token::{Token, OperPrec}, 
    tokenizer::Tokenizer
};

use std::{convert::From};
use std::fmt;

/// The parser structure take a Tokenizer and convert the tokens into node to make the AST
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

/// Parse error enum contains all the parse errors and display them with the `Display` trait 
/// # Arguments
/// * `message: String` - is the error message to display when this is formatted as string
/// # Example
/// ```
/// // We can take the unable to parse enum with an string and display them
/// let parse_error = ParseError::UnableToParse("The value is unable to parse");
/// println!({}, parse_error)
/// ```
/// # Example 2
/// ```
/// // We can match an error expression to handle the error
/// fn handle_error(error: ParseError) {
///     use ParseError::*;
///     match error {
///        UnableToParse => /* do something */,
///        InvalidOperator => /* do another thing */,
///     }
/// }
#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

// The trait for display with format! or println!
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

// Convert from boxed to an error enum variant
impl From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        return ParseError::UnableToParse("Unable to parse".into());
    }
}

impl<'a> Parser<'a> {
    /// Returns a new parse instance
    /// # Arguments
    /// * `expr: 'a str` - An string representing an arithmetic expression like "1*2+5*(10+5)" 
    /// # Returns
    /// * `Result<Parser, ParseError>` - Returns an parser instance `Ok(parser)` or an error `Err(err)`
    /// # Example
    /// ```
    /// // Creates a new instance of the parser structure
    /// let add = Paser::new("2+2");
    /// ```
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        // We create a new lexer instance
        let mut lexer = Tokenizer::new(expr);
        let curr_token = match lexer.next() {
            Some(token) => token,
            // If there is an invalid character
            None => return Err(ParseError::InvalidOperator("Invalid character".into()))
        };

        // We set the curr_token and the lexer
        Ok(Parser {
            tokenizer: lexer,
            current_token: curr_token,
        })
    }

    /// Parse function is the responsible of parse the input `&str` and returns an ast
    /// This ast can be evaluated with the eval funcion within ast.rs file
    /// # Returns
    /// * `Result<Node, ParseError>` - parse returns an ast root node or an error `Ok(root_ast)` or `Err(err)`
    /// 
    /// # Example
    /// ```
    /// let addition = Parser::new("1+1");
    /// let parsed = addtion.parse();
    /// // The parsed ast should be like:
    /// // ADD(Box::new(NUMBER(1.0)), Box::new(NUMBER(2.0)))
    /// ```
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        // We generate the ast
        let ast = self.generate_ast(OperPrec::DEFAULTZERO);
        // And handle a possible error
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e)
        }
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        // We advance to the next token
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into()))
        };

        self.current_token = next_token;
        Ok(())
    }

    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        // We chek if the partentesis are missmatched
        // And if there is missmatched we return an error
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(
                format!("Expected {:?}, got {:?}", expected, self.current_token)))
        }
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        // We clone the current token in the instance
        let token = self.current_token.clone();

        match token {
            // If the token is subtract
            Token::SUBTRACT => {
                // We advance the token to get the number token
                self.get_next_token()?;
                // We generate the negative number operator token
                let expr = self.generate_ast(OperPrec::NEGATIVE)?;
                // And return the Ok with the value
                Ok(Node::NEGATIVE(Box::new(expr)))
            },

            Token::NUM(i) => {
                // If the token is a number we advance and return
                self.get_next_token()?;
                Ok(Node::NUMBER(i))
            },

            Token::LEFTPAREN => {
                // If the token is a left parentesis
                self.get_next_token()?;
                // We generate a default zero token 
                let expr = self.generate_ast(OperPrec::DEFAULTZERO)?;
                // Check for mismatched parentesis
                self.check_paren(Token::RIGHTPAREN)?;
                // If the current token is a left parent
                if self.current_token == Token::LEFTPAREN {
                    // We generate the node with a multiply and division precedence
                    let right = self.generate_ast(OperPrec::MULDIV)?;
                    // and we return de expression node
                    return Ok(Node::MULTIPLY(Box::new(expr), Box::new(right)));
                }
                // We return the expression node
                Ok(expr)
            },
            // If the value is unexpected we return an error
            _ => Err(ParseError::UnableToParse("Unable to Parse".to_string()))
        }
    }

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        // To generate the ast we parse the fisrt number for the left side of the expression
        let mut left_expr = self.parse_number()?;
        // We check if the operation precedence is lowest
        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break
            }
            let right_expr = self.convert_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        // And we return a node for the left expression
        Ok(left_expr)
    }

    fn convert_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        // Here we convert the tokens to nodes
        // is the same for all tokens
        match self.current_token {
            Token::ADD => {
                // We Advance
                self.get_next_token()?;
                // We get the right side expression
                let right_expr = self.generate_ast(OperPrec::ADDSUB)?;
                // We return an operation node
                Ok(Node::ADD(Box::new(left_expr), Box::new(right_expr)))
            },

            Token::SUBTRACT => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::ADDSUB)?;
                Ok(Node::SUBTRACT(Box::new(left_expr), Box::new(right_expr)))
            },

            Token::MULTIPLY => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::MULDIV)?;
                Ok(Node::MULTIPLY(Box::new(left_expr), Box::new(right_expr)))
            },

            Token::DIVIDE => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::MULDIV)?;
                Ok(Node::DIVIDE(Box::new(left_expr), Box::new(right_expr)))
            },

            Token::CARET => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::POWER)?;
                Ok(Node::CARRET(Box::new(left_expr), Box::new(right_expr)))
            },

            _ => {
                Err(ParseError::InvalidOperator(format!(
                    "Please enter valid operator {:?}",
                    self.current_token
                )))
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsemath::ast::Node::*;
    
    #[test]
    fn test_parser_add() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = ADD(Box::new(NUMBER(1.0)), Box::new(NUMBER(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_parser_sub() {
        let mut parser = Parser::new("1-2").unwrap();
        let expected = SUBTRACT(Box::new(NUMBER(1.0)), Box::new(NUMBER(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_parser_mul() {
        let mut parser = Parser::new("1*2").unwrap();
        let expected = MULTIPLY(Box::new(NUMBER(1.0)), Box::new(NUMBER(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_parser_div() {
        let mut parser = Parser::new("1/2").unwrap();
        let expected = DIVIDE(Box::new(NUMBER(1.0)), Box::new(NUMBER(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_parser_caret() {
        let mut parser = Parser::new("1^2").unwrap();
        let expected = CARRET(Box::new(NUMBER(1.0)), Box::new(NUMBER(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_parser_negative() {
        let mut parser = Parser::new("-1").unwrap();
        let expected = NEGATIVE(Box::new(NUMBER(1.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }
}