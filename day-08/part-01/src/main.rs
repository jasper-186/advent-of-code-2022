use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
    let mut grove = [[-1i32; 100]; 100];

    // count the outside trees as we build the grove
    let mut visible_trees:HashSet<(usize,usize)> = HashSet::new();

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
            if i == 0 {
                // each row will have at least 2 visible (outer perimeter)
         //       trees_visible += 2;
            }

            grove[i][j] = height_str.to_digit(10).unwrap() as i32;
        }
    }

    // we over counted by 4 trees, at the verticies of top/left top/right bottom/left bottom/right
   // trees_visible -= 4;

    let grove_hoz_size = grove[0].len();
    let grove_ver_size = grove.len();
    // time to start scanning the grove
    
    // North (looking up, bottom side, 0->j)
    for j in 0..grove_hoz_size {
        let mut max_height = -1;
        for i in (0..(grove_ver_size - 1)).rev() {
            if grove[i][j] > max_height {
                visible_trees.insert((i,j));
                max_height = grove[i][j];
                if max_height == 9 {
                    // 9 is the max height; it will block all other trees
                    break;
                }
            }
        }
    }

    // East ( looking right, left side, 0->i)
    for i in 0..grove_ver_size {
        let mut max_height = -1;
        for j in (0..(grove_hoz_size - 1)).rev() {
            if grove[i][j] > max_height {
                visible_trees.insert((i,j));
                max_height = grove[i][j];
                if max_height == 9 {
                    // 9 is the max height; it will block all other trees
                    break;
                }
            }
        }
    }

    // South (looking down, top side, j->0)
    for j in (0..(grove_hoz_size - 1)).rev(){
        let mut max_height = -1;
        for i in 0..grove_ver_size {
            if grove[i][j] > max_height {
                visible_trees.insert((i,j));
                max_height = grove[i][j];
                if max_height == 9 {
                    // 9 is the max height; it will block all other trees
                    break;
                }
            }
        }
    }

    // West (looking left, right side, i->0)
    for i in (0 ..(grove_ver_size - 1)).rev(){
        let mut max_height = -1;
        for j in 0..grove_hoz_size {
            if grove[i][j] > max_height {
                visible_trees.insert((i,j));
                max_height = grove[i][j];
                if max_height == 9 {
                    // 9 is the max height; it will block all other trees
                    break;
                }
            }
        }
    }
    println!("There are {0} trees visible in the grove", visible_trees.len());
    Ok(())
}
