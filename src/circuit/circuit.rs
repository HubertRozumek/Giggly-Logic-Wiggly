use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

use crate::circuit::gate::Gate;

use super::gate::ConstGate;



#[derive(Debug, Serialize, Deserialize)]
pub struct Circuit {
    #[serde(skip)]
    gates: HashMap<String, Rc<RefCell<dyn Gate>>>,
    outputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]

struct SerializableGate{
    gate_type: String,
    id: String,
    value: Option<bool>,
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

    pub fn save(&self,path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let gates: Vec<SerializableGate> = self.gates.iter().map(|(id,gate)|{
            let gate_ref = gate.borrow();
            SerializableGate {
                gate_type: gate_ref.description(),
                id: id.clone(),
                value: Some(gate_ref.eval()),
            }
        }).collect();

        let circuit_state = (gates, &self.outputs);
        let json = serde_json::to_string_pretty(&circuit_state)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let (gates_data, outputs): (Vec<SerializableGate>, Vec<String>) = serde_json::from_str(&content)?;

         let mut circuit = Circuit::new();
         for sg in gates_data {
            match sg.gate_type.as_str() {
                typ if typ.starts_with("Input") => {
                    circuit.add_gate(sg.id, Rc::new(RefCell::new(InputGate::new(sg.value.unwrap()))));  
                },
                typ if typ.starts_with("Const") => {
                    circuit.add_gate(sg.id, Rc::new(RefCell::new(ConstGate::new(sg.value.unwrap()))));
                },
                _ => return Err(format!("Unsuppoorted gate type: {}", sg.gate_type).into()),
         }
    }
    circuit.outputs = outputs;

    Ok(circuit)
    }
}

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

    #[test]
    fn seria_test(){
        let mut circuit = Circuit::new();

        circuit.add_gate("i1", Rc::new(RefCell::new(InputGate::new(true))));
        circuit.add_output("i1");

        let path = "test_circuit.json";
        circuit.save(path).unwrap();

        let load_circuit = Circuit::load(path).unwrap();
        assert_eq!(load_circuit.eval().get("i1"), Some(&true));

        std::fs::remove_file(path).unwrap();
    }
}