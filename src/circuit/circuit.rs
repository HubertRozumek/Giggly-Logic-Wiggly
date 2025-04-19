use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::circuit::gate::Gate;



#[derive(Debug)]
pub struct Circuit {
    gates: HashMap<String, Rc<RefCell<dyn Gate>>>,
    outputs: Vec<String>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            gates: HashMap::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_gate(&mut self, id: impl Into<String>, gate: Rc<RefCell<dyn Gate>>) {
        self.gates.insert(id.into(), gate);
    }

    pub fn add_output(&mut self, id: impl Into<String>) {
        self.outputs.push(id.into());
    }

    pub fn set_input_signal(&mut self, gate_id: &str, signal: bool) -> Result<(), String>{
            if let Some(gate) = self.gates.get(gate_id) {
                let mut gate_ref = gate.borrow_mut();
                let any_gate = gate_ref.as_any();

                if let Some(input_gate) = any_gate.downcast_mut::<InputGate>() {
                    input_gate.set_signal(signal);
                    Ok(())
                } else {
                    Err(format!("Gate '{}' is not an InputGate", gate_id))
                }

            } else {
                Err(format!("Gate '{}' not found", gate_id))
            }
        }
    

    pub fn eval(&self) -> HashMap<String, bool> {
        let mut result = HashMap::new();

        for id in &self.outputs {
            if let Some(gate) = self.gates.get(id) {
                let val = gate.borrow().eval();
                result.insert(id.clone(),val);
            }
        }
        result
    }

    pub fn description(&self) -> String {
        self.outputs
            .iter()
            .filter_map(|id|{
                self.gates.get(id).map(|gate|{
                    format!("{} => {}", id, gate.borrow().description())
                })
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

}

use crate::circuit::gate::{ConstGate};

use super::gate::InputGate; 

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_circuit(){

        let mut circuit = Circuit::new();

        let const_true = Rc::new(RefCell::new(ConstGate::new(true)));

        circuit.add_gate("A", const_true.clone());
        circuit.add_output("A");

        let result = circuit.eval();
        assert_eq!(result.get("A"), Some(&true));

        let desc = circuit.description();
        assert_eq!(desc, "A => Const true")

    }

    #[test]

    fn test_set_input_signal(){
        let mut circuit = Circuit::new();

        let input_gate = Rc::new(RefCell::new(InputGate::new(false)));
        circuit.add_gate("i1", input_gate.clone());
        circuit.add_output("i1");

        assert_eq!(circuit.eval().get("i1"),Some(&false));

        circuit.set_input_signal("i1", true).unwrap();
        assert_eq!(circuit.eval().get("i1"), Some(&true));

        let err = circuit.set_input_signal("i2", true);
        assert!(err.is_err());

    }
}