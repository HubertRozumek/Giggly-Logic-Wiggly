use std::rc::Rc;
use std::{cell::RefCell, fmt::Debug};
use std::any::Any;



// trait implementation



pub trait Gate: Debug{
    fn eval(&self) -> bool;
    fn description(&self) -> String;

    fn as_any(&mut self) -> &mut dyn Any;
}


// gates structs



#[derive(Debug)]
pub struct ConstGate {
    signal: bool,
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
    last_q: RefCell<bool>,
}

#[derive(Debug)]
pub struct Dlatch {
    d: Rc<RefCell<dyn Gate>>,
    enable: Rc<RefCell<dyn Gate>>,
    state: RefCell<bool>,
}

#[derive(Debug)]
pub struct Dflipflop {
    d: Rc<RefCell<dyn Gate>>,
    clk: Rc<RefCell<dyn Gate>>,
    state: RefCell<bool>,
    last_clk: RefCell<bool>,
}

#[derive(Debug)]
pub struct ClockGate{
    state: RefCell<bool>,
}



// Constructors



impl ConstGate {
    pub fn new(s: bool) -> Self {
        Self {signal: s}
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
        Self { set: (set), reset: (reset), last_q: (RefCell::new(false)) }
    }
}

impl Dlatch {
    pub fn new(d: Rc<RefCell<dyn Gate>>, enable: Rc<RefCell<dyn Gate>>) -> Self {
        Self { d: (d), enable: (enable), state: (RefCell::new(false)) }
    }
}

impl Dflipflop {
    pub fn new(d: Rc<RefCell<dyn Gate>>, clk: Rc<RefCell<dyn Gate>>) -> Self {
        Self { d: (d), clk: (clk), state: (RefCell::new(false)), last_clk: (RefCell::new(false)) }
    }
}

impl ClockGate {
    pub fn new() -> Self {
        Self { state: RefCell::new(false) }
    }

    pub fn tick(&self) {
        let mut state = self.state.borrow_mut();
        *state = !*state;
    }
}


// gate implementation 



impl Gate for ConstGate{
    fn eval(&self) -> bool {
        self.signal
    }

