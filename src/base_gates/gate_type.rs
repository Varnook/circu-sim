use std::fmt;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum GateType {
    Not,
    And,
    Or,
    Xor,
}

impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GateType::Not => write!(f, "Not"),
            GateType::And => write!(f, "And"),
            GateType::Or => write!(f, "Or"),
            GateType::Xor => write!(f, "Xor"),
        }
    }
}
