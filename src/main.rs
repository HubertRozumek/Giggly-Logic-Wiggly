use eframe::{egui};
use logic::circuit::circuit::Circuit;
use logic::circuit::gate::*;
use std::rc::Rc;
use std::cell::RefCell;
use logic::circuit::wire::Wire;

type GateRef = Rc<RefCell<dyn Gate>>;

macro_rules! register_gate {
    ($( $txt:literal => $spawn:ident ),* $(,)?) => {
        pub fn palette(ui:&mut egui::Ui, app:&mut LogicApp) {
            ui.heading("Palette");
            $(
                if ui.button($txt).clicked() { app.$spawn(); }
            )*
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PortKind { In, Out }

struct Port { offset: egui::Vec2, kind: PortKind, gate_id: String }

struct Node {
    id:    String,
    gate:  GateRef,
    rect:  egui::Rect,
    ports: Vec<Port>,
    label: String,
}


struct WireVisual {
    from: (String, usize),     
    to:   (String, usize),
}

struct LogicApp {
    circuit: Circuit,
    nodes:   Vec<Node>,
    wires:   Vec<WireVisual>,
    pending_port: Option<(String, usize)>,

    drag_node:    Option<usize>, 
    drag_offset:  egui::Vec2,

    to_delete_node: Option<usize>,
    to_delete_wire: Option<usize>,
}

trait Snap                { fn snap_to_grid(self, step:f32) -> Self; }
trait ToPos2              { fn to_pos2(self) -> egui::Pos2; }

impl Snap   for egui::Vec2 { fn snap_to_grid(self, s:f32) -> Self {
    egui::vec2((self.x/s).round()*s, (self.y/s).round()*s)
}}
impl ToPos2 for egui::Vec2 { fn to_pos2(self) -> egui::Pos2 { egui::pos2(self.x, self.y) }}




impl Default for LogicApp {
    fn default() -> Self {
        Self {
            circuit: Circuit::new(),
            nodes:   Vec::new(),
            wires:   Vec::new(),
            pending_port: None,
            drag_node: None,
            drag_offset: egui::Vec2::ZERO,
            to_delete_node: None,
            to_delete_wire: None,
        }
    }
}

fn new_input_wire(app: &mut LogicApp, hint: &str) -> String {
    let wid   = format!("{hint}_w{}", app.nodes.len());
    let wgate = Rc::new(RefCell::new(Wire::new(&wid)));
    app.circuit.add_gate(&wid, wgate);
    wid
}

impl LogicApp {

    
    fn spawn_const(&mut self, level: Signal) {
        let id   = self.next_id();
        let gate = Rc::new(RefCell::new(ConstGate::new(level)));
        self.circuit.add_gate(&id, gate.clone());
        self.circuit.add_output(&id);
    
        self.nodes.push(Node {
            label: format!("CONST {}", if level == Signal::High { "1" } else { "0" }),
            id: id.clone(),
            gate,
            rect: egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(60.0, 30.0)),
            ports: vec![Port {
                offset: egui::vec2(60.0, 15.0),
                kind:   PortKind::Out,
                gate_id: id,
            }],
        });
    }

    fn spawn_switch(&mut self) {
        let id   = self.next_id();
        let gate = Rc::new(RefCell::new(InputGate::new(false)));
    
        self.circuit.add_gate(&id, gate.clone());
        self.nodes.push(Node {
            id: id.clone(), gate, label:"SW".into(),
            rect: egui::Rect::from_min_size(egui::pos2(120.0, 60.0), egui::vec2(50.0,30.0)),
            ports: vec![Port{offset:egui::vec2(50.0,15.0), kind:PortKind::Out, gate_id:id}],
        });
    }
    
    fn spawn_and(&mut self) {
        let base  = self.next_id();
        let a_id  = new_input_wire(self, &base);
        let b_id  = new_input_wire(self, &base);
        
        let a = self.circuit.gate(&a_id).unwrap();
        let b = self.circuit.gate(&b_id).unwrap();
        let gate = Rc::new(RefCell::new(AndGate::new(a, b)));
    
        self.circuit.add_gate(&base, gate.clone());
        self.circuit.add_output(&base);
    
        self.nodes.push(Node {
            label: "AND".into(),
            id: base.clone(),
            gate,
            rect: egui::Rect::from_min_size(egui::pos2(240.0, 100.0), egui::vec2(80.0, 40.0)),
            ports: vec![
                Port { offset: egui::vec2(0.0, 10.0), kind: PortKind::In , gate_id: a_id },
                Port { offset: egui::vec2(0.0, 30.0), kind: PortKind::In , gate_id: b_id },
                Port { offset: egui::vec2(80.0,20.0), kind: PortKind::Out, gate_id: base },
            ],
        });
    }

    

    fn spawn_not(&mut self) {
        let base = self.next_id();
        let in_id = new_input_wire(self, &base);
    
        let in_gate = self.circuit.gate(&in_id).unwrap();
        let gate    = Rc::new(RefCell::new(NotGate::new(in_gate)));
    
        self.circuit.add_gate(&base, gate.clone());
        self.circuit.add_output(&base);
    
        self.nodes.push(Node {
            label:"NOT".into(), id:base.clone(), gate,
            rect: egui::Rect::from_min_size(egui::pos2(340.0,100.0), egui::vec2(60.0,40.0)),
            ports: vec![
                Port{offset:egui::vec2(0.0,20.0), kind:PortKind::In , gate_id:in_id},
                Port{offset:egui::vec2(60.0,20.0), kind:PortKind::Out, gate_id:base},
            ],
        });
    }

    fn spawn_clock(&mut self) {
        let id   = self.next_id();
        let clk  = Rc::new(RefCell::new(ClockGate::new()));
        self.circuit.add_gate(&id, clk.clone());
        self.circuit.add_output(&id);
    
        self.nodes.push(Node {
            label:"CLK".into(), id:id.clone(), gate:clk,
            rect: egui::Rect::from_min_size(egui::pos2(420.0,100.0), egui::vec2(60.0,30.0)),
            ports: vec![Port{ offset:egui::vec2(60.0,15.0), kind:PortKind::Out, gate_id:id }],
        });
    }

    fn spawn_button(&mut self) {
        let id   = self.next_id();
        let gate = Rc::new(RefCell::new(InputGate::new(false)));
    
        self.circuit.add_gate(&id, gate.clone());
        self.nodes.push(Node {
            id: id.clone(), gate, label:"BTN".into(),
            rect: egui::Rect::from_min_size(egui::pos2(120.0,110.0), egui::vec2(50.0,30.0)),
            ports: vec![Port{offset:egui::vec2(50.0,15.0), kind:PortKind::Out, gate_id:id}],
        });
    }

    fn spawn_lamp(&mut self) {
        let wid = self.next_id();                  

        let wgate = Rc::new(RefCell::new(Wire::new(&wid)));
        self.circuit.add_gate(&wid, wgate.clone());

        self.nodes.push(Node {
            id:    wid.clone(),
            gate:  wgate,
            label: "LAMP".into(),
            rect:  egui::Rect::from_min_size(
                        egui::pos2(100.0, 220.0),    
                        egui::vec2(60.0, 30.0)),    
            ports: vec![
                Port {
                    offset: egui::vec2(0.0, 15.0),   
                    kind:   PortKind::In,
                    gate_id: wid,                   
                }
            ],
        });
    }

    fn spawn_buffer(&mut self){
        let base = self.next_id();
        let in_id = new_input_wire(self,&base);
        let in_g  = self.circuit.gate(&in_id).unwrap();;
    
        let gate = Rc::new(RefCell::new(BufferGate::new(in_g)));
        self.circuit.add_gate(&base, gate.clone());
        self.circuit.add_output(&base);
    
        self.nodes.push(Node{
            label:"BUF".into(), id:base.clone(), gate,
            rect: egui::Rect::from_min_size(egui::pos2(260.0,160.0), egui::vec2(60.0,30.0)),
            ports: vec![
                Port{offset:egui::vec2(0.0,15.0),kind:PortKind::In ,gate_id:in_id},
                Port{offset:egui::vec2(60.0,15.0),kind:PortKind::Out,gate_id:base},
            ],
        });
    }

    fn spawn_tri(&mut self){
        let base = self.next_id();
        let in_id = new_input_wire(self,&base);
        let en_id = new_input_wire(self,&base);
    
        let in_g = self.circuit.gate(&in_id).unwrap();
        let en_g = self.circuit.gate(&en_id).unwrap();
        let gate = Rc::new(RefCell::new(TriStateGate::new(in_g, en_g)));
    
        self.circuit.add_gate(&base, gate.clone());
        self.circuit.add_output(&base);
    
        self.nodes.push(Node{
            label:"TRI".into(), id:base.clone(), gate,
            rect: egui::Rect::from_min_size(egui::pos2(340.0,160.0), egui::vec2(90.0,40.0)),
            ports: vec![
                Port{offset:egui::vec2(0.0,12.0),kind:PortKind::In ,gate_id:in_id},
                Port{offset:egui::vec2(0.0,32.0),kind:PortKind::In ,gate_id:en_id},
                Port{offset:egui::vec2(90.0,22.0),kind:PortKind::Out,gate_id:base},
            ],
        });
    }

    fn spawn_binary<F>(&mut self, label:&str, ctor:F)
        where F: Fn(GateRef,GateRef)->GateRef
        {
            let base = self.next_id();
            let a_id = new_input_wire(self,&base);
            let b_id = new_input_wire(self,&base);

            let a = self.circuit.gate(&a_id).unwrap();
            let b = self.circuit.gate(&b_id).unwrap();
            let gate = ctor(a,b);

            self.circuit.add_gate(&base, gate.clone());
            self.circuit.add_output(&base);

            self.nodes.push(Node{
                label:label.into(), id:base.clone(), gate,
                rect: egui::Rect::from_min_size(egui::pos2(440.0,160.0), egui::vec2(90.0,40.0)),
                ports: vec![
                    Port{offset:egui::vec2(0.0,10.0),kind:PortKind::In ,gate_id:a_id},
                    Port{offset:egui::vec2(0.0,30.0),kind:PortKind::In ,gate_id:b_id},
                    Port{offset:egui::vec2(90.0,20.0),kind:PortKind::Out,gate_id:base},
                ],
            });
        }

fn spawn_nand(&mut self){ self.spawn_binary("NAND", |a,b| Rc::new(RefCell::new(NandGate::new(a,b)))); }
fn spawn_nor (&mut self){ self.spawn_binary("NOR" , |a,b| Rc::new(RefCell::new(NorGate ::new(a,b)))); }
fn spawn_or  (&mut self){ self.spawn_binary("OR"  , |a,b| Rc::new(RefCell::new(OrGate  ::new(a,b)))); }
fn spawn_xor (&mut self){ self.spawn_binary("XOR" , |a,b| Rc::new(RefCell::new(XorGate ::new(a,b)))); }
fn spawn_xnor(&mut self){ self.spawn_binary("XNOR", |a,b| Rc::new(RefCell::new(XnorGate::new(a,b)))); }

    fn next_id(&self) -> String { format!("g{}", self.nodes.len()) }
}


register_gate! {
    "Const 0" => spawn_const_low,
    "Const 1" => spawn_const_high,
    "Switch"  => spawn_switch,
    "Button"  => spawn_button,
    "Clock"   => spawn_clock,

    "Buffer"  => spawn_buffer,
    "NOT"     => spawn_not,
    "AND"     => spawn_and,
    "NAND"    => spawn_nand,
    "OR"      => spawn_or,
    "NOR"     => spawn_nor,
    "XOR"     => spawn_xor,
    "XNOR"    => spawn_xnor,
    "TRI-State"=> spawn_tri,

    "Lamp"    => spawn_lamp,
}


impl LogicApp {
    fn spawn_const_low (&mut self) { self.spawn_const(Signal::Low ); }
    fn spawn_const_high(&mut self) { self.spawn_const(Signal::High); }
}


impl eframe::App for LogicApp {
    fn update(&mut self, ctx:&egui::Context, _: &mut eframe::Frame) {


        egui::SidePanel::left("palette").show(ctx, |ui| {
            palette(ui, self);
            ui.separator();
        
            if ui.button("Tick clock").clicked() { self.circuit.step(); }
        });
        


        egui::CentralPanel::default().show(ctx, |ui| {
            let painter        = ui.painter();
            let canvas_offset  = ui.min_rect().min.to_vec2();

            let mut click: Option<(String,usize)> = None;

            for (idx, node) in self.nodes.iter_mut().enumerate() {

                if self.drag_node == Some(idx) {
                    if let Some(pointer) = ctx.input(|i| i.pointer.hover_pos()) {

                        let snapped = (pointer.to_vec2() - self.drag_offset - canvas_offset)
                                            .snap_to_grid(20.0)
                                            .to_pos2();

                        node.rect = egui::Rect::from_min_size(snapped, node.rect.size());
                    }
                }

                let rect_screen = node.rect.translate(canvas_offset);

                let base_color = if node.label == "LAMP" {
                    match node.gate.borrow().eval() {
                        Signal::High => egui::Color32::GREEN,
                        Signal::Low  => egui::Color32::RED,
                        Signal::HiZ  => egui::Color32::DARK_GRAY,
                    }
                } else { egui::Color32::DARK_GRAY };

                painter.rect_filled(rect_screen, 4.0, base_color);
                painter.text(rect_screen.center(), egui::Align2::CENTER_CENTER,
                             &node.label, egui::FontId::monospace(12.0), egui::Color32::WHITE);


                             let resp = ui.interact(
                                rect_screen,
                                ui.id().with(node.id.clone()),
                                egui::Sense::click_and_drag()
                                    .union(egui::Sense::click())            
                            );
                
                if resp.drag_started() {
                    self.drag_node   = Some(idx);
                    self.drag_offset = resp.interact_pointer_pos().unwrap()
                                      - node.rect.min - canvas_offset;
                }
                if resp.drag_stopped() {                   
                    self.drag_node = None;
                }

                if resp.secondary_clicked() {
                    self.to_delete_node = Some(idx); 
                }
                

                for (pidx, port) in node.ports.iter().enumerate() {
                    let pin_pos = rect_screen.min + port.offset;
                    painter.circle_filled(pin_pos, 4.0, match node.gate.borrow().eval() {
                        Signal::High => egui::Color32::GREEN,
                        Signal::Low  => egui::Color32::RED,
                        Signal::HiZ  => egui::Color32::GRAY,
                    });

                    let pin_resp = ui.interact(
                        egui::Rect::from_center_size(pin_pos, egui::vec2(8.0,8.0)),
                        ui.id().with((node.id.clone(), pidx)),
                        egui::Sense::click());

                    if pin_resp.clicked() {
                        click = Some((node.id.clone(), pidx));
                    }
                    if pin_resp.double_clicked() && port.kind == PortKind::In {
                        if let Some(inp) = node.gate
                            .borrow_mut()
                            .as_any()
                            .downcast_mut::<InputGate>() {
                            inp.set_signal(!inp.eval().is_high());
                        }
                    }
                }

                if node.label == "SW" {
                    let rect_screen = node.rect.translate(canvas_offset);
                    let resp = ui.interact(
                        rect_screen,
                        ui.id().with(node.id.clone()),
                        egui::Sense::click()
                    );
                    if resp.clicked() {
                        if let Some(inp) = node
                            .gate
                            .borrow_mut()
                            .as_any()
                            .downcast_mut::<InputGate>()
                        {
                            inp.set_signal(!inp.eval().is_high());
                        }
                    }
                }

                if node.label == "BTN" {
                    let rect_screen = node.rect.translate(canvas_offset);
                    let resp = ui.interact(
                        rect_screen,
                        ui.id().with(node.id.clone()),
                        egui::Sense::click_and_drag()   
                    );
                    if resp.is_pointer_button_down_on() {
                        if let Some(inp) = node
                            .gate
                            .borrow_mut()
                            .as_any()
                            .downcast_mut::<InputGate>()
                        {
                            inp.set_signal(true);
                        }
                    }
                    if resp.drag_stopped() {
                        if let Some(inp) = node
                            .gate
                            .borrow_mut()
                            .as_any()
                            .downcast_mut::<InputGate>()
                        {
                            inp.set_signal(false);
                        }
                    }
                }
            }

            if let Some(idx) = self.to_delete_node.take() {
                let id = self.nodes[idx].id.clone();
                self.wires.retain(|w| w.from.0 != id && w.to.0 != id);
                self.circuit.remove_gate(&id);  
                self.nodes.swap_remove(idx);
            }

            if let Some((nid, pidx)) = click {
                match self.pending_port.take() {
                    None => self.pending_port = Some((nid, pidx)),           
                    Some((aid, aidx)) => {                                   
                        self.wires.push(WireVisual { from:(aid.clone(),aidx), to:(nid.clone(),pidx) });

                        let from_gate = self.nodes.iter()
                                        .find(|n| n.id==aid).unwrap().ports[aidx].gate_id.clone();
                        let to_gate   = self.nodes.iter()
                                        .find(|n| n.id==nid).unwrap().ports[pidx].gate_id.clone();
                        let _ = self.circuit.connect(&from_gate,&to_gate);
                    }
                }
            }

            for (w_idx, w) in self.wires.iter().enumerate() {
                let (a_id, a_idx) = &w.from;
                let (b_id, b_idx) = &w.to;

                let na = self.nodes.iter().find(|n| &n.id == a_id).unwrap();
                let nb = self.nodes.iter().find(|n| &n.id == b_id).unwrap();

                let pa = na.rect.min + na.ports[*a_idx].offset + canvas_offset;
                let pb = nb.rect.min + nb.ports[*b_idx].offset + canvas_offset;

                let hit_box = egui::Rect::from_two_pos(pa, pb).expand(4.0);
                let resp = ui.interact(
                    hit_box,
                    ui.id().with(("wire", w_idx)),
                    egui::Sense::click()
                );
                if resp.secondary_clicked() {         
                    self.to_delete_wire = Some(w_idx);
                }

                painter.line_segment([pa, pb],
                    egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE));
            }


            if let Some(idx) = self.to_delete_wire.take() {
                self.wires.swap_remove(idx);
            }


        });

        ctx.request_repaint();  
    }
}





fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Logic",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(LogicApp::default())),
    )
}

