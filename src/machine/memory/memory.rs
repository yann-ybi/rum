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
    /// return a word from the segmented memory based on its id and offset
    /// return None if no word found at that location
    pub fn get(&mut self, id: u32, offset: u32) -> Option<u32> {

        Some(self.segs[id as usize][offset as usize])

    }
    /// set a word in the segmented memory based on its id and offset
    /// return None if no word found at that location
    pub fn set(&mut self, id: u32, offset: u32, val: u32) -> Option<()> {
        
        Some(self.segs[id as usize][offset as usize] = val)


    }

}