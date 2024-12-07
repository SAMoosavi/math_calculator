mod element;
mod scope_marker;

use element::Element;
use scope_marker::ScopeMarker;

use std::fmt::Display;

pub struct Expiration {
    ex: Element,
}

impl Display for Expiration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.ex)
    }
}

impl Expiration {
    pub fn new(ex_str: &str) -> Result<Self, String> {
        Ok(Self {
            ex: Self::pars(ex_str)?,
        })
    }

    fn create_operator(stack: &mut Vec<Element>, operator_stack: &mut Vec<String>) {
        let right = stack.pop().unwrap();
        let operator = &operator_stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(Element::from_operator(left, operator, right));
    }

    fn pars(ex_str: &str) -> Result<Element, String> {
        let mut stack = Vec::new();
        let mut operator_stack: Vec<String> = Vec::new();

        for x in ex_str.split_whitespace() {
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

                    let check_open =
                        |stack: &mut Vec<Element>, operator_stack: &mut Vec<String>| {
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
                    stack.push(Element::from_var(x.to_string()));
                    operator_stack.push(x.to_string());
                }
                _ => match x.parse() {
                    Ok(num) => stack.push(Element::from_val(num)),
                    Err(_) => stack.push(Element::from_var(x.to_string())),
                },
            }
        }

        while stack.len() != 1 {
            Self::create_operator(&mut stack, &mut operator_stack);
        }

        Ok(stack.pop().unwrap())
    }
}
