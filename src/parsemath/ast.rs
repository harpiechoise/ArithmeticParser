use std::error;

/// The node enum hold all the operation variant to make the sintax tree
/// # Arguments
/// * `left: Box<Node>` - the left side of an operation
/// * `right: Box<Node>` - the right side of an operation
/// # Example 
/// ```
/// // To represent an adition we can use the addition variant with two numeric values
/// let left = Box::new(Node::Number(5.0));
/// let right = Box::new(Node::(NUMBER(5.0)));
/// addition = Node::ADD(left, right) // This will reperesent an addition node for the AST
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    ADD(Box<Node>, Box<Node>),
    SUBTRACT(Box<Node>, Box<Node>),
    MULTIPLY(Box<Node>, Box<Node>),
    DIVIDE(Box<Node>, Box<Node>),
    CARRET(Box<Node>, Box<Node>),
    NEGATIVE(Box<Node>),
    NUMBER(f64) // All the numbers are treated like f64
}

/// The eval function takes an operation node and resolve the operation if we take an addition node
/// for example: `Node::ADD(left, right)` we can evaluate the addition with this function 
/// # Arguments
/// * `expr: Node` - Is a node representing an operation node, number node or negative node
/// 
/// # Returns
/// * `Result<f64, Box<dyn error::Error>>` - the eval function returns a `Ok(number)` or `Err(err)` 
///
/// 
/// # Example
/// ```
/// // We create an addition node
/// let addition = Node::ADD(Box::new(Node::Number(5.0)), Box::new(Node::Number(5.0)))
/// let evaluated = eval(addition_node); // This should return a result with Ok(10.0)
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        // If we have a number we return the value
        NUMBER(value) => Ok(value),
        // If we have an operation node we extract the values and evaluate them
        ADD(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?), 
        SUBTRACT(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        MULTIPLY(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        DIVIDE(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        CARRET(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        // If we have a negative number, we evaluate to extract the value
        // and we apply the "-" operation
        NEGATIVE(expr1) => Ok(-(eval(*expr1)?)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn get_operation(token_symbol: &str) -> Node {
        use Node::*;
        if token_symbol == "+" {
            return ADD(Box::new(NUMBER(5.0)), Box::new(NUMBER(5.0)));
        } else if token_symbol == "-" {
            return SUBTRACT(Box::new(NUMBER(5.0)), Box::new(NUMBER(5.0)));
        } else if token_symbol == "*" {
            return MULTIPLY(Box::new(NUMBER(5.0)), Box::new(NUMBER(5.0)));
        } else if token_symbol == "/" {
            return DIVIDE(Box::new(NUMBER(5.0)), Box::new(NUMBER(5.0)));
        } else if token_symbol == "^" {
            return CARRET(Box::new(NUMBER(5.0)), Box::new(NUMBER(5.0)));
        } else {
            return NEGATIVE(Box::new(NUMBER(5.0)))
        }
    }
    #[test]
    fn test_ast_node_addition() {
        let node = get_operation("+");
        let evaluated = eval(node).unwrap();
        assert_eq!(evaluated,  10.0)
    }

    #[test]
    fn test_ast_node_subtraction() {
        let node = get_operation("-");
        let evaluated = eval(node).unwrap();
        assert_eq!(evaluated, 0.0)
    }
    #[test]
    fn test_ast_node_multiplitation() {
        let node = get_operation("*");
        let evaluated = eval(node).unwrap();
        assert_eq!(evaluated, 25.0)
    }

    #[test]
    fn test_ast_node_division() {
        let node = get_operation("/");
        let evaluated = eval(node).unwrap();
        assert_eq!(evaluated, 1.0)
    }

    #[test]
    fn test_ast_node_power() {
        let node = get_operation("^");
        let evaluated = eval(node).unwrap();
        assert_eq!(evaluated, 3125.0);
    }

    #[test]
    fn test_ast_node_negative() {
        let node = get_operation("0");
        let evaluated = eval(node).unwrap();
        assert_eq!(evaluated, -5.0);
    }
}