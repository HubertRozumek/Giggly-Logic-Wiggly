use crate::circuit::wire::Wire;
use std::rc::Rc;
use std::cell::RefCell;
use crate::circuit::gate::{Signal, ConstGate};


// #[test]
// fn test_wire(){
//     let const_gate = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
//     let mut wire = Wire::new("w1");

//     assert_eq!(wire., Signal::Low);
//     assert_eq!(wire.description(),"Wire(w1, connected: None)");

//     wire.connect(const_gate.clone());
//     assert_eq!(wire.eval(), Signal::High);
//     assert_eq!(wire.description(), "Wire(w1, connected: Const 1)")
// }
