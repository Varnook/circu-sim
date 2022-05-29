use crate::base_gates::gate::Gate;
use crate::base_gates::gate_error::GateError;
use std::collections::HashMap;

#[derive(PartialEq)]
struct Cable<'a> {
    value: u64,
    size: usize,
    connected_inputs: HashMap<&'a Gate, Vec<usize>>,
    connected_output: Option<&'a Gate>,
}

impl<'a> Cable<'a> {
    pub fn new(size: usize) -> Cable<'a> {
        Cable {
            value: 0,
            size,
            connected_inputs: HashMap::new(),
            connected_output: None,
        }
    }

    /// Connects a cable to a single gate's input.
    pub fn connect_input(
        &mut self,
        gate_to_connect: &'a mut Gate,
        input_to_connect: usize,
    ) -> Result<(), GateError> {
        // Make sure the output is updated
        self.value = match self.connected_output {
            Some(output) => output.get_output(),
            None => 0
        };

        gate_to_connect.set_inputs(vec![(input_to_connect, self.value)])?;

        match self.connected_inputs.get_mut(gate_to_connect) {
            Some(gate_inputs) => {
                if !gate_inputs.contains(&input_to_connect) {
                    gate_inputs.push(input_to_connect)
                }
            }
            None => {
                self.connected_inputs
                    .insert(gate_to_connect, vec![input_to_connect]);
            }
        }
        Ok(())
    }

    pub fn connect_output(&mut self, gate_to_connect: &'a Gate) {
        self.connected_output = Some(gate_to_connect);
        self.value = gate_to_connect.get_output();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_gates::gate_type::GateType;

    #[test]
    fn connect_gate() {
        let mut g_in = Gate::new(GateType::Or, 2, 1).unwrap();
        let g_out = Gate::try_from((GateType::Xor, 3, 1, [1])).unwrap();
        let mut ca = Cable::new(1); 
        ca.connect_output(&g_out);
        assert_eq!(g_in.get_output(), 0);
        ca.connect_input(&mut g_in, 0);
        assert_eq!(g_in.get_output(), 1);
    }

    #[test]
    fn connect_not_gate() {
        let g_out = Gate::new(GateType::Not, 1, 4).unwrap();
        let mut g_in = Gate::new(GateType::Or, 2, 4).unwrap();
        let mut cable = Cable::new(4);
        cable.connect_output(&g_out);
        cable.connect_input(&mut g_in, 0);
        assert_eq!(g_in.get_output(), 0xF);
    }
}
