use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
pub enum TypeError {
    #[display(fmt = "{}", _0)]
    ParseError(String),
}
