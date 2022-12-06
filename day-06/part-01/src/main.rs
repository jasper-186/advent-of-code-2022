fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("../../input").lines();
    let line_input = lines.next().unwrap();
    let line_length = line_input.len();
    let mut start_of_packet_marker_index: usize = 4;
    let inter = line_input.chars().collect::<Vec<char>>();
    let mut windows = inter.windows(4);

    while start_of_packet_marker_index < (line_length - 4) {
        let start_of_packet_marker = windows.next().unwrap();
        if is_marker(start_of_packet_marker) {
            break;
        }

        start_of_packet_marker_index += 1;
    }

    // 1175 is the correct answer
    println!("Index of first message {0}", start_of_packet_marker_index);
    Ok(())
}

fn is_marker(pot_mark: &[char]) -> bool {
    if pot_mark[0] == pot_mark[1] {
        return false;
    }
    if pot_mark[0] == pot_mark[2] {
        return false;
    }
    if pot_mark[0] == pot_mark[3] {
        return false;
    }

    if pot_mark[1] == pot_mark[2] {
        return false;
    }
    if pot_mark[1] == pot_mark[3] {
        return false;
    }

    if pot_mark[2] == pot_mark[3] {
        return false;
    }

    return true;
}
