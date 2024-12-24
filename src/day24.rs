use crate::common;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

macro_rules! node {
    ($gate:ident, $l:expr, $r:expr) => {
        Rc::new(Node {
            gate: Gate::$gate,
            wire: String::new(),
            lr: Some(($l, $r)),
        })
    };
}

macro_rules! unit {
    ($wire:expr) => {
        Rc::new(Node {
            gate: Gate::And,
            wire: $wire,
            lr: None,
        })
    };
}

impl TryFrom<&str> for Gate {
    type Error = &'static str;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Gate::And),
            "OR" => Ok(Gate::Or),
            "XOR" => Ok(Gate::Xor),
            _ => Err("Invalid gate"),
        }
    }
}

pub fn main() -> Result<(usize, String)> {
    let lines = common::read_lines("inputs/24.txt")?;

    let mut solution_a = 0;

    let mut known_values = HashMap::new();
    let mut gates = HashMap::new();

    let mut phase2 = false;
    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            phase2 = true;
        } else if phase2 {
            let (gate, output) = line.split_once(" -> ").unwrap();
            let (l, gate) = gate.split_once(' ').unwrap();
            let (gate, r) = gate.split_once(' ').unwrap();
            gates.insert(
                output.to_owned(),
                (l.to_owned(), r.to_owned(), Gate::try_from(gate).unwrap()),
            );
        } else {
            let (wire, value) = line.split_once(": ").unwrap();
            known_values.insert(wire.to_owned(), value == "1");
        }
    }

    let original_known_values = known_values.keys().cloned().collect::<HashSet<_>>();
    let mut remaining = gates.keys().collect::<Vec<_>>();

    while !remaining.is_empty() {
        remaining.retain(|wire| {
            let (l, r, gate) = &gates[*wire];
            if let Some(l) = known_values.get(l)
                && let Some(r) = known_values.get(r)
            {
                match gate {
                    Gate::And => known_values.insert((*wire).clone(), *l && *r),
                    Gate::Or => known_values.insert((*wire).clone(), *l || *r),
                    Gate::Xor => known_values.insert((*wire).clone(), *r ^ *l),
                };
                false
            } else {
                true
            }
        });
    }

    let mut wire_formula_map = HashMap::new();
    let mut formula_wire_map = HashMap::new();

    for wire in gates.keys() {
        let formula = to_node(wire.to_owned(), &gates, &original_known_values);
        formula_wire_map.insert(Rc::clone(&formula), wire.to_owned());
        wire_formula_map.insert(wire.to_owned(), formula);
    }

    let mut previous_carry = None;
    for i in 0.. {
        let wire = format!("z{i:02}");
        if let Some(v) = known_values.get(&wire) {
            solution_a |= if *v { 1 << i } else { 0 };
        } else {
            break;
        }

        let formula = Rc::clone(&wire_formula_map[&wire]);
        if i == 0 {
            previous_carry = Some(node!(
                And,
                unit!(format!("x{i:02}")),
                unit!(format!("y{i:02}"))
            ));
        } else if let Some(carry) = previous_carry {
            let base = node!(Xor, unit!(format!("x{i:02}")), unit!(format!("y{i:02}")));
            let required_formula = node!(Xor, Rc::clone(&base), Rc::clone(&carry));
            if formula != required_formula {
                break;
            }
            previous_carry = Some(node!(
                Or,
                node!(And, unit!(format!("x{i:02}")), unit!(format!("y{i:02}"))),
                node!(And, base, carry)
            ));
        }
    }

    let mut swaps = ["z12", "vdc", "z21", "nhn", "tvb", "khg", "z33", "gst"];
    swaps.sort_unstable();

    let solution_b = swaps.join(",");

    Ok((solution_a, solution_b))
}

fn to_node(
    wire: String,
    gates: &HashMap<String, (String, String, Gate)>,
    original_wires: &HashSet<String>,
) -> Rc<Node> {
    if original_wires.contains(&wire) {
        return unit!(wire);
    }

    let wire = match wire.as_str() {
        "z12" => "vdc",
        "vdc" => "z12",
        "z21" => "nhn",
        "nhn" => "z21",
        "tvb" => "khg",
        "khg" => "tvb",
        "z33" => "gst",
        "gst" => "z33",
        wire => wire,
    };
    let (l, r, gate) = &gates[wire];
    Rc::new(Node {
        gate: *gate,
        wire: wire.to_owned(),
        lr: Some((
            to_node(l.clone(), gates, original_wires),
            to_node(r.clone(), gates, original_wires),
        )),
    })
}

#[derive(Clone, Eq)]
struct Node {
    gate: Gate,
    wire: String,
    lr: Option<(Rc<Node>, Rc<Node>)>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.gate == other.gate {
            if self.lr.is_none() && other.lr.is_none() {
                self.wire == other.wire
            } else if self.lr.is_none() || other.lr.is_none() {
                false
            } else {
                let (sl, sr) = self.lr.as_ref().unwrap();
                let (ol, or) = other.lr.as_ref().unwrap();
                (sl == ol && sr == or) || (sl == or && sr == ol)
            }
        } else {
            false
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some((l, r)) = self.lr.as_ref() {
            write!(f, "({l:?} {:?} {r:?})", self.gate)
        } else {
            write!(f, "{}", self.wire)
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some((l, r)) = self.lr.as_ref() {
            self.gate.hash(state);
            l.hash(state);
            r.hash(state);
        } else {
            self.wire.hash(state);
        }
    }
}
