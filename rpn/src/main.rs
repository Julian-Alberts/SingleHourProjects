use std::cmp::Ordering;

fn main() {
    println!("{:#?}", eval_rpn(to_rpn(parse("10/5*2"))));
    println!("{:#?}", eval_rpn(to_rpn(parse("10*2/4"))));
    println!("{:#?}", eval_rpn(to_rpn(parse("(5*4+3*2)-1"))));
}

fn eval_rpn(rpn: Vec<Token>) -> f64 {
    let mut stack = Vec::new();
    for token in rpn {
        match &token {
            Token::Number(num) => stack.push(*num),
            Token::Operation(op) => {
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                let result = match op {
                    Operation::Div => op1 / op2,
                    Operation::Mul => op1 * op2,
                    Operation::Add => op1 + op2,
                    Operation::Sub => op1 - op2,
                };
                stack.push(result);
            }
            Token::OpeningBracket | Token::ClosingBracket => unreachable!()
        }
    }
    stack.pop().unwrap()
}

fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack = Vec::new();
    let mut queue = Vec::with_capacity(tokens.len());

    for token in tokens {
        match (&token, stack.last()) {
            (_, Some(Token::Number(_)) | Some(Token::ClosingBracket)) => unreachable!(),
            (Token::Number(_), _) => queue.push(token),
            (Token::Operation(op), Some(Token::Operation(prev))) => {
                if *prev <= *op {
                    let prev = stack.pop().unwrap();
                    stack.push(token);
                    queue.push(prev);
                } else {
                    stack.push(token);
                }
            },
            (Token::Operation(_), None | Some(Token::OpeningBracket)) => {
                stack.push(token)
            }
            (Token::OpeningBracket, _) => stack.push(Token::OpeningBracket),
            (Token::ClosingBracket, _) => {
                while stack.last() != Some(&Token::OpeningBracket) {
                    queue.push(stack.pop().unwrap())
                }
                stack.pop();
            }
        }
    }

    while !stack.is_empty() {
        queue.push(stack.pop().unwrap());
    }
    queue
}

fn parse(input: &str) -> Vec<Token> {
    let input = input.chars().collect::<Vec<_>>();
    let mut input = input.as_slice();
    let mut list = Vec::new();
    while input.len() > 0 {
        if input[0] == '(' {
            list.push(Token::OpeningBracket);
            input = &input[1..];
            if input.len() <= 0 {
                break
            }
        }

        let (num, remaining) = parse_num(input);
        input = remaining;
        list.push(Token::Number(num));
        if input.len() <= 0 {
            break
        }

        if input[0] == ')' {
            list.push(Token::ClosingBracket);
            input = &input[1..];
            if input.len() <= 0 {
                break
            }
        }

        let op = match input[0] {
            '+' => Operation::Add,
            '-' => Operation::Sub,
            '*' => Operation::Mul,
            '/' => Operation::Div,
            t => panic!("{t:#?}")
        };
        list.push(Token::Operation(op));
        if input.len() > 0 {
            input = &input[1..];
        }
    }
    list
}

fn parse_num(input: &[char]) -> (f64, &[char]) {
    let mut index = 0;
    while let '0'..='9'|'.' = input[index] {
        index += 1;
        if index >= input.len() {
            index = input.len();
            break;
        }
    }
    let num = String::from_iter(&input[..index]).parse().expect("Invalid number");
    (num, &input[index..])
}


#[derive(Debug, PartialEq)]
pub enum Token {
    Operation(Operation),
    Number(f64),
    OpeningBracket,
    ClosingBracket
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Div,
    Mul,
    Add,
    Sub,
}

impl PartialOrd for Operation {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Div | Self::Mul, Self::Div | Self::Mul) => Some(Ordering::Equal),
            (Self::Div | Self::Mul, Self::Add | Self::Sub) => Some(Ordering::Less),
            (Self::Add | Self::Sub, Self::Add | Self::Sub) => Some(Ordering::Equal),
            (Self::Add | Self::Sub, Self::Div | Self::Mul) => Some(Ordering::Greater),
        }
    }

}