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

// pub enum Opcode {
//     CMov, Load, Stor, ADD, MULT, DIV, NAND, HALT, Map, UnMap, Output, Input, LPro, LVal
// }

/// structure containing a parsed instruction
pub struct Dinst {
    pub op: u32,
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub val: u32
}

impl Dinst {

    /// returns a u32 bit word based on its field from an instruction word

    pub fn geta(&mut self, instruction: &u32) {
        self.a = (instruction >> RA.lsb) & ((1 << RA.width) - 1)
    }
    pub fn getb(&mut self, instruction: &u32) {
        self.b = (instruction >> RB.lsb) & ((1 << RB.width) - 1)
    }
    pub fn getc(&mut self, instruction: &u32) {
        self.c = (instruction >> RC.lsb) & ((1 << RC.width) - 1)
    }
    pub fn getv(&mut self, instruction: &u32) {
        self.val = (instruction >> VL.lsb) & ((1 << VL.width) - 1)
    }
    pub fn geta2(&mut self, instruction: &u32) {
        self.a = (instruction >> RL.lsb) & ((1 << RL.width) - 1)
    }

    // returns the opcode
    pub fn op(&mut self, instruction: &u32) {
        self.op = (instruction >> OP.lsb) & mask(OP.width)
    }
    
}

