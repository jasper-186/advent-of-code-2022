use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("../../input").lines();
    let line_input = lines.next().unwrap();
    let line_length = line_input.len();
    let mut start_of_packet_marker_index: usize = 14;
    let inter = line_input.chars().collect::<Vec<char>>();
    let mut windows = inter.windows(14);

    while start_of_packet_marker_index < (line_length - 14) {
        let start_of_packet_marker = windows.next().unwrap();
        // println!("{:#?}", start_of_packet_marker);
        if is_marker(start_of_packet_marker) {
            break;
        }

        start_of_packet_marker_index += 1;
    }

    // 3217 is the correct answer
    println!("Index of first message {0}", start_of_packet_marker_index);
    Ok(())
}

fn is_marker(pot_mark: &[char]) -> bool {
    let mut hashset: HashSet<char> = HashSet::new();
    for c in pot_mark {
        if !hashset.insert(c.clone()) {
            return false;
        }
    }

    if hashset.len() < 14 {
        // println!("{:#?}", hashset);
        return false;
    }

    return true;
}
