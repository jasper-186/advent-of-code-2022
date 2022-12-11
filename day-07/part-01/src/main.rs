use std::{collections::HashMap, panic::AssertUnwindSafe};

use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();

    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut current_path = "".to_string();
    let  current_directory: Directory = Directory {
        full_name: "/".to_string(),
        parent: "".to_string(),
        files: HashMap::new(),
    };
    directories.insert("/".to_string(), current_directory);

    for line in lines {
        println!("processing line: '{0}'", line);
        println!("directory count: '{0}'", directories.len());
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
                    if !arg.contains("/") {
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
                            files: HashMap::new(),
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
                files: HashMap::new(),
            };

            directories.insert(temp_current_path, n);
        } else {
            // {size} {filename}

            let parent_directory = current_path.clone();

            let re = Regex::new(r"([0-9]+) ([a-z\.]+)").unwrap();
            let caps = re.captures(line).unwrap();
            let file_size = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let file_name = caps.get(2).map_or("", |m| m.as_str());

            println!("processing file: '{0}'", file_name);
            directories
                .get_mut(&parent_directory)
                .unwrap()
                .files
                .insert(file_name.to_string(), file_size);
        }
    }

    let mut total =0;
    for (_,d) in &directories{
        let t = d.get_size(&directories);
        if t > 100000{
            continue;
        }
        total = total+t;
    }
 

    println!("total size of dirs under 100000: {0}", total);
    Ok(())
}
struct Directory {
    full_name: String,
    parent: String,
    files: HashMap<String, i32>,
}

impl Directory {
    pub fn get_size(&self, directories_list: &HashMap<String, Directory>) -> i32 {
        let keys = directories_list
            .keys()
            .filter(|k| k.contains(&self.full_name));

        let mut total_size = 0;
        for i in keys {
            let d = directories_list.get(i).unwrap();
            total_size = total_size + d.get_size(directories_list);
        }

        for (_, i) in &self.files {
            total_size = total_size + i;
        }

        return total_size;
    }
}
