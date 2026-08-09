#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Integer(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", if *v { "TRUE" } else { "FALSE" }),
            Value::Text(v) => write!(f, "{}", v),
        }
    }
}

impl Value {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Value::Null => bytes.push(0),
            Value::Integer(v) => {
                bytes.push(1);
                bytes.extend_from_slice(&v.to_le_bytes());
            }
            Value::Float(v) => {
                bytes.push(2);
                bytes.extend_from_slice(&v.to_le_bytes());
            }
            Value::Boolean(v) => {
                bytes.push(3);
                bytes.push(if *v { 1 } else { 0 });
            }
            Value::Text(v) => {
                bytes.push(4);
                let str_bytes = v.as_bytes();
                bytes.extend_from_slice(&(str_bytes.len() as u32).to_le_bytes());
                bytes.extend_from_slice(str_bytes);
            }
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), String> {
        if bytes.is_empty() {
            return Err("Empty byte slice".into());
        }
        match bytes[0] {
            0 => Ok((Value::Null, 1)),
            1 => {
                if bytes.len() < 9 { return Err("Invalid integer bytes".into()); }
                let v = i64::from_le_bytes(bytes[1..9].try_into().unwrap());
                Ok((Value::Integer(v), 9))
            }
            2 => {
                if bytes.len() < 9 { return Err("Invalid float bytes".into()); }
                let v = f64::from_le_bytes(bytes[1..9].try_into().unwrap());
                Ok((Value::Float(v), 9))
            }
            3 => {
                if bytes.len() < 2 { return Err("Invalid boolean bytes".into()); }
                Ok((Value::Boolean(bytes[1] != 0), 2))
            }
            4 => {
                if bytes.len() < 5 { return Err("Invalid text length bytes".into()); }
                let len = u32::from_le_bytes(bytes[1..5].try_into().unwrap()) as usize;
                if bytes.len() < 5 + len { return Err("Invalid text bytes".into()); }
                let text = String::from_utf8(bytes[5..5 + len].to_vec()).map_err(|_| "Invalid UTF8")?;
                Ok((Value::Text(text), 5 + len))
            }
            _ => Err(format!("Unknown value type tag: {}", bytes[0])),
        }
    }

    pub fn serialize_row(row: &[Value]) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Store number of columns
        bytes.extend_from_slice(&(row.len() as u16).to_le_bytes());
        for val in row {
            bytes.extend(val.to_bytes());
        }
        bytes
    }

    pub fn deserialize_row(bytes: &[u8]) -> Result<Vec<Value>, String> {
        if bytes.len() < 2 {
            return Err("Row bytes too short".into());
        }
        let num_cols = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        let mut row = Vec::with_capacity(num_cols as usize);
        let mut offset = 2;
        for _ in 0..num_cols {
            let (val, bytes_read) = Value::from_bytes(&bytes[offset..])?;
            row.push(val);
            offset += bytes_read;
        }
        Ok(row)
    }
}
