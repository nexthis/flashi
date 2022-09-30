
#[derive(Debug, Clone)]
pub enum NumericTypes {
    Decimal,
    Floating,
    Percentage,
}

#[derive(Debug)]
pub enum Token {
    /** End of line */
    EOL,

    /** A sequence of characters */
    String(String),

    /** Operation are 'actions' that you take on an entity i.e. '*', '+', '->' */
    Operator(String),

    /** A single characters */
    Char(char),

    /** A sequence of characters to have number representation  */
    Numeric { raw: String, variant: NumericTypes },
}

impl  Clone for  Token{
    fn clone(&self) -> Self {
        match self {
            Self::EOL => Self::EOL,
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Operator(arg0) => Self::Operator(arg0.clone()),
            Self::Char(arg0) => Self::Char(arg0.clone()),
            Self::Numeric { raw, variant } => Self::Numeric { raw: raw.clone(), variant: variant.clone() },
        }
    }
}
