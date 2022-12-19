
use core::panic;
use std::io::Read;

use crate::machine::registers::registers::{CPU};
use crate::machine::memory::memory::{Memory};
use crate::machine::dinst::{Dinst};

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    let range = 1 << width;
    if  n < range {
        true
    } else{false}
}

/// Universal machine u32 bits structure containing, a memory, registers and a program counter
pub struct UM {
    pub registers: CPU,
    pub memory: Memory,
    pub prog_counter: usize
}

impl  UM {
    /// function constructor for a new Universial machine in its initial state
    pub fn new(instructions: Vec<u32>) -> Self {
        UM {
            registers: CPU::new(),
            memory: Memory::new(instructions),
            prog_counter: 0
        }
    }
    pub fn allocate(&mut self, len: usize) -> usize {
        self.memory.allocate(len)
    }
    pub fn deallocate(&mut self, id: usize) {
        self.memory.deallocate(id)
    }
    
    /// if $r[C] != 0 then $r[A] := $r[B]
    pub fn cdmov(&mut self, inst: Dinst) {

        self.prog_counter += 1;

        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);

        if instc != 0 {
            self.registers.write(instb, inst.a)
        }
    }
    /// $r[A] := $m[$r[B]][$r[C]]
    pub fn sload(&mut self, inst: Dinst) {
        
        self.prog_counter += 1;

        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);

        self.registers.write(self.memory.get(instb, instc), inst.a)
    
    }
    /// $m[$r[A]][$r[B]] := $r[C]
    pub fn store(&mut self, inst: Dinst) {

        self.prog_counter += 1;

        let insta = self.registers.read(inst.a);
        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);


        self.memory.set(insta, instb, instc)
    }
    /// $r[A] := ($r[B] + $r[C]) mod 2 ^ 32
    pub fn add(&mut self, inst: Dinst) {
        self.prog_counter += 1;

        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);

        self.registers.write(instb.wrapping_add(instc), inst.a)
    }
    /// $r[A] := ($r[B] × $r[C]) mod 2 ^ 32
    pub fn mult(&mut self, inst: Dinst) {
        self.prog_counter += 1;
        
        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);

        self.registers.write(instb.wrapping_mul(instc), inst.a)
    }
    /// $r[A] := ($r[B] ÷ $r[C]) (integer division)
    pub fn div(&mut self, inst: Dinst) {
        self.prog_counter += 1;

        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);


        if instc == 0 {
            panic!()
        } else {
            self.registers.write(instb.wrapping_div(instc), inst.a)
       }
    }
    /// $r[A] :=¬($r[B]∧$r[C])
    pub fn nand(&mut self, inst: Dinst) {
        self.prog_counter += 1;

        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);


        self.registers.write(!(instb & instc), inst.a)
    }
    /// Computation stops
    pub fn halt(&mut self) {
        std::process::exit(0)
    }
    /// new segment is created with a number of words equal to the value in $r[C], 
    /// words  initialized to zero
    /// the new segment is mapped as $m[$r[B]].
    /// A bit pattern that is not all zeroes and does not identify any currently mapped segment is placed in $r[B]
    pub fn map(&mut self, inst: Dinst) {
        self.prog_counter += 1;

        let instc = self.registers.read(inst.c);

        let allo_id = self.allocate(instc as usize) as u32;
        self.registers.write(allo_id, inst.b);

    }
    ///  The segment $m[$r[C]] is unmapped
    /// Future Map Segment instructions may reuse the identifier $r[C].
    pub fn unmap(&mut self, inst: Dinst) {
        self.prog_counter += 1;

        let instc = self.registers.read(inst.c);
        self.deallocate(instc as usize)
    }
    /// The value in $r[C] is displayed on the I/O
    ///  Only values from 0 to 255 are allowed.
    pub fn output(&mut self, inst: Dinst) {

        let instc = self.registers.read(inst.c);

        self.prog_counter += 1;
        if fitsu(inst.c.unwrap() as u64, 8) {
            std::io::Write::write(&mut std::io::stdout(), &[instc as u8]).unwrap();
            
        } else {panic!()}
    }
    ///  UM waits for input on the I/O device
    // $r[c] is loaded with the input
    // must be a value from 0 to 255
    // end of input has been signaled, $r[C] is loaded with a full 32-bit word in which every bit is 1
    pub fn input(&mut self, inst: Dinst) {
        self.prog_counter += 1;

        match std::io::stdin().bytes().next().unwrap().unwrap() {
            o  => {
                if o as char == '\n' {
                    self.registers.write(std::u32::MAX, inst.c);
                }
                else if fitsu(o.try_into().unwrap(), 8) {
                    self.registers.write(o.try_into().unwrap(), inst.c);
                }
            }

        } 
    }
    ///  Segment $m[$r[B]] is duplicated
    /// m[0] = duplicate
    /// program counter points to r[c]
    pub fn pload(&mut self, inst: Dinst) {
        self.prog_counter += 1;

        let instb = self.registers.read(inst.b);
        let instc = self.registers.read(inst.c);
        
        if self.registers.read(inst.b) != 0 {
            if (self.memory.segs[instb as usize]).clone() != Vec::new(){

                    let duplicate = self.memory.segs[instb as usize].clone();
                    self.memory.segs[0] = duplicate;
                    self.prog_counter = instc as usize;
                
            }
            else {panic!()}
        } else {
            self.prog_counter = instc as usize;
        }
    }
    /// r[a] = Value
    pub fn vload(&mut self, inst: Dinst) {

        self.prog_counter += 1;

        self.registers.write(inst.val.unwrap(), inst.a)
    }

}
