
use core::panic;
use std::io::Read;

use crate::machine::registers::registers::{CPU};
use crate::machine::memory::memory::{Memory};
use crate::machine::dinst::{Dinst};


/// Universal machine u32 bits structure containing, a memory, registers and a program counter
pub struct UM {
    registers: CPU,
    memory: Memory,
    prog_counter: usize,
    dinst: Dinst,
}

impl  UM {
    /// function constructor for a new Universial machine in its initial state
    pub fn new(instructions: Vec<u32>) -> Self {
        UM {
            registers: CPU::new(),
            memory: Memory::new(instructions),
            prog_counter: 0,
            dinst: Dinst { op: 0, a: 0, b: 0, c: 0, val: 0}
        }
    }
    fn get_i(&self, offset: usize) -> u32 {
        self.memory.get_i(offset)
    }
    fn op(&mut self, instruction: &u32) {
        self.dinst.op(instruction);
    }
    fn allocate(&mut self, len: usize) -> usize {
        self.memory.allocate(len)
    }
    fn deallocate(&mut self, id: usize) {
        self.memory.deallocate(id)
    }
    fn read(&self, register: u32) -> u32 {
        self.registers.read(register)
    }
    fn write(&mut self, val: u32, register: u32) {
        self.registers.write(val, register)
    }
    fn get(&self, id: u32, offset: u32) -> u32 {
        self.memory.get(id, offset)
    }
    /// if $r[C] != 0 then $r[A] := $r[B]
    fn cdmov(&mut self) {

        self.prog_counter += 1;

        let instb = self.read(self.dinst.b);
        let instc = self.read(self.dinst.c);

        if instc != 0 {
            self.write(instb, self.dinst.a)
        }
    }
    /// $r[A] := $m[$r[B]][$r[C]]
    fn sload(&mut self) {
        
        self.prog_counter += 1;

        let instb = self.read(self.dinst.b);
        let instc = self.read(self.dinst.c);

        let got = self.get(instb, instc);

        self.write(got, self.dinst.a)
    
    }
    /// $m[$r[A]][$r[B]] := $r[C]
    fn store(&mut self) {

        self.prog_counter += 1;

        let insta = self.read(self.dinst.a);
        let instb = self.read(self.dinst.b);
        let instc = self.read(self.dinst.c);


        self.memory.set(insta, instb, instc)
    }
    /// $r[A] := ($r[B] + $r[C]) mod 2 ^ 32
    fn add(&mut self) {
        self.prog_counter += 1;

        let instb = self.read(self.dinst.b);
        let instc = self.read(self.dinst.c);

        self.write(instb.wrapping_add(instc), self.dinst.a)
    }
    /// $r[A] := ($r[B] × $r[C]) mod 2 ^ 32
    fn mult(&mut self) {
        self.prog_counter += 1;
        
        let instb = self.read(self.dinst.b);
        let instc = self.read(self.dinst.c);

        self.write(instb.wrapping_mul(instc), self.dinst.a)
    }
    /// $r[A] := ($r[B] ÷ $r[C]) (integer division)
    fn div(&mut self) {
        self.prog_counter += 1;

        let instb = self.read(self.dinst.b);
        let instc = self.read(self.dinst.c);

        self.write(instb.wrapping_div(instc), self.dinst.a)
    }
    /// $r[A] :=¬($r[B]∧$r[C])
    fn nand(&mut self) {
        self.prog_counter += 1;

        let instb = self.read(self.dinst.b);
        let instc = self.read(self.dinst.c);

        self.write(!(instb & instc), self.dinst.a)
    }
    /// Computation stops
    fn halt(&self) {
        std::process::exit(0)
    }
    /// new segment is created with a number of words equal to the value in $r[C], 
    /// words  initialized to zero
    /// the new segment is mapped as $m[$r[B]].
    /// A bit pattern that is not all zeroes and does not identify any currently mapped segment is placed in $r[B]
    fn map(&mut self) {
        self.prog_counter += 1;

        let instc = self.read(self.dinst.c);

        let allo_id = self.allocate(instc as usize) as u32;
        self.write(allo_id, self.dinst.b);

    }
    ///  The segment $m[$r[C]] is unmapped
    /// Future Map Segment instructions may reuse the identifier $r[C].
    fn unmap(&mut self) {
        self.prog_counter += 1;

        let instc = self.read(self.dinst.c);
        self.deallocate(instc as usize)
    }
    /// The value in $r[C] is displayed on the I/O
    ///  Only values from 0 to 255 are allowed.
    fn output(&self) {
        let instc = self.read(self.dinst.c);
        std::io::Write::write(&mut std::io::stdout(), &[instc as u8]).unwrap();
    }
    ///  UM waits for input on the I/O device
    // $r[c] is loaded with the input
    // must be a value from 0 to 255
    // end of input has been signaled, $r[C] is loaded with a full 32-bit word in which every bit is 1
    fn input(&mut self) {
        self.prog_counter += 1;

        match std::io::stdin().bytes().next().unwrap().unwrap() {
            o  => {
                if o as char == '\n' {
                    self.write(std::u32::MAX, self.dinst.c);
                }
                else {
                    self.write(o.try_into().unwrap(), self.dinst.c);
                }
            }

        } 
    }
    ///  Segment $m[$r[B]] is duplicated
    /// m[0] = duplicate
    /// program counter points to r[c]
    fn pload(&mut self) {
        self.prog_counter += 1;

        let instb = self.read(self.dinst.b) as usize;
        let instc = self.read(self.dinst.c) as usize;
        
        if instb != 0 {
            self.memory.segs[0] = self.memory.segs[instb].clone();
            self.prog_counter = instc;

        } else {
            self.prog_counter = instc;
        }
    }
    /// r[a] = Value
    fn vload(&mut self) {
        self.prog_counter += 1;

        self.write(self.dinst.val, self.dinst.a)
    }

    pub fn disassemble(&mut self) {
        let inst = self.get_i(self.prog_counter);
        self.op(&inst);

        match self.dinst.op {
            0 => {
                self.dinst.geta(&inst);
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.cdmov()
            },
            1 => {
                self.dinst.geta(&inst);
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.sload()
            },
            2 => {
                self.dinst.geta(&inst);
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.store()
            },
            3 => {
                self.dinst.geta(&inst);
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.add()
            },
            4 => {
                self.dinst.geta(&inst);
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.mult()
            },
            5 => {
                self.dinst.geta(&inst);
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.div()
            },
            6 => {
                self.dinst.geta(&inst);
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.nand()
            },
            7 => {
                self.halt()
            },
            8 => {
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.map()
            },
            9 => {
                self.dinst.getc(&inst);
                self.unmap()
            },
            10 => {
                self.prog_counter += 1;
                self.dinst.getc(&inst);
                self.output()
            },
            11 => {
                self.dinst.getc(&inst);
                self.input()
            },
            12 => {
                self.dinst.getb(&inst);
                self.dinst.getc(&inst);
                self.pload()
            },
            13 => {
                self.dinst.geta2(&inst);
                self.dinst.getv(&inst);
                self.vload()
            },
            _ => {
                panic!()
            }
        }
    }
}



