use std::{io::{Result, BufReader, BufRead}, fs::File};

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    left: usize,
    right: usize,
}

impl Node {
    fn print(&self) -> () {
        let char_1 = (self.id as f64 / 10000.0).floor() as u8 as char;
        let char_2 = ((self.id % 10000) as f64 / 100.0).floor() as u8 as char;
        let char_3 = (self.id % 100) as u8 as char;
        println!("Node with id: {char_1}{char_2}{char_3}");
    }
}

// Note to self, A = 65 so AAA = 656565
fn string_slice_to_id(slice: &str) -> usize {
    let mut id: usize = 0;
    for (i, char_in_bytes) in slice.as_bytes().iter().enumerate() {
        id += (*char_in_bytes as usize) * 100_usize.pow(i as u32);
    }
    id
}

fn is_node_begin_node(node: &Node) -> bool {
    (node.id % 100) == 65
}

fn is_vec_all_end_nodes(nodes: &Vec<Node>) -> bool {
    for node in nodes {
        if (node.id % 100) != 90 {
            return false;
        }
    }
    true
}

pub fn run() -> Result<String> {
    let filepath = "./src/day8/input.txt";
    let file = File::open(filepath);
    let reader = BufReader::new(file.unwrap());

    let lines_vec: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    let path: Vec<char> = lines_vec.get(0).unwrap().chars().collect();
    let mut nodes: Vec<Node> = Vec::new();
    let mut begin_nodes: Vec<Node> = Vec::new();

    println!("Path: {path:?}");
    for line in lines_vec.iter().skip(2) {
        let id = string_slice_to_id(&line[0..3]);
        let left = string_slice_to_id(&line[7..10]);
        let right = string_slice_to_id(&line[12..15]);

        let new_node = Node {
            id: id,
            left: left,
            right: right
        };
        nodes.push(new_node.clone());

        if is_node_begin_node(&new_node) {
            begin_nodes.push(new_node.clone());
        }
    }
    nodes.sort_by_key(|x| x.id);

    let mut steps_taken: usize = 0;
    let mut are_all_nodes_end = false;

    while !are_all_nodes_end {
        steps_taken += 1;

        for current_node in begin_nodes.iter_mut() {
            for step in path.iter(){
                let left_step = &current_node.left;
                let right_step = &current_node.right;

                let index_next_node = match step {
                    'L' => nodes.binary_search_by_key(left_step, |node| node.id).unwrap(),
                    'R' => nodes.binary_search_by_key(right_step, |node| node.id).unwrap(),
                    _ => panic!("Oi were you going mate"),
                };

                current_node = nodes.get_mut(index_next_node).unwrap();
            }   
        }

        are_all_nodes_end = is_vec_all_end_nodes(&begin_nodes);
    }
    

    Ok(steps_taken.to_string())
}