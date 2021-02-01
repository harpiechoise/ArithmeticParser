use std::cmp::PartialEq;

/// The Token struct holds the token type for a specific symbol or number
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    CARET,
    LEFTPAREN,
    RIGHTPAREN,
    NUM(f64), // If the value is numeric we store the number in an Enum Variant
    EOF,
}

/// The OpenPrec enum holds the operator precendence and allow to compare with ordering 
/// opreratos like "<" or ">" the values of the tokens are 
/// - DEFUALTZERO: 0
/// - ADDSUB: 1 (Adition Subtraction)
/// - MULTDIV: 2 (Multiplication Division)
/// - POWER: 3 (Pow operation)
/// - NEGATIVE: 4 (-5 or -(Token::NUM))
#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DEFAULTZERO,
    ADDSUB,
    MULDIV,
    POWER,
    NEGATIVE
}

impl Token {
    /// This method allow to get the precedence from a certain operation depending of the enum variant
    /// # Retuns
    /// `OperPrec` - An `OperPrec` enum variant
    /// # Example
    /// ```
    /// use parsemath::token::Token;
    /// let token = Token::ADD;
    /// let oper_prec = token.get_oper_prec()
    /// // This will be OperPrec::ADDSUB
    /// ```
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            ADD | SUBTRACT => ADDSUB,
            MULTIPLY | DIVIDE => MULDIV,
            CARET => POWER,
            _ => DEFAULTZERO,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oper_prec_addition_subtraction_addition() {
        let token = Token::ADD.get_oper_prec();
        assert_eq!(token, OperPrec::ADDSUB);
    }

    #[test]
    fn test_oper_prec_addition_subtraction_aubtraction() {
        let token = Token::SUBTRACT.get_oper_prec();
        assert_eq!(token, OperPrec::ADDSUB);
    }

    #[test]
    fn test_oper_prec_multiplication_division_division() {
        let token = Token::DIVIDE.get_oper_prec();
        assert_eq!(token, OperPrec::MULDIV);
    }

    #[test]
    fn test_oper_prec_multiplication_division_multiplication() {
        let token = Token::MULTIPLY.get_oper_prec();
        assert_eq!(token, OperPrec::MULDIV);
    }

    #[test]
    fn test_oper_prec_addition_carrent() {
        let token = Token::CARET.get_oper_prec();
        assert_eq!(token, OperPrec::POWER);
    }

    #[test]
    fn test_oper_prec_default_zero() {
        let token = Token::NUM(25.0).get_oper_prec();
        assert_eq!(token, OperPrec::DEFAULTZERO)
    }

    #[test]
    fn test_oper_prec_addition_subtraction_mult() {
        let add = OperPrec::ADDSUB;
        let mult = OperPrec::MULDIV;
        assert!(mult > add);
    }
    
    #[test]
    fn test_oper_prec_addition_mult_power() {
        let mult = OperPrec::MULDIV;
        let power = OperPrec::POWER;
        assert!(power > mult);
    }
}