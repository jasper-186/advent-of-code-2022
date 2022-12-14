use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    let mut grove = [[-1i32; 100]; 100];

    /*

        // Grid Reference
        // 0,0 -> j,0
        //  |
        //  V
        // 0,i
    */

    for (i, item) in lines.enumerate() {
        // each column will have at least 2 trees visible (outer perimeter)
        // trees_visible += 2;

        for (j, height_str) in item.chars().enumerate() {
            // if i == 0 {
            //     // each row will have at least 2 visible (outer perimeter)
            //     //       trees_visible += 2;
            // }

            grove[i][j] = height_str.to_digit(10).unwrap() as i32;
        }
    }

    // we over counted by 4 trees, at the verticies of top/left top/right bottom/left bottom/right
    // trees_visible -= 4;

    let grove_hoz_size = grove[0].len();
    let grove_ver_size = grove.len();
    // time to start scanning the grove

    let mut max_scenic_score = 0;
    // West (looking left, right side, i->0)
    for i in 0..grove_ver_size {
        for j in 0..grove_hoz_size {
            let test = i;
            let scenic_score = calculate_scenic_score(&(i,j), &grove);
            
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    // 1175 is too low
    println!(
        "The highest scenic score possible for any tree is {0}",
        max_scenic_score
    );
    Ok(())
}

fn calculate_scenic_score(point: &(usize, usize), map: &[[i32; 100]; 100]) -> i32 {
    let mut north_score: i32 = 0;
    let mut east_score: i32 = 0;
    let mut south_score: i32 = 0;
    let mut west_score: i32 = 0;

    // Looking North (up)
    let point_height = map[point.0][point.1];
    for j in (0..(point.1 )).rev() {
        north_score += 1;
        if map[point.0][j] >= point_height {
            break;
        }
    }

    // Looking East (right)
    for i in point.0..(map[0].len()  ) {
        east_score += 1;
        if map[i][point.1] >= point_height {
            break;
        }
    }

    // Looking South (down)
    for j in point.1..(map.len() ) {
        south_score += 1;
        if map[point.0][j] >= point_height {
            break;
        }
    }

    // Looking West (left)
    for i in (0..(point.0 )).rev() {
        west_score += 1;
        if map[i][point.1] >= point_height {
            break;
        }
    }

    let total_scenic_score = north_score * east_score * south_score * west_score;
    return total_scenic_score;
}
