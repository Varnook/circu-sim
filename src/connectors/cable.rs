use crate::base_gates::gate::Gate;
use std::collections::HashMap;

#[derive(PartialEq)]
struct Cable<'a> {
    value: u64,
    size: usize,
    connected_inputs: HashMap<&'a mut Gate, Vec<usize>>,
    connected_output: &'a Gate,
}

impl<'a> Cable<'a> {
    // TODO Error handling when gate bus size and cable size differ.
    /// Connects a cable to a single gate's input.
    pub fn connect_input(mut self, gate_to_connect: &'a mut Gate, input_to_connect: usize) {
        match self.connected_inputs.get_mut(gate_to_connect) {
            Some(gate_inputs) => if !gate_inputs.contains(&input_to_connect) { gate_inputs.push(input_to_connect) },
            None => {self.connected_inputs.insert(gate_to_connect, vec![input_to_connect]); ()}
        }
    }
}
