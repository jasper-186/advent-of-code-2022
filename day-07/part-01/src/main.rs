use std::{collections::HashMap, panic::AssertUnwindSafe};

use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();

    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut files: HashMap<String, i32> = HashMap::new();
    let mut current_path = "".to_string();
    let  current_directory: Directory = Directory {
        full_name: "/".to_string(),
        parent: "".to_string(),
      //  files: HashMap::new(),
    };
    directories.insert("/".to_string(), current_directory);

    for line in lines {
        println!("processing line: '{0}'", line);
        if line.starts_with("$") {
            let re = Regex::new(r"\$ ([a-z]+) ?([a-z/]+)?").unwrap();
            let temp = re.captures(line);
            let caps = temp.unwrap();

            let cmd = caps.get(1).map_or("", |m| m.as_str());
            println!("processing command: '{0}'", cmd);
           
            if cmd == "ls" {
                continue;
            }

            let arg = caps.get(2).map_or("", |m| m.as_str());

            if cmd == "cd" {
                let parent_directory = current_path.clone();
                let mut temp_current_path = current_path.clone();
                // go to the parent directory
                if arg == ".." {
                    let parent_dir_name =
                        directories.get(&parent_directory).unwrap().parent.clone();
                    let parent_dir = directories.get(&parent_dir_name);
                    temp_current_path = parent_dir.unwrap().full_name.clone();
                } else {
                    if !arg.ends_with("/") && !temp_current_path.ends_with("/") {
                        temp_current_path += "/";
                    }

                    temp_current_path += arg;

                    let existing_dir = directories.contains_key(&temp_current_path).clone();
                    if !existing_dir {
                        let n = Directory {
                            //   name: arg.to_string(),
                            full_name: temp_current_path.clone(),
                            parent: directories
                                .get(&parent_directory)
                                .unwrap()
                                .full_name
                                .clone(),
                         //   files: HashMap::new(),
                        };
                        directories.insert(temp_current_path.clone(), n);
                    }
                }

                current_path = temp_current_path.clone();
            }
        } else if line.starts_with("dir") {
            // dir {dirname}
            let re = Regex::new(r"dir ([a-z]+)").unwrap();
            let caps = re.captures(line).unwrap();
            let dir_name = caps.get(1).map_or("", |m| m.as_str());

            println!("processing directory: '{0}'", dir_name);
            let mut temp_current_path = current_path.clone();

            if !dir_name.contains("/") {
                temp_current_path += "/";
            }

            temp_current_path += dir_name;

            let n = Directory {
                // name: dir_name.to_string(),
                full_name: temp_current_path.clone(),
                parent: current_path.clone(),
               // files: HashMap::new(),
            };

            directories.insert(temp_current_path, n);
        } else {
            // {size} {filename}
            let parent_directory = current_path.clone();

            println!("processing file: '{0}'", line);

            let re = Regex::new(r"([0-9]+) ([a-z\.]+)").unwrap();
            let caps = re.captures(line).unwrap();
            let file_size = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let file_name = caps.get(2).map_or("", |m| m.as_str());

            let file_full_name = parent_directory+"/"+file_name;
            files.insert(file_full_name, file_size);
        }
    }

    println!("Lines parsed; calculating size.");
           
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();

    for (k,_) in &directories{
        let mut dir_total:i32=0;
        for key in files.keys().filter(|o| o.contains(k)){
            let file_size = files.get(key).unwrap();
            dir_total += file_size;
        }
        dir_sizes.insert(k.clone(), dir_total);
    }
 
   // print!("{:?}",files);

    let mut small_dir_sizes:i32 = 0;
    for i in dir_sizes.values().filter(|v| v <= &&100000){
        small_dir_sizes += i;
    }

    // 28366 is too low
    println!("total size of dirs under 100000: {0}", small_dir_sizes);
    Ok(())
}

#[derive(Debug)]
struct Directory {
    full_name: String,
    parent: String,
}
