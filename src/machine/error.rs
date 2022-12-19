use derive_more::{Display, Error};
/// struction to handle the universal machine errors during the execution of a UM function
#[derive(Display, Debug, Error)]
pub enum MachError {
    // #[display(fmt = "beginning of a machine cycle the program counter points outside the bounds of $m[0]")]
    // OutOfBound,
    // #[display(fmt = "word pointed to by the program counter does not code for a valid instruction")]
    // UnvalidInstruction,
    #[display(fmt = " segmented load refers to an unmapped segment or refers to a location outside the bounds of a mapped segment")]
    LoadSegmentFailed,
    #[display(fmt = "segmented store refers to an unmapped segment or refers to a location outside the bounds of a mapped segment")]
    StoreSegmentFailed,
    #[display(fmt = "unmaps a segment that is not mapped")]
    NotFoundUnmapSegment,
    #[display(fmt = " instruction divides by zero")]
    DivisionByZero,
    #[display(fmt = "instruction loads a program from a segment that is not mapped")]
    NotFoundLoadProgramSegment,
    #[display(fmt = "instruction outputs a value larger than 255")]
    UnvalidOutput,
    #[display(fmt = "instruction outputs a value larger than 255")]
    UnvalidInput,
}
