mod addition;
mod subtraction;

use std::{char, thread};

use addition::Addition;
use subtraction::Subtraction;

pub trait Element: Send {
    fn new(left: Types, right: Types) -> Self
    where
        Self: Sized;
}


struct Var {
    var: String,
}

impl Var {
    pub fn new(var: String) -> Self {
        Self { var }
    }
}

struct Val {
    val: i32,
}

pub enum Types {
    Var(Var),
    Val(Val),
    Element(Box<dyn Element>),
}

impl Types {
    pub fn from_operator(left: Types, operator: &str, right: Types) -> Self {
        let element: Box<dyn Element> = match operator {
            "+" => Box::new(Addition::new(left, right)),
            "-" => Box::new(Subtraction::new(left, right)),
            // "*" => Some(::new(left,right)),
            // "+" => Some(Addition::new(left,right)),
            _ => panic!("Unknown Element")
        };
        Self::Element(element)
    }

    pub fn from_var(var: String) -> Self {
        Self::Var(Var::new(var))
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
}

pub struct Expiration {
    ex: String,
}

impl Expiration {
    pub fn new(ex: &str) -> Self {
        Self {
            ex: ex.to_string().replace(" ", ""),
        }
    }

    pub fn pars(&self, mut i: u8) -> Result<Types, String> {
        i += 1;
        let mut ex = &self.ex[..];
        let mut left = String::new();
        let mut right = String::new();
        let mut operator = "";
        for _ in 0..2 {
            if ex.is_empty() { break; }
            match Self::find_scope(&ex[..]) {
                Ok((start, end)) => {
                    if ex.len() != end + 1 {
                        left = (&ex[start + 1..end]).to_string();
                        operator = &ex[end + 1..end + 2];
                        ex = &ex[end + 2..];
                    } else {
                        right = ((&ex[start + 1..end])).to_string();
                    }
                }
                Err(_) => {
                    let mut buf = String::new();
                    let mut is_digit = false;
                    let mut is_alpha = false;
                    let mut end = 0;
                    for (index, c) in ex.chars().enumerate() {
                        end = index;
                        if is_digit {
                            if c.is_ascii_digit() {
                                buf.push(c);
                            } else { break; }
                        } else if is_alpha {
                            if ['+', '-', '*', '/', '^'].iter().any(|x| x == &c) {
                                break;
                            } else { buf.push(c); }
                        } else {
                            buf.push(c);
                            if c.is_ascii_digit() { is_digit = true; } else if c.is_ascii_alphabetic() { is_alpha = true; }
                        }
                    }
                    if ex.len() != end + 1 {
                        left = buf;
                        operator = &ex[end..end + 1];
                        ex = &ex[end + 1..];
                    } else {
                        right = buf;
                    }
                }
            }
        }

        if operator.is_empty() {
            if right[0..1].chars().next().unwrap().is_ascii_digit() {
                Ok(Types::from_val(right.parse().unwrap()))
            } else {
                Ok(Types::from_var(right))
            }
        } else {
            if 1 <= i && i <= 2 {
                let h1 = thread::spawn(move || Self::new(&left).pars(i));
                let h2 = thread::spawn(move || Self::new(&right).pars(i));
                let l = h1.join().unwrap();
                let r = h2.join().unwrap();
                Ok(Types::from_operator(l?, operator, r?))
            } else {
                Ok(Types::from_operator(Self::new(&left).pars(i)?, operator, Self::new(&right).pars(i)?))
            }
        }
    }

    fn find_scope(ex: &str) -> Result<(usize, usize), String> {
        let mut chars = ex.chars().enumerate();

        let (start_index, open_scope_mark) = match chars.next() {
            Some(first) => (first.0, ScopeMarker::from_char(first.1)),
            None => return Err("Expiration::find_scope: The input string is empty".to_string()),
        };

        if open_scope_mark == ScopeMarker::Unknown {
            return Err(format!(
                "Expiration::find_scope: The {ex} does not start with a valid ScopeMarker!"
            ));
        }

        let mut number_of_scope = 1;

        for (index, c) in chars {
            let scope_mark = ScopeMarker::from_char(c);

            if scope_mark.same_scope(&open_scope_mark) {
                number_of_scope -= 1;
            } else if scope_mark == open_scope_mark {
                number_of_scope += 1;
            }

            if number_of_scope == 0 {
                return Ok((start_index, index));
            }
        }
        Err("Expiration::find_scope: The scope is not correct!".to_string())
    }
}
