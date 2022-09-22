#[derive(Debug)]
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
