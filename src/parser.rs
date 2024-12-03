mod addition;
mod subtraction;

mod power;

mod division;
mod multiplication;

use addition::Addition;
use division::Division;
use multiplication::Multiplication;
use power::Power;
use std::char;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use subtraction::Subtraction;

pub trait Element: Display {
    fn new(left: Types, right: Types) -> Self
    where
        Self: Sized;
}

struct Var {
    var: String,
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.var)
    }
}

struct Val {
    val: i32,
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

pub enum Types {
    Var(Var),
    Val(Val),
    Element(Box<dyn Element>),
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Var(x) => {
                write!(f, "{}", x)
            }
            Types::Val(x) => {
                write!(f, "{}", x)
            }
            Types::Element(x) => {
                write!(f, "{}", x)
            }
        }
    }
}

impl Types {
    pub fn from_operator(left: Types, operator: &str, right: Types) -> Self {
        let element: Box<dyn Element> = match operator {
            "+" => Box::new(Addition::new(left, right)),
            "-" => Box::new(Subtraction::new(left, right)),
            "*" => Box::new(Multiplication::new(left, right)),
            "/" => Box::new(Division::new(left, right)),
            "^" => Box::new(Power::new(left, right)),
            _ => panic!("Unknown Element"),
        };
        Self::Element(element)
    }

    pub fn from_var(var: String) -> Self {
        Self::Var(Var { var })
    }

    pub fn from_val(val: i32) -> Self {
        Self::Val(Val { val })
    }
}

#[derive(PartialEq)]
enum ScopeMarker {
    OpenParenthesis,  // "("
    CloseParenthesis, // ")"
    OpenCurlyBrace,   // "{"
    CloseCurlyBrace,  // "}"
    OpenBracket,      // "["
    CloseBracket,     // "]"
    Unknown,          // For unsupported characters
}

impl ScopeMarker {
    pub fn from_char(c: char) -> Self {
        match c {
            '(' => Self::OpenParenthesis,
            ')' => Self::CloseParenthesis,
            '{' => Self::OpenCurlyBrace,
            '}' => Self::CloseCurlyBrace,
            '[' => Self::OpenBracket,
            ']' => Self::CloseBracket,
            _ => Self::Unknown,
        }
    }

    pub fn from_str(c: &str) -> Self {
        match c {
            "(" => Self::OpenParenthesis,
            ")" => Self::CloseParenthesis,
            "{" => Self::OpenCurlyBrace,
            "}" => Self::CloseCurlyBrace,
            "[" => Self::OpenBracket,
            "]" => Self::CloseBracket,
            _ => Self::Unknown,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::OpenParenthesis => '(',
            Self::CloseParenthesis => ')',
            Self::OpenCurlyBrace => '{',
            Self::CloseCurlyBrace => '}',
            Self::OpenBracket => '[',
            Self::CloseBracket => ']',
            Self::Unknown => '\0',
        }
    }

    pub fn same_scope(&self, other: &Self) -> bool {
        match self {
            Self::OpenParenthesis => other == &Self::CloseParenthesis,
            Self::CloseParenthesis => other == &Self::OpenParenthesis,
            Self::OpenCurlyBrace => other == &Self::CloseCurlyBrace,
            Self::CloseCurlyBrace => other == &Self::OpenCurlyBrace,
            Self::OpenBracket => other == &Self::CloseBracket,
            Self::CloseBracket => other == &Self::OpenBracket,
            _ => false,
        }
    }

    pub fn is_close(&self) -> bool {
        match self {
            ScopeMarker::CloseParenthesis
            | ScopeMarker::CloseCurlyBrace
            | ScopeMarker::CloseBracket => true,
            _ => false,
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            ScopeMarker::OpenParenthesis
            | ScopeMarker::OpenCurlyBrace
            | ScopeMarker::OpenBracket => true,
            _ => false,
        }
    }
}

pub struct Expiration {
    ex: String,
}

impl Expiration {
    pub fn new(ex: String) -> Self {
        Self { ex }
    }

    pub fn pars(&self) -> Result<Types, String> {
        let mut stack = Vec::new();
        let mut operator_stack: Vec<String> = Vec::new();

let s = Instant::now();
        let mut  a = 0;
        for x in self.ex.split_whitespace() {
            a += 1;
            match x {
                "-" | "+" => {
                    while (|| {
                        if let Some(x) = operator_stack.last() {
                            match &x[..] {
                                "*" | "-" | "^" => true,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    })() {
                        let right = stack.pop().unwrap();
                        let operator = &operator_stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(Types::from_operator(left, operator, right));
                    }
                    operator_stack.push(x.to_string());
                }
                "*" | "/" => {
                    while (|| {
                        if let Some(x) = operator_stack.last() {
                            match &x[..] {
                                "^" => true,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    })() {
                        let right = stack.pop().unwrap();
                        let operator = &operator_stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(Types::from_operator(left, operator, right));
                    }
                    operator_stack.push(x.to_string());
                }
                "^" => {
                    operator_stack.push(x.to_string());
                }
                x if ScopeMarker::is_close(&ScopeMarker::from_str(x)) => {
                    let scope = ScopeMarker::from_str(x);
                    while (|| {
                        if let Some(x) = operator_stack.last() {
                            let current_scope = ScopeMarker::from_str(x);
                            if scope.same_scope(&current_scope) {
                                operator_stack.pop().unwrap();
                                let lase_element = stack.pop().unwrap();
                                let len = stack.len();
                                stack[len - 1] = lase_element;
                                false
                            } else {
                                true
                            }
                        } else {
                            false
                        }
                    })() {
                        let right = stack.pop().unwrap();
                        let operator = &operator_stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(Types::from_operator(left, operator, right));
                    }
                }
                x if ScopeMarker::is_open(&ScopeMarker::from_str(x)) => {
                    stack.push(Types::from_var(x.to_string()));
                    operator_stack.push(x.to_string());
                }
                _ => match x.parse() {
                    Ok(num) => stack.push(Types::from_val(num)),
                    Err(_) => stack.push(Types::from_var(x.to_string())),
                },
            }
        }

        while stack.len() != 1 {
            let right = stack.pop().unwrap();
            let operator = &operator_stack.pop().unwrap();
            let left = stack.pop().unwrap();
            stack.push(Types::from_operator(left, operator, right));
        }
let e = Instant::now();

        Err(format!("Parsing error: {} {:?}",a ,  e - s))
    }
}
