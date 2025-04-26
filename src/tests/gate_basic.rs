use crate::circuit::gate::*;
use crate::circuit::gate::{ClockGate, Dflipflop, FullAdder, HalfAdder};
use std::cell::RefCell;
#[warn(unused_imports)]
use std::rc::Rc;

#[test]
fn test_and_gate() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let b = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let gate = AndGate::new(a, b);
    assert_eq!(gate.eval(), Signal::Low);
}

#[test]
fn test_or_gate() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let b = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let gate = OrGate::new(a, b);
    assert_eq!(gate.eval(), Signal::High)
}

#[test]
fn test_not_gate() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let gate = NotGate::new(a);
    assert_eq!(gate.eval(), Signal::Low)
}

#[test]
fn test_nesting_gate() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let gate = OrGate::new(
        a,
        Rc::new(RefCell::new(NotGate::new(Rc::new(RefCell::new(
            ConstGate::new(Signal::Low),
        ))))),
    );
    assert_eq!(gate.eval(), Signal::High);
}

#[test]
fn test_nesting_xor_gate() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let gate = XorGate::new(
        a,
        Rc::new(RefCell::new(NotGate::new(Rc::new(RefCell::new(
            ConstGate::new(Signal::Low),
        ))))),
    );
    assert_eq!(gate.eval(), Signal::High);
}

#[test]
fn test_nesting_nor_gate() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let gate = NorGate::new(
        a,
        Rc::new(RefCell::new(NotGate::new(Rc::new(RefCell::new(
            ConstGate::new(Signal::High),
        ))))),
    );
    assert_eq!(gate.eval(), Signal::High);
}

#[test]
fn test_nesting_nand_gate() {
    let t = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let f = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));

    assert_eq!(NandGate::new(f.clone(), f.clone()).eval(), Signal::High);
    assert_eq!(NandGate::new(f.clone(), t.clone()).eval(), Signal::High);
    assert_eq!(NandGate::new(t.clone(), f.clone()).eval(), Signal::High);
    assert_eq!(NandGate::new(t.clone(), t.clone()).eval(), Signal::Low);
}

#[test]
fn test_gate_description() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let b = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let gate = AndGate::new(a.clone(), b.clone());
    assert_eq!(gate.description(), "And(Const 1, Const 0)");
}

#[test]
fn test_halfadder() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let b = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let ha = HalfAdder::new(a, b);

    assert_eq!(ha.sum.borrow().eval(), Signal::Low);
    assert_eq!(ha.carry.borrow().eval(), Signal::High);
}

#[test]
fn test_fulladder() {
    let a = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let b = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let cin = Rc::new(RefCell::new(ConstGate::new(Signal::High)));

    let fa = FullAdder::new(a, b, cin);
    assert_eq!(fa.sum.borrow().eval(), Signal::High);
    assert_eq!(fa.carry.borrow().eval(), Signal::High);
}

#[test]
fn test_input_gate() {
    let mut input_gate = InputGate::new(false);
    assert_eq!(input_gate.eval(), Signal::Low);

    input_gate.set_signal(true);
    assert_eq!(input_gate.eval(), Signal::High);
}

#[test]
fn test_op_gate() {
    let input_gate = Rc::new(RefCell::new(ConstGate::new(Signal::High)));
    let output_gate = OutputGate::new(input_gate.clone());

    assert_eq!(output_gate.eval(), Signal::High);
    assert_eq!(output_gate.description(), "Output (Const 1)")
}

#[test]
fn test_srlatch() {
    let set = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let reset = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));

    let latch = SRLatch::new(set.clone(), reset.clone());

    assert_eq!(latch.eval(), Signal::Low);

    set.borrow_mut().set_level(Signal::High);
    reset.borrow_mut().set_level(Signal::Low);
    assert_eq!(latch.eval(), Signal::High);

    set.borrow_mut().set_level(Signal::Low);
    assert_eq!(latch.eval(), Signal::High);

    reset.borrow_mut().set_level(Signal::High);
    assert_eq!(latch.eval(), Signal::Low);

    set.borrow_mut().set_level(Signal::High);
    assert_eq!(latch.eval(), Signal::Low)
}

#[test]
fn test_dlatch() {
    let d = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));
    let enable = Rc::new(RefCell::new(ConstGate::new(Signal::Low)));

    let latch = Dlatch::new(d.clone(), enable.clone());

    assert_eq!(latch.eval(), Signal::Low);

    d.borrow_mut().set_level(Signal::High);
    enable.borrow_mut().set_level(Signal::High);
    assert_eq!(latch.eval(), Signal::High);

    d.borrow_mut().set_level(Signal::Low);
    enable.borrow_mut().set_level(Signal::Low);
    assert_eq!(latch.eval(), Signal::High);

    enable.borrow_mut().set_level(Signal::High);
    assert_eq!(latch.eval(), Signal::Low);
}

#[test]
fn test_d_flip_flop() {
    let d = Rc::new(RefCell::new(InputGate::new(false)));
    let clk = Rc::new(RefCell::new(InputGate::new(false)));

    let ff = Dflipflop::new(d.clone(), clk.clone());

    assert_eq!(ff.eval(), Signal::Low);

    {
        d.borrow_mut().set_signal(true);
        clk.borrow_mut().set_signal(true);
    }
    assert_eq!(ff.eval(), Signal::High);

    {
        d.borrow_mut().set_signal(false);
        clk.borrow_mut().set_signal(false);
    }
    ff.eval();

    {
        clk.borrow_mut().set_signal(true);
    }
    assert_eq!(ff.eval(), Signal::Low);
}

#[test]
fn test_clock() {
    let clk = ClockGate::new();
    assert_eq!(clk.eval(), Signal::Low);

    clk.tick();
    assert_eq!(clk.eval(), Signal::High);

    clk.tick();
    assert_eq!(clk.eval(), Signal::Low);
}
