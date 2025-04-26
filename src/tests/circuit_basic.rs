use crate::circuit::circuit::Circuit;
use crate::circuit::gate::*;
use crate::circuit::wire::Wire;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_circuit() {
    let mut circuit = Circuit::new();

    let const_true = Rc::new(RefCell::new(ConstGate::new(Signal::High)));

    circuit.add_gate("A", const_true.clone());
    circuit.add_output("A");

    let result = circuit.eval();
    assert_eq!(result.get("A"), Some(&true));

    let desc = circuit.description();
    assert_eq!(desc, "A => Const 1")
}

#[test]

fn test_set_input_signal() {
    let mut circuit = Circuit::new();

    let input_gate = Rc::new(RefCell::new(InputGate::new(false)));
    circuit.add_gate("i1", input_gate.clone());
    circuit.add_output("i1");

    assert_eq!(circuit.eval().get("i1"), Some(&false));

    circuit.set_input_bool("i1", true).unwrap();
    assert_eq!(circuit.eval().get("i1"), Some(&true));

    let err = circuit.set_input_bool("i2", true);
    assert!(err.is_err());
}

#[test]
fn test_circuit_with_wire() {
    let mut circuit = Circuit::new();

    let const_gate = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    circuit.add_gate("c1", const_gate);

    let wire = Wire::new("w1");
    circuit.add_wire("w1", wire);

    circuit.connect("c1", "w1").unwrap();

    circuit.add_output("w1");

    let result = circuit.eval();
    assert_eq!(result.get("w1"), Some(&true));

    let desc = circuit.description();
    assert_eq!(desc, "w1 => Wire(w1, connected: Const 1)")
}

#[test]
fn test_fa() {
    let mut circuit = Circuit::new();

    circuit.add_gate("a", Rc::new(RefCell::new(ConstGate::new(Signal::High))));
    circuit.add_gate("b", Rc::new(RefCell::new(ConstGate::new(Signal::High))));
    circuit.add_gate("cin", Rc::new(RefCell::new(ConstGate::new(Signal::High))));

    circuit
        .add_full_adder("a", "b", "cin", "sum", "carry")
        .unwrap();
    circuit.add_output("sum");
    circuit.add_output("carry");

    let result = circuit.eval();
    assert_eq!(result.get("sum"), Some(&true));
    assert_eq!(result.get("carry"), Some(&true));
}

#[test]
fn test_4bit_adder() {
    let mut circuit = Circuit::new();

    circuit.add_gate("a0", Rc::new(RefCell::new(ConstGate::new(Signal::High))));
    circuit.add_gate("a1", Rc::new(RefCell::new(ConstGate::new(Signal::High))));
    circuit.add_gate("a2", Rc::new(RefCell::new(ConstGate::new(Signal::Low))));
    circuit.add_gate("a3", Rc::new(RefCell::new(ConstGate::new(Signal::Low))));

    circuit.add_gate("b0", Rc::new(RefCell::new(ConstGate::new(Signal::High))));
    circuit.add_gate("b1", Rc::new(RefCell::new(ConstGate::new(Signal::Low))));
    circuit.add_gate("b2", Rc::new(RefCell::new(ConstGate::new(Signal::High))));
    circuit.add_gate("b3", Rc::new(RefCell::new(ConstGate::new(Signal::Low))));

    circuit.add_gate("cin", Rc::new(RefCell::new(ConstGate::new(Signal::Low))));

    circuit
        .add_4bit_adder(
            "a0",
            "a1",
            "a2",
            "a3",
            "b0",
            "b1",
            "b2",
            "b3",
            "cin",
            ["sum0", "sum1", "sum2", "sum3"],
            "cout",
        )
        .unwrap();

    circuit.add_output("sum0");
    circuit.add_output("sum1");
    circuit.add_output("sum2");
    circuit.add_output("sum3");
    circuit.add_output("cout");

    let out = circuit.eval();

    assert_eq!(out.get("sum0"), Some(&false));
    assert_eq!(out.get("sum1"), Some(&false));
    assert_eq!(out.get("sum2"), Some(&false));
    assert_eq!(out.get("sum3"), Some(&true));
    assert_eq!(out.get("cout"), Some(&false));
}
