use crate::circuit::gate::Gate;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use std::any::Any;

#[derive(Debug)]
pub struct Wire {
    source: Option<Rc<RefCell<dyn Gate>>>,
    label: String,
}


impl Wire {
    pub fn new(label:impl Into<String>) -> Self {
        Self {
            source: None,
            label: label.into(),
        }
    }

    pub fn connect(&mut self, gate: Rc<RefCell<dyn Gate>>) {
        self.source = Some(gate);
    }

}

impl Gate for Wire {
    fn eval(&self) -> bool {
        self.source
            .as_ref()
            .map(|gate| gate.borrow().eval())
            .unwrap_or(false)
    }

    fn description(&self) -> String {
        format!(
            "Wire({}, connected: {})",
            self.label,
            self.source.as_ref()
                .map(|gate| gate.borrow().description())
                .unwrap_or("None".to_string())
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::circuit::gate::ConstGate;

    #[test]
    fn test_wire(){
        let const_gate = Rc::new(RefCell::new(ConstGate::new(true)));
        let mut wire = Wire::new("w1");

        assert_eq!(wire.eval(),false);
        assert_eq!(wire.description(),"Wire(w1, connected: None)");

        wire.connect(const_gate.clone());
        assert_eq!(wire.eval(), true);
        assert_eq!(wire.description(), "Wire(w1, connected: Const true)")
    }
}