use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    let mut grove: Vec<Vec<i32>> = Vec::new();

    /*

        // Grid Reference
        // 0,0 -> j,0
        //  |
        //  V
        // 0,i
    */

    for (i, item) in lines.enumerate() {
        grove.insert(i, Vec::new());
        for (j, height_str) in item.chars().enumerate() {
           
            grove[i].insert(j, height_str.to_digit(10).unwrap() as i32);
        }
    }

  
    let grove_hoz_size = grove[0].len() -1;
    let grove_ver_size = grove.len() -1 ;
  
    // time to start scanning the grove

    let mut max_scenic_score = 0;

    // West (looking left, right side, i->0)
    for i in 1..grove_ver_size {
        for j in 1..grove_hoz_size {
            let scenic_score = calculate_scenic_score(&(i, j), &grove);

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    // 536625 is the correct answer
    println!(
        "The highest scenic score possible for any tree is {0}",
        max_scenic_score
    );
    Ok(())
}

fn calculate_scenic_score(point: &(usize, usize), map: &Vec<Vec<i32>>) -> i32 {
    let mut north_score: i32 = 0;
    let mut east_score: i32 = 0;
    let mut south_score: i32 = 0;
    let mut west_score: i32 = 0;

    // point(y,x)
    // point(j,i)

    let point_height = map[point.0][point.1];
   
    // Looking North (up)
    let north_boundry =  point.0;
        for j in (0..north_boundry).rev() {
            north_score += 1;
            if map[j][point.1] >= point_height {
                break;
            }
        }
    
        // Looking East (right)
        let east_boundry =  map[point.0].len();
        for i in (point.1+1)..east_boundry {           
            east_score += 1;
            if map[point.0][i] >= point_height {
                break;
            }
        }
    

    // Looking South (down)
    let south_boundry =  map.len();
      
        for j in (point.0+1)..south_boundry {
            south_score += 1;
            if map[j][point.1] >= point_height {
                break;
            }
        }
    
        // Looking West (left)
        let west_boundry =  point.1;      
        for i in (0..west_boundry).rev() {
            west_score += 1;
            if map[point.0][i] >= point_height {
                break;
            }
        }
    

    let total_scenic_score = north_score * east_score * south_score * west_score;
    return total_scenic_score;
}
