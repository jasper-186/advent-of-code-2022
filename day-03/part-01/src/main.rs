use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    
    // for n in 1..26 {
    //     sum += n;
    // }
    let mut sum_of_the_priorities=0;

    for  line in lines {
        let line_len = line.len();
        let pockets = line.split_at(line_len/2);
        let first_pocket:HashSet<char>= HashSet::<_>::from_iter(pockets.0.chars());
        let second_pocket:HashSet<char>= HashSet::<_>::from_iter(pockets.1.chars());

        let overlapping_items = first_pocket.intersection(&second_pocket);
        for item in overlapping_items{

            sum_of_the_priorities += match item {
                'a' => 1,
                'b' => 2,
                'c' => 3,
                'd' => 4,
                'e' => 5,
                'f' => 6,
                'g' => 7,
                'h' => 8,
                'i' => 9,
                'j' => 10,
                'k' => 11,
                'l' => 12,
                'm' => 13,
                'n' => 14,
                'o' => 15,
                'p' => 16,
                'q' => 17,
                'r' => 18,
                's' => 19,
                't' => 20,
                'u' => 21,
                'v' => 22,
                'w' => 23,
                'x' => 24,
                'y' => 25,
                'z' => 26,
                'A' => 27,
                'B' => 28,
                'C' => 29,
                'D' => 30,
                'E' => 31,
                'F' => 32,
                'G' => 33,
                'H' => 34,
                'I' => 35,
                'J' => 36,
                'K' => 37,
                'L' => 38,
                'M' => 39,
                'N' => 40,
                'O' => 41,
                'P' => 42,
                'Q' => 43,
                'R' => 44,
                'S' => 45,
                'T' => 46,
                'U' => 47,
                'V' => 48,
                'W' => 49,
                'X' => 50,
                'Y' => 51,
                'Z' => 52,
                _ => panic!("Unhandled: '{}'", line),
            };
        }
    }

    println!("sum_of_the_priorities: '{}'", sum_of_the_priorities);
    Ok(())
}