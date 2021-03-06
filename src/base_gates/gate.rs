use crate::base_gates::gate_error::GateError;
use crate::base_gates::gate_type::GateType;
use std::convert;

#[derive(PartialEq, Eq, Hash)]
pub struct Gate {
    gate_type: GateType,
    input_size: usize,
    inputs: Vec<u64>,
    bus_size_mask: u64,
    output: u64,
}

impl Gate {
    pub fn new(gate_type: GateType, input_size: usize, bus_size: u64) -> Result<Gate, GateError> {
        if bus_size > 64 {
            return Err(GateError::BusTooBig(bus_size));
        }

        if input_size > 1 && gate_type == GateType::Not {
            return Err(GateError::NotGateGT1Input(input_size));
        }

        if input_size < 2 && gate_type != GateType::Not {
            return Err(GateError::DoubleEntryGateLT2Inputs(gate_type));
        }

        let inputs = vec![0; input_size];
        let bus_size_mask = !(!0 << bus_size);
        let mut g = Gate {
            gate_type,
            input_size,
            inputs,
            bus_size_mask,
            output: 0,
        };

        g.logic();
        Ok(g)
    }

    pub fn set_inputs(&mut self, inputs: Vec<(usize, u64)>) -> Result<(), GateError> {
        if self.input_size < inputs.len() {
            return Err(GateError::GateInputSmallerThanGiven(
                self.input_size,
                inputs.len(),
            ));
        }

        for (i, val) in inputs {
            self.inputs[i] = val;
        }
        self.logic();
        Ok(())
    }

    pub fn get_output(&self) -> u64 {
        self.output
    }

    fn logic(&mut self) {
        let operation = match self.gate_type {
            GateType::And => |a, b| a & b,
            GateType::Or => |a, b| a | b,
            GateType::Xor => |a, b| a ^ b,
            GateType::Not => |a, _b| a,
        };

        if self.gate_type == GateType::Not {
            self.output = !self.inputs[0] & self.bus_size_mask;
        } else {
            self.output =
                self.inputs[1..].iter().fold(self.inputs[0], operation) & self.bus_size_mask;
        };
    }
}

impl<const N: usize> TryFrom<(GateType, usize, u64, [u64; N])> for Gate {
    type Error = GateError;
    fn try_from(data: (GateType, usize, u64, [u64; N])) -> Result<Self, GateError> {
        let (gate_type, input_size, bus_size, inputs) = data;
        if input_size < inputs.len() {
            return Err(GateError::GateInputSmallerThanGiven(
                input_size,
                inputs.len(),
            ));
        }

        let mut g = Gate::new(gate_type, input_size, bus_size)?;
        for (i, input) in inputs.iter().enumerate() {
            g.inputs[i] = *input;
        }

        g.logic();
        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_gate() {
        let g = Gate::new(GateType::And, 2, 8).unwrap();
        assert_eq!(g.gate_type, GateType::And);
        assert_eq!(g.input_size, 2);
        assert_eq!(g.bus_size_mask, 255);
        assert_eq!(g.inputs.len(), g.input_size);
        for input in g.inputs.iter() {
            assert_eq!(input, &0);
        }
    }

    #[test]
    fn new_not_gate() {
        let g = Gate::new(GateType::Not, 1, 4).unwrap();
        assert_eq!(g.output, 15);
    }

    #[test]
    fn set_2_input_gate() {
        let mut g = Gate::new(GateType::And, 2, 16).unwrap();
        g.set_inputs([(0, 0xFF33), (1, 0x33B3)].to_vec()).unwrap();
        assert_eq!(g.output, 0x3333);
    }

    #[test]
    fn set_3_input_gate() {
        let mut g = Gate::new(GateType::And, 3, 4).unwrap();
        g.set_inputs([(0, 0x3), (1, 0x7), (2, 0xF)].to_vec())
            .unwrap();
        assert_eq!(g.output, 0x3);
    }

    #[test]
    fn try_from_new_gate() {
        let g = Gate::try_from((GateType::Xor, 4, 8, [0xFF, 0x03])).unwrap();
        assert_eq!(g.output, 0xFC);
    }
}
