use std::rc::Rc;
use std::{cell::RefCell, fmt::Debug};
use std::any::Any;


pub trait Gate: Debug{
    fn eval(&self) -> Signal;
    fn description(&self) -> String;

    fn as_any(&mut self) -> &mut dyn Any;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signal {
    Low,
    High,
    HiZ,
}


#[derive(Debug)]
pub struct ConstGate { level: Signal }

#[derive(Debug)]
pub struct SwitchGate {
    level: bool,
}

#[derive(Debug)]
pub struct ButtonGate {
    input: bool,
}

// #[derive(Debug)]
// pub struct LowConstGate {
//     input: Signal,
// }

// #[derive(Debug)]
// pub struct HighConstGate {
//     input: Signal,
// }

#[derive(Debug)]
pub struct BufferGate {
    input: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct XnorGate {
    signal_one: Rc<RefCell<dyn Gate>>,
    signal_two: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct TriStateGate{
    input: Rc<RefCell<dyn Gate>>,
    enable: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct InputGate {
    signal: bool,
}

#[derive(Debug)]
pub struct OutputGate {
    input: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct AndGate{
    signal_one: Rc<RefCell<dyn Gate>>,
    signal_two: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct OrGate{
    signal_one: Rc<RefCell<dyn Gate>>,
    signal_two: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct XorGate{
    signal_one: Rc<RefCell<dyn Gate>>,
    signal_two: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct NorGate{
    signal_one: Rc<RefCell<dyn Gate>>,
    signal_two: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct NandGate{
    signal_one: Rc<RefCell<dyn Gate>>,
    signal_two: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct NotGate{
    signal: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct HalfAdder {
    pub sum: Rc<RefCell<dyn Gate>>,
    pub carry: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct FullAdder {
    pub sum: Rc<RefCell<dyn Gate>>,
    pub carry: Rc<RefCell<dyn Gate>>,
}

#[derive(Debug)]
pub struct SRLatch {
    set: Rc<RefCell<dyn Gate>>,
    reset: Rc<RefCell<dyn Gate>>,
    last_q: RefCell<Signal>,
}

#[derive(Debug)]
pub struct Dlatch {
    d: Rc<RefCell<dyn Gate>>,
    enable: Rc<RefCell<dyn Gate>>,
    state: RefCell<Signal>,
}

#[derive(Debug)]
pub struct Dflipflop {
    d: Rc<RefCell<dyn Gate>>,
    clk: Rc<RefCell<dyn Gate>>,
    state: RefCell<Signal>,
    last_clk: RefCell<Signal>,
}

#[derive(Debug)]
pub struct ClockGate{
    state: RefCell<Signal>,
}



impl Signal {
    #[inline] pub fn is_low(self) -> bool { self == Signal::Low }
    #[inline] pub fn is_high(self) -> bool { self == Signal::High }

    pub fn invert(self) -> Self {
        match self {
            Signal::High => Signal::Low,
            Signal::Low => Signal::High,
            Signal::HiZ => Signal::HiZ
            
        }
    }
}

impl From<Signal> for bool {
    fn from(s: Signal) -> Self {s == Signal::High}
}

impl ConstGate {
    pub fn new(level: Signal) -> Self { Self { level } }

pub fn set_level(&mut self, s: Signal) { self.level = s; }
}

impl SwitchGate {
    pub fn new(init: bool) -> Self {Self { level: init } }
    pub fn toggle(&mut self) { self.level = !self.level; }
}

impl ButtonGate {
    pub fn press(&mut self)  { self.input = true; }
    pub fn release(&mut self){ self.input = false; }
}

impl BufferGate {
    pub fn new(input: Rc<RefCell<dyn Gate>>) -> Self { Self {input} }
}

impl XnorGate {
    pub fn new(signal_one: Rc<RefCell<dyn Gate>>, signal_two: Rc<RefCell<dyn Gate>>) -> Self { Self { signal_one, signal_two} }
}

impl TriStateGate {
    pub fn new(input: Rc<RefCell<dyn Gate>>, enable: Rc<RefCell<dyn Gate>>) -> Self {
        Self { input, enable }        
    }
}

impl InputGate {
    pub fn new(s: bool) -> Self{
        Self {signal: s}
    }

    pub fn set_signal(&mut self, new_signal:bool) {
        self.signal = new_signal;
    }
}

impl OutputGate {
    pub fn new(input_gate: Rc<RefCell<dyn Gate>>) -> Self {
        Self {input: input_gate}
    }
}

impl AndGate{
    pub fn new(a:Rc<RefCell<dyn Gate>>, b: Rc<RefCell<dyn Gate>>) -> Self {
        Self { signal_one: a, signal_two: b}
    }
}

impl OrGate{
    pub fn new(a: Rc<RefCell<dyn Gate>>, b: Rc<RefCell<dyn Gate>>) -> Self {
        Self { signal_one: a, signal_two: b}
    }
}

impl NotGate{
    pub fn new(a: Rc<RefCell<dyn Gate>>) -> Self {
        Self { signal: a}
    }
}

impl XorGate{
    pub fn new(a:Rc<RefCell<dyn Gate>>, b: Rc<RefCell<dyn Gate>>  ) -> Self{
        Self {signal_one:a, signal_two:b}
    }
}

impl NorGate{
    pub fn new(a: Rc<RefCell<dyn Gate>>, b: Rc<RefCell<dyn Gate>>) -> Self{
        Self {signal_one:a,signal_two:b}
    }
}

impl NandGate{
    pub fn new(a: Rc<RefCell<dyn Gate>>, b: Rc<RefCell<dyn Gate>>) -> Self{
        Self {signal_one:a,signal_two:b}
    }
}

impl HalfAdder {
    pub fn new(a: Rc<RefCell<dyn Gate>>, b: Rc<RefCell<dyn Gate>>) -> Self {
        let sum = Rc::new(RefCell::new(XorGate::new(a.clone(), b.clone())));
        let carry = Rc::new(RefCell::new(AndGate::new(a,b)));

        Self {sum, carry}
    }
}

impl FullAdder {
    pub fn new(a: Rc<RefCell<dyn Gate>>,b: Rc<RefCell<dyn Gate>>, cin: Rc<RefCell<dyn Gate>>) -> Self {
        
        let ha_one_sum = Rc::new(RefCell::new(XorGate::new(a.clone(),b.clone())));
        let ha_one_carry = Rc::new(RefCell::new(AndGate::new(a.clone(),b.clone())));

        let ha_two_sum = Rc::new(RefCell::new(XorGate::new(ha_one_sum.clone(),cin.clone())));
        let ha_two_carry = Rc::new(RefCell::new(AndGate::new(ha_one_sum.clone(), cin.clone())));
        
        let f_carry = Rc::new(RefCell::new(OrGate::new(ha_one_carry,ha_two_carry)));

        Self {
            sum: ha_two_sum,
            carry: f_carry,
        }
    }
}

impl SRLatch{
    pub fn new(set: Rc<RefCell<dyn Gate>>, reset: Rc<RefCell<dyn Gate>>) -> Self {
        Self { set: (set), reset: (reset), last_q: (RefCell::new(Signal::Low)) }
    }
}

impl Dlatch {
    pub fn new(d: Rc<RefCell<dyn Gate>>, enable: Rc<RefCell<dyn Gate>>) -> Self {
        Self { d: (d), enable: (enable), state: (RefCell::new(Signal::Low)) }
    }
}

impl Dflipflop {
    pub fn new(d: Rc<RefCell<dyn Gate>>, clk: Rc<RefCell<dyn Gate>>) -> Self {
        Self { d: (d), clk: (clk), state: (RefCell::new(Signal::Low)), last_clk: (RefCell::new(Signal::Low)) }
    }
}

impl ClockGate {
    pub fn new() -> Self {
        Self { state: RefCell::new(Signal::Low) }
    }

    pub fn tick(&self) {
        let mut state = self.state.borrow_mut();
        *state = if *state == Signal::Low { Signal::High } else { Signal::Low };
    }
}

#[inline]
pub fn and(a: Signal, b: Signal) -> Signal {
    match (a,b) {
        (Signal::High, Signal::High) => Signal::High,
        (Signal::HiZ, _) | (_, Signal::HiZ) => Signal::HiZ,
        _ => Signal::Low,
    }
}

#[inline]
pub fn or(a: Signal, b: Signal) -> Signal {
    match (a,b) {
        (Signal::Low, Signal::Low) => Signal::Low,
        (Signal::HiZ, _) | (_, Signal::HiZ) => Signal::HiZ,
        _ => Signal::High,
    }
}

#[inline]
pub fn xor(a: Signal, b: Signal) -> Signal {
    use Signal::*;
    match (a,b) {
        (HiZ, _) | (_, HiZ) => HiZ,
        (High, High) | (Low, Low) => Low,
        _ => High,
    }
}



// gate implementation 
impl Gate for SwitchGate {
    fn eval(&self) -> Signal {
        if self.level { Signal::High} else { Signal::Low}
    }
    fn description(&self) -> String {
        format!("Switch {}", self.level)
    }
    fn as_any(&mut self) -> &mut dyn Any { self }
}

impl Gate for ButtonGate {
    fn eval(&self) -> Signal {
        if self.input { Signal::High } else { Signal::Low }
    }

    fn description(&self) -> String { "Button".into()}
    fn as_any(&mut self) -> &mut dyn Any { self }
}

// impl Gate for LowConstGate {
//     fn eval(&self) -> Signal { Signal::Low }
//     fn description(&self) -> String { "Const 0".into() }
//     fn as_any(&mut self) -> &mut dyn Any { self }
// }

// impl Gate for HighConstGate {
//     fn eval(&self) -> Signal { Signal::High }
//     fn description(&self) -> String { "Const 1".into() }
//     fn as_any(&mut self) -> &mut dyn Any { self }
// }
impl Gate for BufferGate {
    fn eval(&self) -> Signal { self.input.borrow().eval() }

    fn description(&self) -> String {
        format!("Buffer({})", self.input.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any { self }
}

impl Gate for XnorGate {
    fn eval(&self) -> Signal {
        let s = xor(self.signal_one.borrow().eval(), self.signal_two.borrow().eval());
        match s {
            Signal::High => Signal::Low,
            Signal::Low => Signal::High,
            Signal::HiZ => Signal::HiZ,
        }
    }

    fn description(&self) -> String {
        format!("Xnor({},{})", self.signal_one.borrow().description(), self.signal_two.borrow().description())
    }
    fn as_any(&mut self) -> &mut dyn Any { self }
}

impl Gate for TriStateGate {
    fn eval(&self) -> Signal {
        if self.enable.borrow().eval().is_high(){
            self.input.borrow().eval()
        } else {
        Signal::HiZ
        }
    }
    fn description(&self) -> String {
        format!("TriStateGate({},{})", self.input.borrow().description(), self.enable.borrow().description())
    }
    fn as_any(&mut self) -> &mut dyn Any { self }
}

impl Gate for ConstGate {
    fn eval(&self) -> Signal { self.level }
    fn description(&self) -> String {
        match self.level {
            Signal::High => "Const 1".into(),
            Signal::Low  => "Const 0".into(),
            Signal::HiZ  => "Const Z".into(),
        }
    }
    fn as_any(&mut self) -> &mut dyn Any { self }
}

impl Gate for InputGate{
    fn eval(&self) -> Signal {
        if self.signal { Signal::High } else { Signal::Low }
    }

    fn description(&self) -> String {
        format!("Input {}",self.signal)
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for OutputGate {
    fn eval(&self) -> Signal {
        self.input.borrow().eval()
    }

    fn description(&self) -> String {
        format!("Output ({})",self.input.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for AndGate {
    fn eval(&self) -> Signal {
        and(self.signal_one.borrow().eval(),self.signal_two.borrow().eval())
    }

    fn description(&self) -> String{
        format!("And({}, {})",
         self.signal_one.borrow().description(), 
         self.signal_two.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for OrGate {
    fn eval(&self) -> Signal {
        or(self.signal_one.borrow().eval(), self.signal_two.borrow().eval())
    }

    fn description(&self) -> String{
        format!("Or({}, {})",
         self.signal_one.borrow().description(), 
         self.signal_two.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for NotGate {
    fn eval(&self) -> Signal {
        self.signal.borrow().eval().invert()
    }

    fn description(&self) -> String{
        format!("Not({})",
        self.signal.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for XorGate {
    fn eval(&self) -> Signal {
        xor(self.signal_one.borrow().eval(), self.signal_two.borrow().eval())
    }

    fn description(&self) -> String {
        format!("Xor({},{})",
        self.signal_one.borrow().description(),
        self.signal_two.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for NorGate {
    fn eval(&self) -> Signal {
        or(self.signal_one.borrow().eval(), self.signal_two.borrow().eval()).invert()
    }

    fn description(&self) -> String {
        format!("Nor({},{})",
        self.signal_one.borrow().description(),
        self.signal_two.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for NandGate {
    fn eval(&self) -> Signal {
        and(self.signal_one.borrow().eval(), self.signal_two.borrow().eval()).invert()
    }

    fn description(&self) -> String {
        format!("Nand({},{})",
        self.signal_one.borrow().description(),
        self.signal_two.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for SRLatch {
    fn eval(&self) -> Signal {
        let s = self.set.borrow().eval();
        let r = self.reset.borrow().eval();

        let next = match (s, r) {
            (Signal::High, Signal::Low) => Signal::High,
            (Signal::Low, Signal::High) => Signal::Low,
            (Signal::Low, Signal::Low) => *self.last_q.borrow(),
            _ =>  *self.last_q.borrow()
            
        };

        *self.last_q.borrow_mut() = next;
        next
    }

    fn description(&self) -> String {
        format!(
            "SRLatch({:?})", *self.last_q.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for Dlatch {
    fn eval(&self) -> Signal {
        if self.enable.borrow().eval().is_high() {
            *self.state.borrow_mut() = self.d.borrow().eval();
        }

        *self.state.borrow()
    }

    fn description(&self) -> String {
        format!(
            "DLatch {:?}", *self.state.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for Dflipflop {
    fn eval(&self) -> Signal {
        let clk_val = self.clk.borrow().eval();
        let last_clk = *self.last_clk.borrow();

        if last_clk == Signal::Low && clk_val == Signal::High {
            *self.state.borrow_mut() = self.d.borrow().eval();
        }
        *self.last_clk.borrow_mut() = clk_val;
        *self.state.borrow()
    }

    fn description(&self) -> String {
        format!(
            "DFlipFlop({:?})", *self.state.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for ClockGate {
    fn eval(&self) -> Signal {
        *self.state.borrow()
    }

    fn description(&self) -> String {
        format!(
            "Clock({:?})",
            *self.state.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}