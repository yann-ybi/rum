use std::{collections::VecDeque, usize, vec};

/// Memory structure of the UM machine
pub struct Memory {
    // segmented memory
    pub segs: Vec<Vec<u32>>,
    // unmapped segments ready to be mapped again
    pub unmapped_segs: VecDeque<usize>
}
impl Memory {
    /// Memory constructor function
    pub fn new(instructions: Vec<u32>) -> Self {

        Memory {
            segs: vec![instructions],
            unmapped_segs: VecDeque::new()
        }
    }
    pub fn get_i(&mut self, offset: usize) -> u32 {
        self.segs[0][offset]
    }
    
    /// return a word from the segmented memory based on its id and offset
    /// return None if no word found at that location
    pub fn get(&mut self, id: u32, offset: u32) -> u32 {
        self.segs[id as usize][offset as usize]

    }
    /// set a word in the segmented memory based on its id and offset
    /// return None if no word found at that location
    pub fn set(&mut self, id: u32, offset: u32, val: u32) {
        
        self.segs[id as usize][offset as usize] = val

    }
    pub fn allocate(&mut self, len: usize) -> usize {
        let new_seg: Vec<u32> = vec![0_u32; len];
        
        match self.unmapped_segs.pop_front() {
            Some(o) => {
                self.segs[o] = new_seg;
                o
            },
            None => {
                self.segs.push(new_seg);
                self.segs.len() - 1
            }
        }
    }
    pub fn deallocate(&mut self, id: usize) {
        self.segs[id] = Vec::new();
        self.unmapped_segs.push_back(id)
    }


}