fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();

    let mut current_overlap = 0;
    for line in lines {
        let mut pairs = line.split(',');
        let mut first_pair = pairs.next().unwrap().split('-');
        let first_min: i32 = first_pair.next().unwrap().parse::<i32>().unwrap();
        let first_max: i32 = first_pair.next().unwrap().parse().unwrap();

        let mut second_pair = pairs.next().unwrap().split('-');
        let second_min: i32 = second_pair.next().unwrap().parse().unwrap();
        let second_max: i32 = second_pair.next().unwrap().parse().unwrap();

        if first_min <= second_min {
            // to fully encapsulate the min must be the min
            if first_max >= second_max {
                // the first pair fully encapsulate the second pair
                //  println!("Pair ({0},{1}) fully eats pair ({2},{3})",first_min,first_max,second_min,second_max);
                current_overlap = current_overlap + 1;
                
                // in the case that min==min and max==max, we dont want it counted twice
                continue;
            }
        }

        if second_min <= first_min {
            // to fully encapsulate the min must be the min
            if second_max >= first_max {
                // the second pair fully encapsulate the second pair
                // println!("Pair ({0},{1}) fully eats pair ({2},{3})",second_min,second_max,first_min,first_max);
                current_overlap = current_overlap + 1;
            }
        }
    }

    // 538 is the correct answer

    println!(
        "Total pairs fully containing another pair: {0}",
        current_overlap
    );
    Ok(())
}
