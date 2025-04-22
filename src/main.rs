use eframe::{egui};
use logic::circuit::circuit::Circuit;
use logic::circuit::gate::InputGate;
use std::rc::Rc;
use std::cell::RefCell;

struct LogicApp {
    circuit: Circuit,
    input_state: bool,
}

impl Default for LogicApp {

    fn default() -> Self {
        let mut circuit = Circuit::new();
        circuit.add_gate("i1", Rc::new(RefCell::new(InputGate::new(false))));
        circuit.add_output("i1");

        Self {
            circuit,
            input_state: false,
        }
    }
}

impl eframe::App for LogicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Logic Simulator GUI");
            
            if ui.button("Toggle Input Gate").clicked() {
                self.input_state = !self.input_state;
                self.circuit.set_input_signal("i1", self.input_state).unwrap();
            }
            let binding = self.circuit.eval();
            let output = binding.get("i1").unwrap();
            ui.label(format!("Current Input State: {}", output));
        });
    }
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Logic",
        options,
        Box::new(|_cc| Box::new(LogicApp::default())),
    )
}