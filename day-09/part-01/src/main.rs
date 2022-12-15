use std::collections::{HashSet};


fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    let mut all_tail_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut head_position: (i32, i32) = (0, 0);
    let mut original_head_position: (i32, i32);
    let mut tail_position: (i32, i32) = (0, 0);

    // insert the orginal tail position
    all_tail_positions.insert(tail_position);
    for line in lines {
        // parse line
        let line_chars: Vec<&str> = line.split_ascii_whitespace().collect();
        
        let distance:u32 = line_chars[1].parse().unwrap();


        println!("{0} -> {1} {2}",line, line_chars[0],distance);
        for _ in 0..distance {
            // remember previous head position
            original_head_position = head_position;

            // move head
            head_position = move_head(line_chars[0], head_position);

            // move tail?
            if does_tail_move(&head_position, &tail_position) {
               
                // println!(
                //     "Head ({0},{1}) Tail ({2},{3}) -> ({4},{5})",
                //     head_position.0,
                //     head_position.1,
                //     tail_position.0,
                //     tail_position.1,                    
                //     original_head_position.0,
                //     original_head_position.1,
                // );
                tail_position = original_head_position;
                all_tail_positions.insert(tail_position);
                
            }else{
                // println!(
                //     "Head ({0},{1}) Tail ({2},{3})",
                //     head_position.0,
                //     head_position.1,
                //     tail_position.0,
                //     tail_position.1,
                // );
            }
        }
    }

    // 3301 is too low
    println!(
        "The tail of the rope visit {0} positions at least one time",
        all_tail_positions.len()
    );
    Ok(())
}

fn does_tail_move(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    let x_dist = ( head.0-tail.0).abs();
    let y_dist = ( head.1-tail.1).abs();

    if x_dist>1 {
         return true;
    }

    if y_dist>1 {
         return true;
    }

    return false;
}

fn move_head(direction: &str, current_position: (i32, i32)) -> (i32, i32) {
    // println!(
    //     "{0}",
    //     direction
    // );

    match direction{
        "U"=>{
            return (current_position.0, (current_position.1+1))
        },
        "D"=>{
            return (current_position.0, (current_position.1-1))
        },
        "L"=>{
            return ((current_position.0-1), current_position.1)
        },
        "R"=>{
            return ((current_position.0+1), current_position.1)
        },
        _=>{
            return current_position;
        }
    };
}
