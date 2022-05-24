use crate::base_gates::gate_type::GateType;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum GateError {
    BusTooBig(u64),
    NotGateGT1Input(usize),
    DoubleEntryGateLT2Inputs(GateType),
    GateInputSmallerThanGiven(usize, usize),
}

impl fmt::Display for GateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GateError::BusTooBig(s) => write!(f, "Invalid bus size: {}; the max is 64 bits.", s),
            GateError::NotGateGT1Input(i) => write!(
                f,
                "Not gate set with {} inputs; it can only operate on 1.",
                i
            ),
            GateError::DoubleEntryGateLT2Inputs(t) => {
                write!(f, "{} gate set with 1 input; at least 2 are needed.", t)
            }
            GateError::GateInputSmallerThanGiven(size, given) => {
                write!(f, "Gate has {} inputs, but {} were set", size, given)
            }
        }
    }
}

impl error::Error for GateError {}
