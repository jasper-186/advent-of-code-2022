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

        let low_max;
        let high_min;
        let high_max;

        if first_min <= second_min {
            low_max = first_max;
            high_min = second_min;
            high_max = second_max;
        } else {
            low_max = second_max;
            high_min = first_min;
            high_max = first_max;
        }

        // we've established the low min is lower or equal to low min
        if low_max >= high_max {
            current_overlap = current_overlap + 1;
            continue;
        }

        if low_max >= high_min {
            current_overlap = current_overlap + 1;
            continue;
        }
    }

    // 792 is the right answer
    println!("Total overlapping assignments: {0}", current_overlap);
    Ok(())
}
