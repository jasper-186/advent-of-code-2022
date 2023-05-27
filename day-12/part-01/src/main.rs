
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use anyhow::Ok;
use itertools::Itertools;


#[derive(PartialEq, Eq, Clone, Hash, Copy)]
enum VisitedState{
    Unvisited=0,
    Visited=1
}

#[derive(PartialEq, Eq, Clone)]
struct Vertex {
    x:i32,
    y:i32,
    elevation_char:char,
    elevation:u32,
    distance:i32,
    neighbors:Vec<Vertex>,
    state:VisitedState    
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}


fn main() -> anyhow::Result<()> {
    // Dijkstra's algorithm seems ineffienct to do, but thats probably what this problem wants in part 2 so doing it that way
    let mut lines = include_str!("../../input").lines();
    let mut read_file=true;
    let mut heightmap:Vec<Vec<u32>> = Vec::new();
    
    let mut start:(i32,i32)=(0,0);
    let mut end:(i32,i32)=(0,0);
    
    let mut file_y=0;
    while read_file{
        // Line should be the "Monkey {}:"
        let line = lines.next();   

        // Check if its empty, then ignore it
        if line.is_none(){
            read_file=false;
            continue;
        } 

        // line vec 
        let mut heightmap_line:Vec<u32> = Vec::new();
        let mut file_x =0;
        for character in line.unwrap().chars(){
            // intentionally convert these to numbers as then i can easily check elevation distance
           let temp = character as u32;

            // "S" in ascii int
            if temp == 83{
                start = (file_x,file_y);
            }

            // "E" in ascii int
            if temp == 69{
                end = (file_x,file_y);
            }

            heightmap_line.push(temp);
            file_x+=1;
        }

        heightmap.push(heightmap_line);
        file_y+=1;
        // Blank line seperating Monkeys
        lines.next();
    }

    let mut verticies:HashMap<(i32,i32),Vertex> = HashMap::new();
    let mut queued_verticies: VecDeque<Vertex> = VecDeque::new();
    
    // add the starting vertix to the queue
    let elevation = heightmap[start.1 as usize][start.0 as usize];
    let elevation_char = char::from_u32(elevation).unwrap();
    let start_vertex = Vertex {
        x:start.0 as i32,
        y:start.1 as i32,
        elevation_char: elevation_char,
        elevation:elevation,
        distance:0,
        neighbors: Vec::new(),
        state:VisitedState::Unvisited    
    };

    queued_verticies.push_back(start_vertex);


    while queued_verticies.len() > 0 {
        // Grab the next node on the list
        let mut vertex = queued_verticies.pop_front().unwrap();

        // is this the final node?
        if vertex.x == end.0 && vertex.y == end.1 {
            // do something
        }

        // grab neighbors
            
        // Top Neighbor
    if vertex.y > 0 {
        // check cache 
        let address = (vertex.x,vertex.y-1);

        let path_length = vertex.distance+1;
        if verticies.contains_key(&address)
        {
            let mut neighbor_node = verticies.get_mut(&address).unwrap();
            // is this an elegible neighbor or is it too steep
            if (neighbor_node.elevation as i32 - vertex.elevation as i32).abs() < 2{
                if path_length < neighbor_node.distance{
                    neighbor_node.distance = path_length;
                    
                    // add this vertex as a neighbor
                    neighbor_node.neighbors.push(vertex);

                    // dedupe the neighbors(just in case)
                    let temp = neighbor_node.neighbors.clone();
                    neighbor_node.neighbors = temp.into_iter().unique().collect();
                }
            }
        }else{
            let n_elevation = heightmap[address.1 as usize][address.0 as usize];
            // is this an elegible neighbor or is it too steep
            if (n_elevation as i32 - elevation as i32).abs() < 2 {
                // Vertex isnt in the cache, so its new, 
                let elevation_char = char::from_u32(elevation).unwrap();
                let mut n_vertex = Vertex {
                    x:address.0,
                    y:address.1,
                    elevation_char: elevation_char,
                    elevation:elevation,
                    distance:path_length,
                    neighbors: Vec::new(),
                    state:VisitedState::Unvisited    
                };

                n_vertex.neighbors.push(vertex);
                queued_verticies.push_back(n_vertex);
            }
        }        
    }

    // Left Neighbor
    if vertex.x > 0 {
        // check cache 
        let address = (vertex.x-1,vertex.y);

        let path_length = vertex.distance + 1;
        if verticies.contains_key(&address)
        {
            let mut neighbor_node = verticies.get_mut(&address).unwrap();
            // is this an elegible neighbor or is it too steep
            if (neighbor_node.elevation as i32 - vertex.elevation as i32).abs() < 2{
                if path_length < neighbor_node.distance{
                    neighbor_node.distance = path_length;
                    
                    // add this vertex as a neighbor
                    neighbor_node.neighbors.push(vertex);

                    // dedupe the neighbors(just in case)
                    let temp = neighbor_node.neighbors.clone();
                    neighbor_node.neighbors = temp.into_iter().unique().collect();
                }
            }
        }else{
            let n_elevation = heightmap[address.1 as usize][address.0 as usize];
            // is this an elegible neighbor or is it too steep
            if (n_elevation as i32 - elevation as i32).abs() < 2 {
                // Vertex isnt in the cache, so its new, 
                let elevation_char = char::from_u32(elevation).unwrap();
                let mut n_vertex = Vertex {
                    x:address.0,
                    y:address.1,
                    elevation_char: elevation_char,
                    elevation:elevation,
                    distance:path_length,
                    neighbors: Vec::new(),
                    state:VisitedState::Unvisited    
                };

                n_vertex.neighbors.push(vertex);
                queued_verticies.push_back(n_vertex);
            }
        }        
    }
    // Right Neighbor
    // Bottom Neighbor

    }

    







    // // K, map is created, time to build the verticies, specifically the neighbors
    // for y in 0..heightmap.len(){
    //     for x in 0..heightmap[y].len(){
    //         let elevation = heightmap[y][x];
    //         let elevation_char = char::from_u32(elevation).unwrap();
    //         let mut vert = Vertex {
    //             x:x as i32,
    //             y:y as i32,
    //             elevation_char: elevation_char,
    //             elevation:elevation,
    //             distance:-1,
    //             neighbors: Vec::new(),
    //             state:VisitedState::Unvisited    
    //         };

    //         let mut neighbors :Vec<Vertex> = Vec::new();

    //        

    //     }
    // }


   
    //println!("Monkey Business: {}", monkey_business);
    Ok(())
}

