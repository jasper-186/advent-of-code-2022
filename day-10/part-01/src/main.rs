use std::collections::{HashSet};


#[derive(PartialEq, Eq)]
enum Instruction{
    None =-1,
    Noop = 0,
    Addx = 2
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    
    let x_register = 1;
    let y_register = 1;
    let z_register = 1;
    let current_operation = Instruction::None;
    let cycle =1;
    let run_program=true;
while run_program{
    if current_operation == Instruction::None{
        let next_operation = lines.take(1);
        

    }
}

    Ok(())
}

