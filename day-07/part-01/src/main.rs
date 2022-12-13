use std::{collections::HashMap};

use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("../../input").lines();

    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut files: HashMap<String, i64> = HashMap::new();
    let mut current_path = "".to_string();
    // let  current_directory: Directory = 
    directories.insert("/".to_string(), Directory {
            full_name: "/".to_string(),
            parent: "".to_string(),
          //  files: HashMap::new(),
        });

    for line in lines {
        //println!("processing line: '{0}'", line);
        if line.starts_with("$") {
            let re = Regex::new(r"\$ ([a-z]+) ?([a-z/\.]+)?").unwrap();
            let temp = re.captures(line);
            let caps = temp.unwrap();

            let cmd = caps.get(1).map_or("", |m| m.as_str());
            // println!("processing command: '{0}'", cmd);
           
            if cmd == "ls" {                
             //   println!("current_path     {0}",current_path);                
                continue;
            }

            let arg = caps.get(2).map_or("", |m| m.as_str());

            if cmd == "cd" {
                let parent_directory = current_path.clone();
                let mut temp_current_path = current_path.clone();
                // println!("Changing directory to {0}",arg);
                // println!("current_path     {0}",current_path);
                
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

                //println!("new current_path {0}",current_path);
            }
        } else if line.starts_with("dir") {
            // dir {dirname}
            let re = Regex::new(r"dir ([a-z\.]+)").unwrap();
            let caps = re.captures(line).unwrap();
            let dir_name = caps.get(1).map_or("", |m| m.as_str());

            //println!("processing directory: '{0}'", dir_name);
            let mut temp_current_path = current_path.clone();

            if !dir_name.contains("/") && !temp_current_path.ends_with("/") {
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

            
            let re = Regex::new(r"([0-9]+) ([a-z\.]+)").unwrap();
            let caps = re.captures(line).unwrap();
            let file_size = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap());
            let file_name = caps.get(2).map_or("", |m| m.as_str());
            

            let mut spacer = "";
            if !file_name.contains("/") && !parent_directory.ends_with("/") {
                spacer = "/";
            }


            let file_full_name = parent_directory+spacer+file_name;
            //println!("current_path     {0}",current_path);
           // println!("fileFullName     {0}",file_full_name);
            // println!("processing file: '{0}'", line);
            // println!("current path: '{0}'", current_path);
            // println!("file name: '{0}'", file_name);
            // println!("file full name: '{0}'", file_full_name);

            files.insert(file_full_name, file_size);
        }
    }

    println!("Lines parsed; calculating size.");
           
    let mut dir_sizes: HashMap<String, i64> = HashMap::new();
    let directories_sorted = directories.keys().sorted_by(|a,b| a.len().cmp(&b.len())).rev();

    for k in directories_sorted{
        println!("finding size for dir {0}", k);
        // get all files in the directory
        let mut dir_total:i64=0;
        let contains_string = k.to_string()+"/";
        let files_subfiles = files.keys().filter(|o| o.contains(&contains_string)).collect::<Vec<&String>>();
        if files_subfiles.len() ==0{
            println!("Directory {0} has no files", k);
        }else{
            println!("Directory {0} has {1} files", k,files_subfiles.len());
        }

        for key in files_subfiles {
            let file_size = files.get(key).unwrap();
            dir_total = dir_total+( file_size.clone() as i64);
        }

        // get all directories in the directory (count twice)
        // let subdir = dir_sizes.keys().filter(|o| o.contains(k)).collect::<Vec<&String>>();
        // if subdir.len() ==0{
        //     println!("Directory {0} has no directories", k);
        // }else{
        //     println!("Directory {0} has {1} directories", k,subdir.len());
        // }
        
        // for key in subdir {
        //     let dir_size = dir_sizes.get(key).unwrap();
        //     dir_total += dir_size;
        // }

        dir_sizes.insert(k.clone(), dir_total);
    }
 
   // print!("{:?}",dir_sizes);
    // for k in dir_sizes.keys().sorted(){
    //     println!("{0} - {1}" ,k ,dir_sizes.get(k).unwrap());
    // }
    println!("Files Layout:");
    for key in files.keys().sorted(){
        println!("{0} - {1}", format!("{:0>10}",  files.get(key).unwrap()),key);
    }

    // println!("directory Layout:");
    // for key in dir_sizes.keys().sorted(){
    //     println!("{0} - {1}",   format!("{:0>10}",  dir_sizes.get( key).unwrap())   ,key);
    // }

    let mut small_dir_sizes:i64 = 0;
    for i in dir_sizes.values().filter(|v| v < &&100000){
        small_dir_sizes += i ;
    }

    println!("total file count {0}", files.len());

    println!("total dir count {0}", dir_sizes.len());
    
    // 1685109 is not the right answer
    // {1743217}
    println!("total size of dirs under 100000: {0}", small_dir_sizes);
    Ok(())
}

#[derive(Debug)]
struct Directory {
    full_name: String,
    parent: String,
}
