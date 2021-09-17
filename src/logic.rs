use crate::token::Token;

pub fn parse(input: &String) -> Vec<Token> {
    let input = input.as_str().chars().collect::<Vec<char>>();
    let mut output: Vec<Token> = Vec::new();
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            '+' => output.push(Token::Plus),
            '-' => output.push(Token::Minus),
            '*' => output.push(Token::Multiplication),
            '/' => output.push(Token::Division),
            '%' => output.push(Token::Modulo),
            '^' => output.push(Token::Power),
            '!' => output.push(Token::Factorial),
            '~' => output.push(Token::Negation),
            '(' => output.push(Token::OpeningParentheses),
            ')' => output.push(Token::ClosingParentheses),
            ' ' => (),
            '\n' => (),
            '\r' => (),
            c => {
                if c.is_digit(10) || c == '.' {
                    let mut number_string = String::new();
                    number_string.push(c);

                    while input[i + 1].is_digit(10) || input[i + 1] == '.' {
                        i += 1;
                        number_string.push(input[i]);
                    }

                    output.push(Token::Number(number_string.parse().expect(
                        format!("Invalid number: {}", number_string.as_str()).as_str(),
                    )))
                } else if c.is_alphabetic() {
                    let mut operator = String::new();
                    operator.push(c);

                    while input[i + 1].is_alphabetic() {
                        i += 1;
                        operator.push(input[i]);
                    }

                    operator = operator.to_lowercase();

                    match operator.as_str() {
                        "cos" => output.push(Token::Cos),
                        "sin" => output.push(Token::Sin),
                        "tan" => output.push(Token::Tan),
                        "asin" => output.push(Token::ArcSin),
                        "acos" => output.push(Token::ArcCos),
                        "atan" => output.push(Token::ArcTan),
                        "ln" => output.push(Token::NaturalLogarithm),
                        "log" => output.push(Token::Logarithm),
                        "p" => output.push(Token::Permutation),
                        "c" => output.push(Token::Combination),
                        "sqrt" => output.push(Token::SquareRoot),
                        _ => panic!("Unknown operator: {}", operator),
                    }
                } else {
                    panic!("Unknown character: {}", c);
                }
            }
        };
        i += 1;
    }

    output
}

pub fn infix_to_postfix(expression: &Vec<Token>) -> Vec<Token> {
    use Token::*;

    let mut stack = Vec::new();
    let mut result = Vec::new();

    for (i, token) in expression.iter().enumerate() {
        match token {
            Token::Number(_) => result.push(*token),
            Token::OpeningParentheses => {
                if i != 0
                    && (expression[i - 1].is_number() || expression[i - 1] == ClosingParentheses)
                {
                    stack.push(Multiplication);
                }
                stack.push(OpeningParentheses);
            }
            Token::ClosingParentheses => {
                while !stack.is_empty() && stack.last().unwrap() != &OpeningParentheses {
                    result.push(stack.pop().unwrap());
                }
                stack.pop();
            }
            _ => {
                let mut token = *token;

                if token == Minus {
                    if i == 0
                        || expression[i - 1].is_operator()
                        || expression[i - 1] == OpeningParentheses
                    {
                        token = Negation;
                    }
                }

                if (token.is_unary_operator() && token != Negation && token != Factorial)
                    && i != 0
                    && (expression[i - 1].is_number() || expression[i - 1] == ClosingParentheses)
                {
                    stack.push(Multiplication);
                }

                while !stack.is_empty()
                    && stack.last().unwrap() != &OpeningParentheses
                    && stack.last().unwrap().has_higher_precedence_than(token)
                {
                    result.push(stack.pop().unwrap());
                }

                if token == Factorial {
                    result.push(token);
                } else {
                    stack.push(token);
                }
            }
        }
    }

    while !stack.is_empty() {
        result.push(stack.pop().unwrap());
    }

    result
}

pub fn evaluate_postfix(expression: &Vec<Token>) -> f64 {
    let mut stack = Vec::new();

    for element in expression.iter() {
        if element.is_number() {
            stack.push(*element);
        } else if element.is_operator() {
            let answer;

            if element.is_unary_operator() {
                answer = unary_operation(*element, stack.pop().unwrap())
            } else {
                answer = binary_operation(*element, stack.pop().unwrap(), stack.pop().unwrap())
            }

            stack.push(Token::Number(answer));
        }
    }

    match stack.pop().unwrap() {
        Token::Number(value) => value,
        other => panic!("Invalid result: {:?}", other),
    }
}

fn unary_operation(operator: Token, operand: Token) -> f64 {
    use Token::*;

    let number: f64 = match operand {
        Number(value) => value,
        other => panic!("Invalid operand: {:?}", other),
    };

    match operator {
        Factorial => factorial(number),
        Sin => number.sin(),
        Cos => number.cos(),
        Tan => number.tan(),
        ArcSin => number.asin(),
        SquareRoot => number.sqrt(),
        Negation => number * -1.0,
        Logarithm => number.log10(),
        NaturalLogarithm => number.ln(),
        _ => panic!("Not a unary operator: {:?}", operator),
    }
}

fn binary_operation(operator: Token, operand2: Token, operand1: Token) -> f64 {
    use Token::*;

    let operand1: f64 = match operand1 {
        Number(value) => value,
        other => panic!("Invalid operand: {:?}", other),
    };
    let operand2: f64 = match operand2 {
        Number(value) => value,
        other => panic!("Invalid operand: {:?}", other),
    };

    match operator {
        Plus => operand1 + operand2,
        Minus => operand1 - operand2,
        Multiplication => operand1 * operand2,
        Division => operand1 / operand2,
        Power => operand1.powf(operand2),
        Permutation => factorial(operand1) / factorial(operand1 - operand2),
        Combination => factorial(operand1) / (factorial(operand1 - operand2) * factorial(operand2)),
        _ => panic!("Not a binary operator: {:?}", operator),
    }
}

fn factorial(n: f64) -> f64 {
    if n < 0.0 {
        panic!("Cannot find factorial of negative number: {}", n);
    } else if (n - n.floor()) != 0.0 {
        panic!("Cannot find factorial of decimal number: {}", n);
    }

    if n < 2.0 {
        return 1.0;
    } else {
        return n * factorial(n - 1.0);
    }
}
