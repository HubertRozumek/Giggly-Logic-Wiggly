use crate::circuit::gate::Gate;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use std::any::Any;
use crate::circuit::gate::Signal;

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
    fn eval(&self) -> Signal {
        self.source
            .as_ref()
            .map(|gate| gate.borrow().eval())
            .unwrap_or(Signal::Low)
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