    fn description(&self) -> String {
        format!("Const {}",self.signal)
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for InputGate{
    fn eval(&self) -> bool {
        self.signal
    }

    fn description(&self) -> String {
        format!("Input {}",self.signal)
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for OutputGate {
    fn eval(&self) -> bool {
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
    fn eval(&self) -> bool {
        self.signal_one.borrow().eval() && self.signal_two.borrow().eval()
    }

    fn description(&self) -> String{
        format!("AND({}, {})",
         self.signal_one.borrow().description(), 
         self.signal_two.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for OrGate {
    fn eval(&self) -> bool {
        self.signal_one.borrow().eval() || self.signal_two.borrow().eval()
    }

    fn description(&self) -> String{
        format!("OR({}, {})",
         self.signal_one.borrow().description(), 
         self.signal_two.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for NotGate {
    fn eval(&self) -> bool {
        !self.signal.borrow().eval()
    }

    fn description(&self) -> String{
        format!("NOT({})",
        self.signal.borrow().description())
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for XorGate {
    fn eval(&self) -> bool {
        self.signal_one.borrow().eval() != self.signal_two.borrow().eval()
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
    fn eval(&self) -> bool {
        !(self.signal_one.borrow().eval() || self.signal_two.borrow().eval())
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
    fn eval(&self) -> bool {
        !(self.signal_one.borrow().eval() && self.signal_two.borrow().eval())
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
    fn eval(&self) -> bool {
        let s = self.set.borrow().eval();
        let r = self.reset.borrow().eval();

        let new_q = match (s, r) {
            (true, false) => true,
            (false, true) => false,
            (false, false) => *self.last_q.borrow(),
            (true, true) => {
                *self.last_q.borrow()
            }
        };

        *self.last_q.borrow_mut() = new_q;
        new_q
    }

    fn description(&self) -> String {
        format!(
            "SRLatch(Set: {}, Reset: {}, Q: {})",
            self.set.borrow().description(),
            self.reset.borrow().description(),
            self.last_q.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for Dlatch {
    fn eval(&self) -> bool {
        if self.enable.borrow().eval(){
            let value = self.d.borrow().eval();
            *self.state.borrow_mut() = value;
        }
        *self.state.borrow()
    }

    fn description(&self) -> String {
        format!(
            "DLatch(Set: {}, Reset: {}, Q: {})",
            self.d.borrow().description(),
            self.enable.borrow().description(),
            self.state.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for Dflipflop {
    fn eval(&self) -> bool {
        let clk_val = self.clk.borrow().eval();
        let last_clk = *self.last_clk.borrow();

        if !last_clk && clk_val {
            let d = self.d.borrow().eval();
            *self.state.borrow_mut() = d;
        }

        *self.last_clk.borrow_mut() = clk_val;
        *self.state.borrow()
    }

    fn description(&self) -> String {
        format!(
            "DFlipFlop(D: {}, CLK: {}, Q: {})",
            self.d.borrow().description(),
            self.clk.borrow().description(),
            self.state.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Gate for ClockGate {
    fn eval(&self) -> bool {
        
        *self.state.borrow()
    }

    fn description(&self) -> String {
        format!(
            "Clock({})",
            self.state.borrow()
        )
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

// Tests



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate(){
        let a = Rc::new(RefCell::new(ConstGate::new(true)));
        let b = Rc::new(RefCell::new(ConstGate::new(false)));
        let gate = AndGate::new(a,b);
        assert_eq!(gate.eval(),false);
    }

    #[test]
    fn test_or_gate(){
        let a = Rc::new(RefCell::new(ConstGate::new(true)));
        let b = Rc::new(RefCell::new(ConstGate::new(false)));
        let gate = OrGate::new(a,b);
        assert_eq!(gate.eval(),true)
    }

    #[test]
    fn test_not_gate(){
        let a = Rc::new(RefCell::new(ConstGate::new(true)));
        let gate = NotGate::new(a);
        assert_eq!(gate.eval(),false)
    }

    #[test]
    fn test_nesting_gate(){
        let a = Rc::new(RefCell::new(ConstGate::new(false)));
        let gate = OrGate::new(a,Rc::new(RefCell::new(NotGate::new(Rc::new(RefCell::new(ConstGate::new(false)))))));
        assert_eq!(gate.eval(),true);  
    }

    #[test]
    fn test_nesting_xor_gate(){
        let a = Rc::new(RefCell::new(ConstGate::new(false)));
        let gate = XorGate::new(a,Rc::new(RefCell::new(NotGate::new(Rc::new(RefCell::new(ConstGate::new(false)))))));
        assert_eq!(gate.eval(),true);  
    }

    #[test]
    fn test_nesting_nor_gate(){
        let a = Rc::new(RefCell::new(ConstGate::new(false)));
        let gate = NorGate::new(a,Rc::new(RefCell::new(NotGate::new(Rc::new(RefCell::new(ConstGate::new(true)))))));
        assert_eq!(gate.eval(),true);  
    }

    #[test]
    fn test_nesting_nand_gate(){
        let t = Rc::new(RefCell::new(ConstGate::new(true)));
        let f = Rc::new(RefCell::new(ConstGate::new(false)));

        assert_eq!(NandGate::new(f.clone(), f.clone()).eval(), true);
        assert_eq!(NandGate::new(f.clone(), t.clone()).eval(), true); 
        assert_eq!(NandGate::new(t.clone(), f.clone()).eval(), true); 
        assert_eq!(NandGate::new(t.clone(), t.clone()).eval(), false); 
    }

    #[test]
    fn test_gate_description() {
        let a = Rc::new(RefCell::new(ConstGate::new(true)));
        let b = Rc::new(RefCell::new(ConstGate::new(false)));
        let gate = AndGate::new(a.clone(), b.clone());
        assert_eq!(gate.description(), "AND(Const true, Const false)");
    }

    #[test]
    fn test_halfadder(){
        let a = Rc::new(RefCell::new(ConstGate::new(true)));
        let b = Rc::new(RefCell::new(ConstGate::new(true)));
        let ha = HalfAdder::new(a,b);

        assert_eq!(ha.sum.borrow().eval(), false);
        assert_eq!(ha.carry.borrow().eval(), true);
    }

    #[test]
    fn test_fulladder(){
        let a = Rc::new(RefCell::new(ConstGate::new(true)));
        let b = Rc::new(RefCell::new(ConstGate::new(true)));
        let cin = Rc::new(RefCell::new(ConstGate::new(true)));

        let fa = FullAdder::new(a,b,cin);
        assert_eq!(fa.sum.borrow().eval(), true);
        assert_eq!(fa.carry.borrow().eval(), true);
    }

    #[test]
    fn test_input_gate(){
        let mut input_gate = InputGate::new(false);
        assert_eq!(input_gate.eval(),false);

        input_gate.set_signal(true);
        assert_eq!(input_gate.eval(),true);
    }

    #[test]
    fn test_op_gate(){
        let input_gate = Rc::new(RefCell::new(ConstGate::new(true)));
        let output_gate = OutputGate::new(input_gate.clone());

        assert_eq!(output_gate.eval(),true);
        assert_eq!(output_gate.description(),"Output (Const true)")
    }
    
    #[test]
    fn test_srlatch(){
        let set = Rc::new(RefCell::new(ConstGate::new(false)));
        let reset = Rc::new(RefCell::new(ConstGate::new(false)));

        let latch = SRLatch::new(set.clone(),reset.clone());

        assert_eq!(latch.eval(), false);

        set.borrow_mut().signal = true;
        reset.borrow_mut().signal = false;
        assert_eq!(latch.eval(), true);

        set.borrow_mut().signal = false;
        assert_eq!(latch.eval(),true);

        reset.borrow_mut().signal = true;
        assert_eq!(latch.eval(), false);

        set.borrow_mut().signal = true;
        assert_eq!(latch.eval(), false)
    }

    #[test]
    fn test_dlatch(){
        let d = Rc::new(RefCell::new(ConstGate::new(false)));
        let enable = Rc::new(RefCell::new(ConstGate::new(false)));

        let latch = Dlatch::new(d.clone(), enable.clone());

        assert_eq!(latch.eval(), false);

        d.borrow_mut().signal = true;
        enable.borrow_mut().signal = true;
        assert_eq!(latch.eval(),true);

        d.borrow_mut().signal = false;
        enable.borrow_mut().signal = false;
        assert_eq!(latch.eval(),true);

        enable.borrow_mut().signal = true;
        assert_eq!(latch.eval(),false);
    }

    #[test]
    fn test_d_flip_flop() {
        let d = Rc::new(RefCell::new(InputGate::new(false)));
        let clk = Rc::new(RefCell::new(InputGate::new(false)));

        let ff = Dflipflop::new(d.clone(), clk.clone());

        assert_eq!(ff.eval(), false);

        {
            d.borrow_mut().set_signal(true);
            clk.borrow_mut().set_signal(true);
        }
        assert_eq!(ff.eval(), true);

        {
            d.borrow_mut().set_signal(false);
            clk.borrow_mut().set_signal(false);
        }
        ff.eval();

        {
        clk.borrow_mut().set_signal(true);
        }
        assert_eq!(ff.eval(), false);
    }

    #[test]
    fn test_clock() {
        let clk = ClockGate::new();
        assert_eq!(clk.eval(),false);
        
        clk.tick();
        assert_eq!(clk.eval(), true);

        clk.tick();
        assert_eq!(clk.eval(), false);
    }

}

