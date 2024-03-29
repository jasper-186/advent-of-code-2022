use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    let mut all_tail_positions: HashSet<(i32, i32)> = HashSet::new();

    //let mut head_position: (i32, i32) = (0, 0);
    //let mut original_head_position: (i32, i32);
    //let mut tail_position: (i32, i32) = (0, 0);

    let mut rope: Vec<(i32, i32)> = Vec::new();
    for i in 0..10 {
        rope.insert(i, (0, 0));
    }

    // insert the orginal tail position
    all_tail_positions.insert((0, 0));
    for line in lines {
        // parse line
        let line_chars: Vec<&str> = line.split_ascii_whitespace().collect();

        let distance: u32 = line_chars[1].parse().unwrap();

        println!("{0} -> {1} {2}", line, line_chars[0], distance);
        for _ in 0..distance {
            // remember previous head position
           // original_head_position = rope[0];

            // move head
            rope[0] = move_head(line_chars[0], rope[0]);

            //println!("({0},{1})", rope[0].0,rope[0].1);
            for i in 1..10 {
                // move tail?
                if does_tail_move(&rope[i - 1], &rope[i]) {
                    let temp = move_tail(&rope[i-1], &rope[i]);
                     rope[i]=temp;
                    if i == 9 {
                        println!("({0},{1})", rope[9].0,rope[9].1);
                        all_tail_positions.insert(rope[9]);
                    }
                } else {
                    break;
                }
            }
        }
        //println!("{:?}", rope);
        
    }

    // 2449 is the Correct Answer
    
    println!(
        "The tail of the rope visit {0} positions at least one time",
        all_tail_positions.len()
    );
    Ok(())
}

fn does_tail_move(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    let x_dist = (head.0 - tail.0).abs();
    let y_dist = (head.1 - tail.1).abs();

    if x_dist > 1 {
        return true;
    }

    if y_dist > 1 {
        return true;
    }

    return false;
}

fn move_head(direction: &str, current_position: (i32, i32)) -> (i32, i32) {
   
    match direction {
        "U" => return (current_position.0, (current_position.1 + 1)),
        "D" => return (current_position.0, (current_position.1 - 1)),
        "L" => return ((current_position.0 - 1), current_position.1),
        "R" => return ((current_position.0 + 1), current_position.1),
        _ => {
            return current_position;
        }
    };
}

fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let x_dist = head.0 - tail.0;
    let y_dist = head.1 - tail.1;

    // Diagonally 
    // (Up/Right)
    if x_dist > 0 && y_dist > 0 {
        return (tail.0+1, tail.1+1);
    }
    
    // (Down/Left)
    if x_dist < 0 && y_dist < 0  {
        return (tail.0-1, tail.1-1);
    }

    // (Up/Left)
    if x_dist < 0 && y_dist > 0 {
        return (tail.0-1, tail.1+1);
    }

    // (Down/Right)
    if x_dist > 0 && y_dist < 0  {
        return (tail.0+1, tail.1-1);
    }

    // Orthagonally
    if x_dist > 1 {
        return (tail.0+1,tail.1);
    }

    if y_dist > 1 {
        return (tail.0,tail.1+1);
    }

    if x_dist < -1 {
        return (tail.0-1,tail.1);
    }

    if y_dist < -1 {
        return (tail.0,tail.1-1);
    }

    return (tail.0,tail.1);   
}
