
use std::io::Read;

use crate::machine::registers::registers::*;
use crate::machine::memory::memory::*;
use crate::machine::dinst::{Dinst};
use crate::machine::error::*;

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
    /// counter points to an memory location based on an offset at id 0
    pub fn set_pcounter(&mut self, offset: u32) -> Result<(), MachError>  {
        self.prog_counter = offset as usize;
        Ok(())
    }
    pub fn advance_pcounter(&mut self) {
        self.prog_counter += 1;
    }
    
    /// if $r[C] != 0 then $r[A] := $r[B]
    pub fn cdmov(&mut self, inst: Dinst) {
        self.advance_pcounter();
        if self.registers.read(inst.c) != 0 {
            self.registers.write(self.registers.read(inst.b), inst.a)
        }
    }
    /// $r[A] := $m[$r[B]][$r[C]]
    pub fn sload(&mut self, inst: Dinst) -> Result<(), MachError> {
        self.advance_pcounter();

        match self.memory.get(self.registers.read(inst.b), self.registers.read(inst.c)) {
            Some(o) => Ok ({
                self.registers.write(o, inst.a)
            }),
            None => Err(MachError::LoadSegmentFailed),
        }
    }
    /// $m[$r[A]][$r[B]] := $r[C]
    pub fn store(&mut self, inst: Dinst) -> Result<(), MachError> {
        self.advance_pcounter();

        match self.memory.set(self.registers.read(inst.a), self.registers.read(inst.b), self.registers.read(inst.c)) {
            Some(_) => Ok(()),
            None => Err(MachError::StoreSegmentFailed),
        }

    }
    /// $r[A] := ($r[B] + $r[C]) mod 2 ^ 32
    pub fn add(&mut self, inst: Dinst) {
        self.advance_pcounter();
        self.registers.write(self.registers.read(inst.b).wrapping_add(self.registers.read(inst.c)), inst.a)
    }
    /// $r[A] := ($r[B] × $r[C]) mod 2 ^ 32
    pub fn mult(&mut self, inst: Dinst) {
        self.advance_pcounter();
        self.registers.write(self.registers.read(inst.b).wrapping_mul(self.registers.read(inst.c)), inst.a)
    }
    /// $r[A] := ($r[B] ÷ $r[C]) (integer division)
    pub fn div(&mut self, inst: Dinst) -> Result<(), MachError> {
        self.advance_pcounter();
        if self.registers.read(inst.c) == 0 {
            Err(MachError::DivisionByZero)
        } else {
            Ok(self.registers.write(self.registers.read(inst.b).wrapping_div(self.registers.read(inst.c)), inst.a))
       }
    }
    /// $r[A] :=¬($r[B]∧$r[C])
    pub fn nand(&mut self, inst: Dinst) {
        self.advance_pcounter();
        self.registers.write(!(self.registers.read(inst.b) & self.registers.read(inst.c)), inst.a)
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
        self.advance_pcounter();
        let new_seg: Vec<u32> = vec![0_u32; self.registers.read(inst.c) as usize];
        
        match self.memory.unmapped_segs.pop_front() {
            Some(o) => {
                self.registers.write(o as u32, inst.b);
                self.memory.segs[o] = Box::new(Some(new_seg));
            },
            None => {
                self.registers.write(self.memory.segs.len() as u32, inst.b);
                self.memory.segs.push(Box::new(Some(new_seg)));
            }
        }
    }
    ///  The segment $m[$r[C]] is unmapped
    /// Future Map Segment instructions may reuse the identifier $r[C].
    pub fn unmap(&mut self, inst: Dinst) -> Result<(), MachError> {
        self.advance_pcounter();

        if self.registers.read(inst.c) == 0 {
            Err(MachError::NotFoundUnmapSegment)
        }else {
            match *self.memory.segs[self.registers.read(inst.c) as usize] {
                None => Err(MachError::NotFoundUnmapSegment),
                Some(_) => Ok({
                    self.memory.segs[self.registers.read(inst.c) as usize] = Box::new(None);
                    self.memory.unmapped_segs.push_back(self.registers.read(inst.c) as usize);
                }),
            }
        }
    }
    /// The value in $r[C] is displayed on the I/O
    ///  Only values from 0 to 255 are allowed.
    pub fn output(&mut self, inst: Dinst) -> Result<(), MachError> {
        self.advance_pcounter();
        if fitsu(inst.c.unwrap() as u64, 8) {
            std::io::Write::write(&mut std::io::stdout(), &[self.registers.read(inst.c) as u8]).unwrap();
            Ok(())
        } else {Err(MachError::UnvalidOutput)}
    }
    ///  UM waits for input on the I/O device
    // $r[c] is loaded with the input
    // must be a value from 0 to 255
    // end of input has been signaled, $r[C] is loaded with a full 32-bit word in which every bit is 1
    pub fn input(&mut self, inst: Dinst) -> Result<(), MachError> {
        self.advance_pcounter();
        match std::io::stdin().bytes().next().unwrap() {
            Ok(o)  => {
                if o as char == '\n' {
                    Ok(self.registers.write(std::u32::MAX, inst.c))
                }
                else if fitsu(o.try_into().unwrap(), 8) {
                    Ok(self.registers.write(o.try_into().unwrap(), inst.c))
                }else {
                    Err(MachError::UnvalidInput)
                }
            }
            Err(_) => {
                Err(MachError::UnvalidInput)
            }
        } 
    }
    ///  Segment $m[$r[B]] is duplicated
    /// m[0] = duplicate
    /// program counter points to r[c]
    pub fn pload(&mut self, inst: Dinst) -> Result<(), MachError> {
        self.advance_pcounter();
        match *(self.memory.segs[self.registers.read(inst.b) as usize]).clone() {
            Some(_) => Ok({
                let duplicate = Box::new(*(self.memory.segs[self.registers.read(inst.b) as usize]).clone());
                self.memory.segs[0] = duplicate;
                self.set_pcounter( self.registers.read(inst.c)).unwrap();
            }),
            None => {Err(MachError::NotFoundLoadProgramSegment)},
        }
    }
    /// r[a] = Value
    pub fn vload(&mut self, inst: Dinst) {
        self.advance_pcounter();
        self.registers.write(inst.val.unwrap(), inst.a)
    }

}
