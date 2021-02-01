use std::io;
mod parsemath;
use parsemath::ast;
use parsemath::parser::{ParseError, Parser};

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}

fn main() {
    println!("Arithmetic Expression Evaluator.");
    println!("You can calculate value for expression such as 2*3+(4-5)+2^3/4.");
    println!("Allowed numbers are: Positive, Negative and Decimals");
    println!("Supported operands: Add, Subtract, Multiply, Divide, Powerof(^).");
    println!("Enter your arithmetic expression below:");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is: {}", val),
                    Err(_) => {
                        println!("Error evaluating expression. Please enter a valid expression\n")
                    }
                };
            }
            Err(error) => println!("error {}", error),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_main_evaluate() {
        let result = evaluate(String::from("5+5+10")).unwrap();
        let expected = 20.0;
        assert_eq!(result, expected)
    }
}