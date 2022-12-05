use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    //     [G] [R]                 [P]
    //     [H] [W]     [T] [P]     [H]
    //     [F] [T] [P] [B] [D]     [N]
    // [L] [T] [M] [Q] [L] [C]     [Z]
    // [C] [C] [N] [V] [S] [H]     [V] [G]
    // [G] [L] [F] [D] [M] [V] [T] [J] [H]
    // [M] [D] [J] [F] [F] [N] [C] [S] [F]
    // [Q] [R] [V] [J] [N] [R] [H] [G] [Z]
    //  1   2   3   4   5   6   7   8   9

    // Start on line 11
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Stack 01
    let mut stack: Vec<char> = Vec::new();
    stack.push('Q');
    stack.push('M');
    stack.push('G');
    stack.push('C');
    stack.push('L');
    stacks.push(stack);

    // Stack 02
    stack = Vec::new();
    stack.push('R');
    stack.push('D');
    stack.push('L');
    stack.push('C');
    stack.push('T');
    stack.push('F');
    stack.push('H');
    stack.push('G');
    stacks.push(stack);

    // Stack 03
    stack = Vec::new();
    stack.push('V');
    stack.push('J');
    stack.push('F');
    stack.push('N');
    stack.push('M');
    stack.push('T');
    stack.push('W');
    stack.push('R');
    stacks.push(stack);

    // Stack 04
    stack = Vec::new();
    stack.push('J');
    stack.push('F');
    stack.push('D');
    stack.push('V');
    stack.push('Q');
    stack.push('P');
    stacks.push(stack);

    // Stack 05
    stack = Vec::new();
    stack.push('N');
    stack.push('F');
    stack.push('M');
    stack.push('S');
    stack.push('L');
    stack.push('B');
    stack.push('T');
    stacks.push(stack);

    // Stack 06
    stack = Vec::new();
    stack.push('R');
    stack.push('N');
    stack.push('V');
    stack.push('H');
    stack.push('C');
    stack.push('D');
    stack.push('P');
    stacks.push(stack);

    // Stack 07
    stack = Vec::new();
    stack.push('H');
    stack.push('C');
    stack.push('T');
    stacks.push(stack);

    // Stack 08
    stack = Vec::new();
    stack.push('G');
    stack.push('S');
    stack.push('J');
    stack.push('V');
    stack.push('Z');
    stack.push('N');
    stack.push('H');
    stack.push('P');
    stacks.push(stack);

    // Stack 09
    stack = Vec::new();
    stack.push('Z');
    stack.push('F');
    stack.push('H');
    stack.push('G');
    stacks.push(stack);

    for line in lines {
        // the first 10-ish lines are the setup for the stacks
        // move 5 from 8 to 2
        if !line.starts_with("move") {
            continue;
        }

        let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let caps = re.captures(line).unwrap();
        
        let qty_str = caps.get(1).map_or("", |m| m.as_str());
        let from_str = caps.get(2).map_or("", |m| m.as_str());
        let to_str = caps.get(3).map_or("", |m| m.as_str());

        let qty:i32 = qty_str.parse().unwrap();

        let from:i32 = (from_str.parse::<i32>().unwrap()) - 1;
        let to:i32 = (to_str.parse::<i32>().unwrap()) - 1;

        if qty > stacks[from as usize].len() as i32 {
            println!("Gonna pop too many boxes!")
        }

        println!("Moving {0} boxes from {1} to {2}", qty,from_str,to_str);
        for i in 1..(qty+1){
            println!("Moving box {0}", i);    
            let item:char = stacks[from as usize].pop().unwrap();
            stacks[to as usize].push(item);
        }
        
        //print_stacks(&stacks);
    }

    println!("Crates on Top:");
    for i in 0..9{
        print!("{0}",stacks[i].pop().unwrap());
    }
    println!("");


// Correct Answer: GCFGLDNJZ
    Ok(())
}

fn print_stacks( cargo_stacks:&Vec<Vec<char>>){
    println!("Stacks State");
    for i in 0..cargo_stacks.len(){
        print!("Stack-{0} - ",i);
        for j in 0..cargo_stacks[i].len(){
            print!("{0}",cargo_stacks[i][j]);
        }   
        println!();
    }
}