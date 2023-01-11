use std::str::FromStr;

#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub enum ParseValueError {
    InvalidValue,
}

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.starts_with('"') && string.ends_with('"') {
            return Ok(Value::Str(string[1..string.len() - 1].into()));
        }

        if let Ok(i) = string.parse::<i64>() {
            return Ok(Value::Int(i));
        }

        if let Ok(f) = string.parse::<f64>() {
            return Ok(Value::Float(f));
        }

        if let Ok(b) = string.parse::<bool>() {
            return Ok(Value::Bool(b));
        }

        if string == "nil" {
            return Ok(Value::Nil);
        }

        Err(ParseValueError::InvalidValue)
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Str(s) => format!("s:{}", s),
            Value::Int(i) => format!("i:{}", i),
            Value::Float(f) => format!("f:{}", f),
            Value::Bool(b) => format!("b:{}", b),
            Value::Nil => "n:nil".into(),
        }
    }
}
