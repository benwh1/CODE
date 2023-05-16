#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integer,
    String,
    Custom(String),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value.as_ref() {
            "int" => Self::Integer,
            "string" => Self::String,
            _ => Self::Custom(value.to_owned()),
        }
    }
}
