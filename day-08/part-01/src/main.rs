use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    let mut grove = [[0u32; 100]; 100];

    // count the outside trees as we build the grove
    let mut trees_visible = 0;
    for (i, item) in lines.enumerate() {
        // each column will have at least 2 trees visible (outer perimeter)
        trees_visible += 2;

        for (j, height_str) in item.chars().enumerate() {
            if i == 0 {
                // each row will have at least 2 visible (outer perimeter)
                trees_visible += 2;
            }

            grove[i][j] = height_str.to_digit(10).unwrap();
        }
    }

    // we over counted by 4 trees, at the verticies of top/left top/right bottom/left bottom/right
    trees_visible -= 4;

    let grove_hoz_size = grove[0].len();
    let grove_ver_size = grove.len();
    // time to start scanning the grove
    // North (looking up)

    for i in 0..grove_hoz_size {
        let mut max_height = 0;
        for j in (grove_ver_size - 1)..0 {
            if grove[i][j] > max_height {
                trees_visible += 1;
                max_height = grove[i][j];
                if max_height == 9 {
                    // 9 is the max height; it will block all other trees
                    break;
                }
            }
        }
    }

    // East
    // South
    // West

    println!("There are {0} trees visible in the grove", trees_visible);
    Ok(())
}
