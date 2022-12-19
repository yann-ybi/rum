use super::machine::UM;

type Umi = u32;
/// filed of a u32 bit word
pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 {(1 << bits) - 1}

/// returns a u32 bit word based on its field from an instruction word
pub fn get(field: &Field, instruction: Umi) -> Option<u32> {
    Some((instruction >> field.lsb) & mask(field.width))
}
// returns the opcode
pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

// pub enum Opcode {
//     CMov, Load, Stor, ADD, MULT, DIV, NAND, HALT, Map, UnMap, Output, Input, LPro, LVal
// }

/// structure containing a parsed instruction
pub struct Dinst {
    pub op: u32,
    pub a: Option<u32>,
    pub b: Option<u32>,
    pub c: Option<u32>,
    pub val: Option<u32>
}

impl Dinst {
    // parse an instruction with the op, a, b, c, val, makes a Dinst struct out of it and calls the appropiate machine function for execution
    pub fn disassemble(machine: &mut UM) {
        let inst = machine.memory.get(0_u32, machine.prog_counter as u32).unwrap();
        match op( inst) {
            0 => {
                machine.cdmov(Dinst {
                    op: 0, a: get(&RA, inst), b: get(&RB, inst), c: get(&RC, inst), val: None
                })
            },
            1 => {
                machine.sload(Dinst {
                    op: 1, a: get(&RA, inst), b: get(&RB, inst), c: get(&RC, inst), val: None
                }).unwrap()
            },
            2 => {
                machine.store(Dinst {
                    op: 2, a: get(&RA, inst), b: get(&RB, inst), c: get(&RC, inst), val: None
                }).unwrap()
            },
            3 => {
                machine.add(Dinst {
                    op: 3, a: get(&RA, inst), b: get(&RB, inst), c: get(&RC, inst), val: None
                })
            },
            4 => {
                machine.mult(Dinst {
                    op: 4, a: get(&RA, inst), b: get(&RB, inst), c: get(&RC, inst), val: None
                })
            },
            5 => {
                machine.div(Dinst {
                    op: 5, a: get(&RA, inst), b: get(&RB, inst), c: get(&RC, inst), val: None
                }).unwrap()
            },
            6 => {
                machine.nand(Dinst {
                    op: 6, a: get(&RA, inst), b: get(&RB, inst), c: get(&RC, inst), val: None
                })
            },
            7 => {
                machine.halt()
            },
            8 => {
                machine.map(Dinst {
                    op: 8, a: None, b: get(&RB, inst), c: get(&RC, inst), val: None
                })
            },
            9 => {
                machine.unmap(Dinst {
                    op: 9, a: None, b: None, c: get(&RC, inst), val: None
                }).unwrap()
            },
            10 => {
                machine.output(Dinst {
                    op: 10, a: None, b: None, c: get(&RC, inst), val: None
                }).unwrap()
            },
            11 => {
                machine.input(Dinst {
                    op: 11, a: None, b: None, c: get(&RC, inst), val: None
                }).unwrap()
            },
            12 => {
                machine.pload(Dinst {
                    op: 12, a: None, b: get(&RB, inst), c: get(&RC, inst), val: None
                }).unwrap()
            },
            13 => {
                machine.vload(Dinst {
                    op: 13, a: get(&RL, inst), b: None, c: get(&RC, inst), val: get(&VL, inst)
                })
            },
            _ => { panic!()}
        }
    }
}

