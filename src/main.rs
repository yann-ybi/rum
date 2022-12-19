mod machine;
use crate::machine::machine::UM;
use crate::machine::dinst::Dinst;
use rum::rumload;

fn main() {

    let input = std::env::args().nth(1);

    let mut machine = UM::new(rumload::load(input.as_deref()));
    
    // let mut inst_numb = 0;
    // let now = Instant::now();
    loop {
        // inst_numb += 1;
        Dinst::disassemble(&mut machine);
    }

}

