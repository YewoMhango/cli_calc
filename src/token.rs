#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiplication,
    Division,
    Modulo,
    Power,
    SquareRoot,
    Combination,
    Permutation,
    Logarithm,
    NaturalLogarithm,
    ArcTan,
    ArcCos,
    ArcSin,
    Tan,
    Sin,
    Cos,
    Factorial,
    Negation,
    Number(f64),
    OpeningParentheses,
    ClosingParentheses,
}

impl Token {
    /// Returns true if `self` is an instance of a unary operator
    ///
    /// # Example
    ///```
    /// let open_bracket = Token::OpeningParentheses;
    /// let factorial = Token::Factorial;
    ///
    /// assert(factorial.is_operator());
    /// assert(!open_bracket.is_operator());
    ///```
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Number(_) => false,
            Token::ClosingParentheses => false,
            Token::OpeningParentheses => false,
            _ => true,
        }
    }

    /// Returns true if `self` is an instance of a unary operator
    ///
    /// # Example
    ///```
    /// let plus = Token::Plus;
    /// let factorial = Token::Factorial;
    ///
    /// assert(factorial.is_unary_operator());
    /// assert(!plus.is_unary_operator());
    ///```
    pub fn is_unary_operator(&self) -> bool {
        use Token::*;

        const UNARY_OPERATORS: [Token; 11] = [
            Negation,
            Factorial,
            Cos,
            Sin,
            Tan,
            ArcCos,
            ArcSin,
            ArcTan,
            Logarithm,
            NaturalLogarithm,
            SquareRoot,
        ];

        UNARY_OPERATORS.contains(&self)
    }

    /// Returns true if `self` is an instance of `Token::Number`
    ///
    /// # Example
    ///```
    /// let three = Token::Number(3);
    /// let plus = Token::Plus;
    ///
    /// assert!(three.is_number());
    /// assert!(!plus.is_number());
    ///```
    pub fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            _ => false,
        }
    }

    /// Returns true if `self` has higher precedence than `other`
    ///
    /// # Example
    ///```
    /// use Token::*;
    ///
    /// assert(SquareRoot.has_higher_precedence_than(Plus));
    /// assert(!Tan.has_higher_precedence_than(Negation));
    ///```
    pub fn has_higher_precedence_than(&self, other: Token) -> bool {
        use Token::*;

        const ORDER: [Token; 19] = [
            Negation,
            Factorial,
            Cos,
            Sin,
            Tan,
            ArcCos,
            ArcSin,
            ArcTan,
            NaturalLogarithm,
            Logarithm,
            Permutation,
            Combination,
            SquareRoot,
            Power,
            Division,
            Multiplication,
            Modulo,
            Plus,
            Minus,
        ];

        let index_of = |token: Token| {
            for (i, val) in ORDER.iter().enumerate() {
                if val == &token {
                    return i as i32;
                }
            }

            return -1;
        };

        if !ORDER.contains(&self) {
            panic!("Invalid operator: {:?}", self);
        }
        if !ORDER.contains(&other) {
            panic!("Invalid operator: {:?}", other);
        }

        index_of(*self) < index_of(other)
    }
}
