
#[derive(PartialEq, Eq)]
enum Instruction{
    None =-1,
    Noop = 0,
    Addx = 2
}

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("../../input").lines();
    
    // Cycle 1
    let mut v_register = 0;
    // Cycle 2
    let mut w_register = 0;
    // Register
    let mut x_register = 1;
    let mut current_operation = Instruction::None;
    let mut cycle = 1;
    let mut run_program=true;

    while run_program{
        // get the value of x at 20 units, and every 40  units there after
        if cycle%40 == 1{
            print!("\n")
        }

        // AT the Start of the Cycle, are we currently executing a previous execution?
        //  If not, read the next instruction
        if current_operation == Instruction::None{
            let next_operation = lines.next();      
            if next_operation.is_none(){
                run_program=false;
                continue;
            }  
            let mut parts = next_operation.unwrap().split_ascii_whitespace();
            let operation = parts.next().unwrap();
            if operation.to_uppercase() == "ADDX"{
                let amount_str = parts.next();
                let amount: i32 = amount_str.unwrap().parse().unwrap();
                v_register = amount;
                current_operation = Instruction::Addx;
            } else if operation.to_uppercase() == "NOOP"{
                current_operation = Instruction::Noop;
            }
    
        }

        let pixel_column = ((cycle)%40)-1;
        let sprite_low = x_register-1;
        let sprite_high = x_register+1;
        if sprite_low<=pixel_column && pixel_column<=sprite_high {
            print!("x");
        }else{
            print!(".")
        }

        // Move v->w, and w->x
        if current_operation == Instruction::Addx {
            // this should happen after the 2nd cycle of a instruction
            if w_register != 0 {
                x_register += w_register;
                w_register = 0;
                // adding this to the x_reg completes the operation, so clear the thing
                current_operation=Instruction::None;
            }

            // this should happen in the same cycle as the initial instruction add
            if v_register != 0 {
                w_register = v_register;
                v_register = 0;
            }
    
        }

        cycle+=1;
        // doing this because i expect part 2 to use NOOP somehow
        if current_operation == Instruction::Noop {
            current_operation = Instruction::None;
        }


    }
    Ok(())
}

