use std::collections::HashMap;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

pub type GateId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateKind {
    Const(bool),
    Input(bool),
    And([GateId; 2]),
    Or([GateId; 2]),
    Xor([GateId; 2]),
    Nor([GateId; 2]),
    Nand([GateId; 2]),
    Not(GateId),

    SRLatch {set: GateId, reset: GateId, q: bool},
    DLatch {d: GateId, enable: GateId, q: bool},
    DFlipFlop {d: GateId, clk: GateId, q: bool, last_clk: bool},
    Clock(bool),
    Wire(GateId),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NetList {
    pub gates: HashMap<GateId, GateKind>,
    pub outputs: Vec<GateId>,
}

impl NetList {

    fn eval_gate_with_memo<'a>(
        &'a self,
        id: &'a GateId,
        memo: &mut HashMap<&'a GateId, bool>,
        visiting: &mut HashSet<&'a GateId>,
    ) -> bool {
        if let Some(&v) = memo.get(id) {
            return v;
        }

        if !visiting.insert(id) {

            return *memo.get(id).unwrap_or(&false);
        }

        let val = match &self.gates[id] {
            GateKind::Const(v) | GateKind::Input(v) | GateKind::Clock(v) => *v,
            GateKind::Wire(src) => self.eval_gate_with_memo(src, memo, visiting),
            GateKind::And([a,b])  => self.eval_gate_with_memo(a,memo,visiting) & self.eval_gate_with_memo(b,memo,visiting),
            GateKind::Or([a,b]) => self.eval_gate_with_memo(a,memo,visiting) | self.eval_gate_with_memo(b,memo,visiting),
            GateKind::Xor([a,b]) => self.eval_gate_with_memo(a,memo,visiting) ^ self.eval_gate_with_memo(b,memo,visiting),
            GateKind::Nand([a,b]) => !(self.eval_gate_with_memo(a,memo,visiting) & self.eval_gate_with_memo(b,memo,visiting)),
            GateKind::Nor([a,b]) => !(self.eval_gate_with_memo(a,memo,visiting) | self.eval_gate_with_memo(b,memo,visiting)),
            GateKind::Not(a) => !self.eval_gate_with_memo(a,memo,visiting),
            GateKind::DFlipFlop { q, ..} => *q,
            GateKind::DLatch { q, ..} => *q,
            GateKind::SRLatch { q, ..} => *q,
        };

        visiting.remove(id);
        memo.insert(id, val);
        val
    }

    fn eval_gate(&self, id: &GateId) -> bool {
        let mut memo = HashMap::new();
        let mut visiting = HashSet::new();
        self.eval_gate_with_memo(id, &mut memo, &mut visiting)
    }

    pub fn propagate(&mut self) {
        for _ in 0..10 {
            let mut changed = false;

            let updates: Vec<(GateId, bool, Option<bool>)> = self.gates.iter()
                .filter_map(|(id, g)| {
                    let new_val = self.eval_gate(id);
                    match g {
                        GateKind::DFlipFlop {  last_clk, .. } => Some((id.clone(), new_val, Some(*last_clk))),
                        GateKind::DLatch    {  .. } => Some((id.clone(), new_val, None)),
                        GateKind::SRLatch   {  .. } => Some((id.clone(), new_val, None)),
                        _ => None
                    }
                })
                .collect();

                for (id, v, _) in updates {
                    let clk_lvl = if let Some(GateKind::DFlipFlop { clk, .. }) = self.gates.get(&id) {
                        self.eval_gate(clk)
                    } else { false };
                
                    if let Some(g) = self.gates.get_mut(&id) {
                        match g {
                            GateKind::DFlipFlop { q, last_clk, .. } => {
                                changed |= *q != v || *last_clk != clk_lvl;
                                *q       = v;
                                *last_clk = clk_lvl;
                            }
                            GateKind::DLatch { q, ..  }
                          | GateKind::SRLatch { q, .. } => {
                                changed |= *q != v;
                                *q = v;
                            }
                            _ => {}
                        }
                    }
                }

            if !changed { break; }
        }
    }

    pub fn set_input(&mut self, id: &str, value: bool) -> Result<(), String> {
        match self.gates.get_mut(id) {
            Some(GateKind::Input(v)) => {
                if *v != value {
                    *v = value;
                    self.propagate();
                }
                Ok(())
            }
            Some(_) => Err(format!("Gate '{id}' is not of type Input")),
            None => Err(format!("Gate '{id}' does not exist"))
        }
    }

    pub fn read_outputs(&mut self) -> HashMap<GateId, bool> {
        self.propagate();
        self.outputs()
    }

    pub fn tick(&mut self, clk_id: &str) {
        if let Some(GateKind::Clock(v)) = self.gates.get_mut(clk_id) {
            *v = !*v
        }
    }

    pub fn outputs(&self) -> HashMap<GateId,bool> {
        self.outputs.iter()
            .map(|id| (id.clone(), self.eval_gate(id)))
            .collect()
    }

    pub fn add_const(&mut self, id: &str, v: bool) -> &mut Self{
        self.gates.insert(id.into(), GateKind::Const(v)); self
    }

    pub fn add_input(&mut self, id: &str, v: bool) -> &mut Self {
        self.gates.insert(id.into(), GateKind::Input(v)); self
    }

    pub fn add_not(&mut self, id: &str, src: &str) -> &mut Self{
        self.gates.insert(id.into(), GateKind::Not(src.into())); self
    }

    pub fn add_and(&mut self, id: &str, a: &str, b: &str) -> &mut Self {
        self.gates.insert(id.into(), GateKind::And([a.into(), b.into()])); self
    }

    pub fn add_xor(&mut self, id: &str, a: &str, b: &str) -> &mut Self {
        self.gates.insert(id.into(), GateKind::Xor([a.into(), b.into()])); self
    }

    pub fn add_halfadder(&mut self, a: &str, b: &str, sum_id: &str, carry_id: &str) -> &mut Self {
        self.add_xor(sum_id,a,b);
        self.add_and(carry_id,a,b);
        self
    }

    pub fn add_clock(&mut self, id: &str, init:bool) -> &mut Self {
        self.gates.insert(id.into(), GateKind::Clock(init)); self
    }

    pub fn add_dflipflop(&mut self, q: &str, clk: &str, init:bool) -> &mut Self {
        let d_not = format!("_not_{q}");
        self.gates.insert(
            q.into(),
            GateKind::DFlipFlop { d: (d_not.clone()), clk: (clk.into()), q: (init), last_clk: (false) }
        );
        self.add_not(&d_not, q);
        self
    }
}
