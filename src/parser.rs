use std::{fmt::{Display, Formatter},cmp::max};

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

struct BinaryOperator {
    left: Box<Types>,
    right: Box<Types>,
    depth: u32,
}

pub enum Types {
    Var(Var),
    Val(Val),
    Add(BinaryOperator),
    Subtract(BinaryOperator),
    Multiple(BinaryOperator),
    Division(BinaryOperator),
    Power(BinaryOperator),
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
            Types::Add(x) => {
                write!(f, "({} + {})", x.left, x.right)
            }
            Types::Subtract(x) => {
                write!(f,"({} - {})",x.left, x.right)
            }
            Types::Multiple(x) => {
                write!(f,"({} * {})",x.left, x.right)
            }
            Types::Division(x) => {
                write!(f,"({} / {})",x.left, x.right)
            }
            Types::Power(x) => {
                write!(f,"({} ^ {})",x.left, x.right)
            }
        }
    }
}

impl Types {
    pub fn from_operator(left: Types, operator: &str, right: Types) -> Self {
        let depth = max(left.get_depth(), right.get_depth()) + 1;
        let left = Box::new(left);
        let right = Box::new(right);
        let binary_operator = BinaryOperator {
            left,
            right,
            depth,
        };
        match operator {
            "+" => Types::Add(binary_operator),
            "-" => Types::Subtract(binary_operator),
            "*" => Types::Multiple(binary_operator),
            "/" => Types::Division(binary_operator),
            "^" => Types::Power(binary_operator),
            _ => panic!("Unknown Element"),
        }
    }

    pub fn from_var(var: String) -> Self {
        Self::Var(Var { var })
    }

    pub fn from_val(val: i32) -> Self {
        Self::Val(Val { val })
    }

    fn get_depth(&self) -> u32 {
        match self {
            Types::Val(_) => {0},
            Types::Var(_) => {0},
            Types::Add(x) => x.depth,
            Types::Subtract(x) => x.depth,
            Types::Multiple(x) => x.depth,
            Types::Division(x) => x.depth,
            Types::Power(x) => x.depth,
        }
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
        matches!(
            self,
            ScopeMarker::CloseParenthesis
                | ScopeMarker::CloseCurlyBrace
                | ScopeMarker::CloseBracket
        )
    }

    pub fn is_open(&self) -> bool {
        matches!(
            self,
            ScopeMarker::OpenParenthesis | ScopeMarker::OpenCurlyBrace | ScopeMarker::OpenBracket
        )
    }
}

pub struct Expiration {
    ex: String,
}

impl Expiration {
    pub fn new(ex: String) -> Self {
        Self { ex }
    }

    fn create_operator(stack: &mut Vec<Types>, operator_stack: &mut Vec<String>) {
        let right = stack.pop().unwrap();
        let operator = &operator_stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(Types::from_operator(left, operator, right));
    }

    pub fn pars(&self) -> Result<Types, String> {
        let mut stack = Vec::new();
        let mut operator_stack: Vec<String> = Vec::new();

        for x in self.ex.split_whitespace() {
            match x {
                "-" | "+" => {
                    let check_precedence = |operator_stack: &Vec<String>| {
                        if let Some(x) = operator_stack.last() {
                            matches!(x.as_str(), "*" | "-" | "^")
                        } else {
                            false
                        }
                    };

                    while check_precedence(&operator_stack) {
                        Self::create_operator(&mut stack, &mut operator_stack);
                    }
                    operator_stack.push(x.to_string());
                }
                "*" | "/" => {
                    let check_precedence = |operator_stack: &Vec<String>| {
                        if let Some(x) = operator_stack.last() {
                            matches!(x.as_str(), "^")
                        } else {
                            false
                        }
                    };
                    while check_precedence(&operator_stack) {
                        Self::create_operator(&mut stack, &mut operator_stack);
                    }
                    operator_stack.push(x.to_string());
                }
                "^" => {
                    operator_stack.push(x.to_string());
                }
                x if ScopeMarker::is_close(&ScopeMarker::from_str(x)) => {
                    let scope = ScopeMarker::from_str(x);

                    let check_open = |stack: &mut Vec<Types>, operator_stack: &mut Vec<String>| {
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
                    };

                    while check_open(&mut stack, &mut operator_stack) {
                        Self::create_operator(&mut stack, &mut operator_stack);
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
            Self::create_operator(&mut stack, &mut operator_stack);
        }

        Ok(stack.pop().unwrap())
    }
}
