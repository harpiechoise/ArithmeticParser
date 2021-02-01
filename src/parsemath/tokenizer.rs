/// This module holds the **Tokenizer** Structure, the tokenizer is responsible for convert the
/// characters to tokens and give it to the Parser to construct the AST (Abstract Sintax Tree)


use std::{
    str::Chars, 
    iter::Peekable};
use super::token::Token;

/// The tokenizer struct holds all the methods to take the text and convert him to tokens
pub struct Tokenizer<'a> {
    // The pekeeable is an iterator with the method peek that pop the first element in the stack
    expr: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
    /// Returns a new instance of the Tokenizer struct
    /// # Arguments
    /// * `new_expr` - A string containing an Arithmetical Expression
    /// 
    /// # Returns 
    /// * `Tokenizer` - A new instance of a tokenizer object
    /// # Examples
    /// 
    /// ```
    ///
    /// use parsemat::tokenizer::Tokenizer;
    /// // We create a new Tokenizer holding the expression 42
    /// let tokenizer = Tokenizer::new("42")
    /// ```
    
    // We take a lifetime rule for prevent borrowing
    // When the variable goes out of scope
    pub fn new(new_expr: &'a str) -> Self {
        
        Tokenizer {
            // We convert the input expr to a peekeable
            expr: new_expr.chars().peekable(),
        }
    }

    /// Peeks a single character or a number and return a Token Variant
    /// # Examples
    /// ```
    /// use parsemath::tokenizer::Tokenizer;
    /// let tokenizer = Tokenizer::new("42");
    /// let token = tokenizer.next()?;
    /// // The token would be Token::NUM(42.0)
    pub fn next(&mut self) -> Option<Token> {
        // We take the next character in the stack and we store it into a variable
        let next_char = self.expr.next();
        match next_char {
            // If the next char is a number
            Some('0'..='9') => {
                // We store the value of the number in a variable
                // And we unwrap it and send the error with the option 
                // Type, for that we use de '?' operator
                let mut number = next_char?.to_string();
                // if the next value is a number we parse until this the next character be a 
                // Symbol
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        // If is a number or a decimal point we push it to the number String
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                // We return a option type with the token
                Some(Token::NUM(number.parse::<f64>().unwrap()))
            }
            // if the token are not numeric
            // We tokenize the mathematical symbol
            Some('+') => Some(Token::ADD),
            Some('-') => Some(Token::SUBTRACT),
            Some('*') => Some(Token::MULTIPLY),
            Some('/') => Some(Token::DIVIDE),
            Some('^') => Some(Token::CARET),
            Some('(') => Some(Token::LEFTPAREN),
            Some(')') => Some(Token::RIGHTPAREN),
            // If there is no more symbols we send a End-Of-File Indication to the parser
            None => Some(Token::EOF),
            // Whatever other symbol is and this isn't a token we return None
            Some(_) => None,
        }
    } 
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_number_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        let num = match tokenizer.next() {
            Some(value) => match value {
                Token::NUM(value) => value,
                _ => 0.0
            },
            None => -32.0
        };
        assert_eq!(num, 34.0)
    }

    #[test]
    fn test_number_decimal() {
        let mut tokenizer = Tokenizer::new("34.4");
        let num = match tokenizer.next() {
            Some(value) => match value {
                Token::NUM(value) => value,
                _ => 0.0
            },
            None => -60.0
        };
        assert_eq!(num, 34.4)
    }
    
    #[test]
    fn test_token_divide() {
        let mut tokenizer = Tokenizer::new("/");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::DIVIDE, token);
    }

    #[test]
    fn test_token_multiply() {
        let mut tokenizer = Tokenizer::new("*");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::MULTIPLY, token);
    }

    #[test]
    fn test_token_add() {
        let mut tokenizer = Tokenizer::new("+");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::ADD, token);
    }

    #[test]
    fn test_token_subtract() {
        let mut tokenizer = Tokenizer::new("-");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::SUBTRACT, token);
    }

    #[test]
    fn test_token_caret() {
        let mut tokenizer = Tokenizer::new("^");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::CARET, token);
    }

    #[test]
    fn test_token_rparent() {
        let mut tokenizer = Tokenizer::new(")");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::RIGHTPAREN, token);
    }
    #[test]
    fn test_token_lparent() {
        let mut tokenizer = Tokenizer::new("(");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::LEFTPAREN, token);
    }
    
    #[test]
    fn test_token_eof() {
        let mut tokenizer = Tokenizer::new("");
        let token = match tokenizer.next() {
            Some(token) => token,
            None => Token::EOF
        };
        assert_eq!(Token::EOF, token);
    }

}