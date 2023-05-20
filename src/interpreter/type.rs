#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Z,
    String,
    Function,
    Custom(String),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "int" => Self::Int,
            "z" => Self::Z,
            "string" => Self::String,
            "fctn" => Self::Function,
            _ => Self::Custom(value.to_owned()),
        }
    }
}
