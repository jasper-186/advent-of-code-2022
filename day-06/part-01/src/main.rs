use std::collections::HashSet;

use regex::Regex;

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("../../input").lines();
    let line_input=lines.next().unwrap();

    let marker:Vec<char>=Vec::new();
    let mut index:i32 =0;
    for char_input in line_input.chars() {
        index=index+1;

        
    }

  //  println!("Index of first message {0}", char_set.len());
    Ok(())
}

fn is_marker(pot_mark:&Vec<char>)-> bool {

    if  pot_mark.len() < 4 {
        return false;
    }

    let v: Vec<_> = pot_mark.clone().into_iter().unique().collect();

    return true;
}