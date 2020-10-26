
use std::collections::HashMap;

pub enum Value {
    Undefined,
    Null,
    Number(f64),
    Nan,
    String(String),
    Boolean(bool),
    Object(HashMap<String, Value>),
}
