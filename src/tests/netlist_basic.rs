#[warn(unused_imports)]
use crate::circuit::netlist::{GateKind, NetList};
#[test]
fn const_and_not() {
    let mut nl = NetList::default();

    nl.gates.insert("a".into(), GateKind::Const(true));
    nl.gates.insert("b".into(), GateKind::Not("a".into()));
    nl.outputs.push("b".into());

    nl.propagate();
    let out = nl.outputs();
    assert_eq!(out["b"], false)
}

#[test]
fn ha_netlist() {
    let mut nl = NetList::default();

    nl.add_const("a", true).add_const("b", true);

    nl.add_halfadder("a", "b", "sum", "carry");
    nl.outputs.extend(["sum".into(), "carry".into()]);

    nl.propagate();
    let o = nl.outputs();

    assert_eq!(o["sum"], false);
    assert_eq!(o["carry"], true);

}

#[test]
fn dff_counter_stable() {
    let mut n = NetList::default();
    n.add_const("clk_low", false)
     .add_clock("clk",  false)      // zegar
     .add_dflipflop("q", "clk", false);

    n.outputs.push("q".into());

    for _ in 0..32 {
        n.tick("clk");
        n.propagate();
        println!("q = {}", n.outputs()["q"]);
    }
}

#[test]
fn interactive_io(){
    let mut nl = NetList::default();
    nl.add_input("A", false)
        .add_input("B", false)
        .add_and("R", "A", "B")
        .outputs.push("R".into());

    assert_eq!(nl.read_outputs()["R"],false);

    nl.set_input("A",  true).unwrap();
    assert_eq!(nl.read_outputs()["R"], false);

    nl.set_input("B", true).unwrap();
    assert_eq!(nl.read_outputs()["R"], true);

    assert!(nl.set_input("R", false).is_err());
    assert!(matches!(nl.set_input("R", false), Err(_)));

}
