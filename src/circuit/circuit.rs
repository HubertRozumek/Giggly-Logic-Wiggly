use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

use crate::circuit::gate::Gate;
use crate::circuit::wire::Wire;
use super::gate::*;
use crate::circuit::gate::{FullAdder, HalfAdder, ClockGate};


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

    pub fn add_wire(&mut self, id: impl Into<String>, wire: Wire) {
        self.gates.insert(id.into(), Rc::new(RefCell::new(wire)));
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
    
    pub fn connect(&mut self, from_gate_id: &str, wire_id: &str) -> Result<(), String> {
        let gate = self.gates.get(from_gate_id)
            .ok_or_else(|| format!("Gate '{}' not found", from_gate_id))?
            .clone();

        let wire_ref = self.gates.get(wire_id)
            .ok_or_else(|| format!("Wire '{}' not found", wire_id))?
            .clone();

        let mut wire = wire_ref.borrow_mut();
        let any_wire = wire.as_any();

        if let Some(wire) = any_wire.downcast_mut::<Wire>() {
            wire.connect(gate);
            Ok(())
        } else {
            Err(format!("Gate '{}' is not a Wire", wire_id))
        }
    }

    pub fn add_halfadder(&mut self, a_id: &str, b_id: &str, sum_id: &str, carry_id: &str) -> Result<(), String> {
        let a = self.gates.get(a_id)
            .ok_or_else(|| format!("Gate '{}' not found", a_id))?
            .clone();

        let b = self.gates.get(b_id)
            .ok_or_else(|| format!("Gate '{}' not found", b_id))?
            .clone();

        let ha = HalfAdder::new(a,b);

        self.add_gate(sum_id, ha.sum.clone());
        self.add_gate(carry_id, ha.carry.clone());

        Ok(())
    }

    pub fn add_full_adder(&mut self, a_id: &str, b_id: &str, cin_id: &str, sum_id: &str, carry_id: &str) -> Result<(), String> {
        let a = self.gates.get(a_id)
            .ok_or_else(|| format!("Gate '{}' not found", a_id))?
            .clone();

        let b = self.gates.get(b_id)
            .ok_or_else(|| format!("Gate '{}' not found",b_id))?
            .clone();

        let cin = self.gates.get(cin_id)
            .ok_or_else(|| format!("Gate '{}' not found", cin_id))?
            .clone();

        let fa = FullAdder::new(a,b,cin);

        self.add_gate(sum_id, fa.sum.clone());
        self.add_gate(carry_id, fa.carry.clone());

        Ok(())
    }

    pub fn add_4bit_adder(
        &mut self, 
        a0_id: &str, a1_id: &str, a2_id: &str, a3_id: &str,
        b0_id: &str, b1_id: &str, b2_id: &str, b3_id: &str,
        cin_id: &str,
        sum_ids: [&str; 4],
        cout_id: &str,
        ) -> Result<(), String> {
        
        let a = [
            self.gates.get(a0_id).ok_or_else(|| format!("Gate '{}' not found", a0_id))?.clone(),
            self.gates.get(a1_id).ok_or_else(|| format!("Gate '{}' not found", a1_id))?.clone(),
            self.gates.get(a2_id).ok_or_else(|| format!("Gate '{}' not found", a2_id))?.clone(),
            self.gates.get(a3_id).ok_or_else(|| format!("Gate '{}' not found", a3_id))?.clone(),
        ];
    
        let b = [
            self.gates.get(b0_id).ok_or_else(|| format!("Gate '{}' not found", b0_id))?.clone(),
            self.gates.get(b1_id).ok_or_else(|| format!("Gate '{}' not found", b1_id))?.clone(),
            self.gates.get(b2_id).ok_or_else(|| format!("Gate '{}' not found", b2_id))?.clone(),
            self.gates.get(b3_id).ok_or_else(|| format!("Gate '{}' not found", b3_id))?.clone(),
        ];

        let mut carry = self.gates.get(cin_id).ok_or_else(|| format!("Gate '{}' not found",cin_id))?.clone();

        for i in 0..4 {
            let fa = FullAdder::new(a[i].clone(), b[i].clone(),carry.clone());
            self.add_gate(sum_ids[i], fa.sum.clone());
            carry = fa.carry.clone();
        }

        self.add_gate(cout_id, carry);
        
        Ok(())
    }    

    pub fn step(&mut self) {
        for gate in self.gates.values() {
            if let Some(clock) = gate.borrow_mut().as_any().downcast_mut::<ClockGate>() {
                clock.tick()
            }
        }

    }

    pub fn eval(&self) -> HashMap<String, bool> {
        let mut result = HashMap::new();

        for id in &self.outputs {
            if let Some(gate) = self.gates.get(id) {
                let val = gate.borrow().eval();
                result.insert(id.clone(),val.into());
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

    // pub fn save(&self,path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let gates: Vec<SerializableGate> = self.gates.iter().map(|(id,gate)|{
    //         let gate_ref = gate.borrow();
    //         SerializableGate {
    //             gate_type: gate_ref.description(),
    //             id: id.clone(),
    //             value: Some(gate_ref.eval()),
    //         }
    //     }).collect();

    //     let circuit_state = (gates, &self.outputs);
    //     let json = serde_json::to_string_pretty(&circuit_state)?;
    //     std::fs::write(path, json)?;
    //     Ok(())
    // }

    // pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    //     let content = std::fs::read_to_string(path)?;
    //     let (gates_data, outputs): (Vec<SerializableGate>, Vec<String>) = serde_json::from_str(&content)?;

    //      let mut circuit = Circuit::new();
    //      for sg in gates_data {
    //         match sg.gate_type.as_str() {
    //             typ if typ.starts_with("Input") => {
    //                 circuit.add_gate(sg.id, Rc::new(RefCell::new(InputGate::new(sg.value.unwrap()))));  
    //             },
    //             typ if typ.starts_with("Const") => {
    //                 circuit.add_gate(sg.id, Rc::new(RefCell::new(ConstGate::new(sg.value.unwrap()))));
    //             },
    //             _ => return Err(format!("Unsuppoorted gate type: {}", sg.gate_type).into()),
    //      }
    // }
    // circuit.outputs = outputs;

    // Ok(circuit)
    // }
}