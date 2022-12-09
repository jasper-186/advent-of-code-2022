use std::collections::HashMap;

use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();
  let mut current_path="".to_string();

  for line in lines{
    if line.starts_with("$"){
        let re = Regex::new(r"$ ([a-z]+) ([a-z]+)").unwrap();
        let caps = re.captures(line).unwrap();
        
        let cmd = caps.get(1).map_or("", |m| m.as_str());
        let arg = caps.get(2).map_or("", |m| m.as_str());
        
        if cmd=="ls" {
            continue;
        }

        if cmd == "cd" {
            if arg==".."{
                let mut current_path_parts = current_path.split("/").collect::<Vec<&str>>();
                
                // drop the last folder
                current_path_parts.truncate(current_path_parts.len()-1);
                let temp = current_path_parts.iter().map(|i| i.to_string()).collect::<Vec<String>>();
                
                current_path = temp.join("/");
                



            }
        }

    }

  }
  //  println!("Index of first message {0}", start_of_packet_marker_index);
    Ok(())
}
struct Directory{
    name: String,
    parent: String ,
    child_dirs: Vec<Directory>,
    files: HashMap<String,i32>
}
