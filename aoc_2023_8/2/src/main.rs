extern crate petgraph;
extern crate num_integer;

use petgraph::graph::Graph;
use petgraph::data::{FromElements};
use petgraph::dot::{Dot, Config};

fn lcm(nums: Vec<usize>) -> usize{
    let mut lcm = 1;
    for num in nums{
        lcm = num_integer::lcm(lcm, num);
    }
    return lcm;
}

fn main() {
    let mut g = Graph::<String, &str>::new(); // Change node type to String
    let lines = include_str!("../../input.txt").lines().collect::<Vec<_>>();
    let instructions = lines[0].chars().collect::<Vec<_>>();
    println!("instructions: {:?}", instructions);
    for line in lines {
        if line.contains("="){
            let split = line.split(" = ").collect::<Vec<_>>();
            let from = split[0].to_string(); // Convert to String
            let dests = split[1].split(", ").collect::<Vec<_>>();
            let left = dests[0].replace("(","");
            let right = dests[1].replace(")","");
            println!("{} -> {} , {}", from, left, right);


            //Only add from node if it doesn't exist
            let fromi: petgraph::prelude::NodeIndex;
            if g.node_indices().find(|i| g[*i] == from).is_none(){
                fromi = g.add_node(from.clone());
            }
            else{
                fromi = g.node_indices().find(|i| g[*i] == from).unwrap();
            }
            println!("fromi: {:?}", fromi);
            //let fromi = g.add_node(from); // Now from is a String
            //Only add left node if it doesn't exist
            let lefti: petgraph::prelude::NodeIndex;
            if g.node_indices().find(|i| g[*i] == left).is_none(){
                lefti = g.add_node(left.clone());
            }
            else{
                lefti = g.node_indices().find(|i| g[*i] == left).unwrap();
            }
            //Only add right node if it doesn't exist
            let righti: petgraph::prelude::NodeIndex;
            if g.node_indices().find(|i| g[*i] == right).is_none(){
                righti = g.add_node(right.clone());
            }
            else{
                righti = g.node_indices().find(|i| g[*i] == right).unwrap();
            }
            g.add_edge(fromi, lefti, "left");
            g.add_edge(fromi, righti, "right");
        }
    }
    //Print the graph
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    //Traverse the graph starting in "AAA" until "ZZZ" is found
    let mut start_nodes: Vec<petgraph::prelude::NodeIndex> = Vec::new();
    let mut end_nodes: Vec<petgraph::prelude::NodeIndex> = Vec::new();
    let mut current_nodes: Vec<petgraph::prelude::NodeIndex> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();
    for node in g.node_indices(){
        if g[node].chars().last().unwrap() == 'A'{
            start_nodes.push(node);
            current_nodes.push(node);
            counts.push(0);
        }
        if g[node].chars().last().unwrap() == 'Z'{
            end_nodes.push(node);
        }

    }
    //Print start and end nodes' string data
    for node in start_nodes.clone(){
        println!("start: {}", g[node]);
    }
    for node in end_nodes.clone(){
        println!("end: {}", g[node]);
    }
    let mut done = false;
    let mut instruction_index = 0;
    while !done{
        let mut i = 0;
        while i < current_nodes.len(){
            if end_nodes.contains(&current_nodes[i]){
                i += 1;
                continue;
            }
            //println!("current: {}", g[current_nodes[i]]);
            let next: petgraph::prelude::NodeIndex;
            let neighbors = g.neighbors(current_nodes[i]).collect::<Vec<_>>();
            if instructions[instruction_index] == 'L'{
                next = neighbors[1];
            }
            else{
                next = neighbors[0];
            }
            current_nodes[i] = next;
            counts[i] += 1;
            i += 1;
        }
        if instruction_index < instructions.len() - 1 {
            instruction_index += 1;
        }
        else{
            instruction_index = 0;
        }
        //Check if all current nodes are end nodes
        let mut all_end_nodes = true;
        for current in current_nodes.clone(){
            if !end_nodes.contains(&current){
                all_end_nodes = false;
            }
        }
        if all_end_nodes{
            done = true;
        }
        //Print current nodes' string data
        // for node in current_nodes.clone(){
        //     println!("current: {}", g[node]);
        // }
    }
    println!("sol: {:?}", lcm(counts));
}