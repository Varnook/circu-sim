use crate::base_gates::gate::Gate;
use std::collections::HashMap;

#[derive(PartialEq)]
struct Cable<'a> {
    value: u64,
    size: usize,
    connected_inputs: HashMap<&'a mut Gate, Vec<usize>>,
    connected_output: &'a Gate,
}

impl<'a> Cable<'a> {}
