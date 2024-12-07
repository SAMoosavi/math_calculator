#[derive(PartialEq)]
pub enum ScopeMarker {
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
