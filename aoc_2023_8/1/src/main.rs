extern crate petgraph;

use petgraph::graph::Graph;
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::{FromElements, Build};
use petgraph::dot::{Dot, Config};

fn main() {
    let mut g = Graph::<String, &str>::new(); // Change node type to String
    let lines = include_str!("../../input.txt").lines().collect::<Vec<_>>();
    let instructions = lines[0].chars().collect::<Vec<_>>();
    println!("instructions: {:?}", instructions);
    for line in lines {
        if line.contains("="){
            let mut split = line.split(" = ").collect::<Vec<_>>();
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
    let start = g.node_indices().find(|i| g[*i] == "AAA").unwrap();
    let end = g.node_indices().find(|i| g[*i] == "ZZZ").unwrap();

    let mut count = 0;
    let mut current = start;
    let mut instruction_index = 0;
    while current != end {
        println!("current: {}", g[current]);
        let next: petgraph::prelude::NodeIndex;
        
        let neighbors = g.neighbors(current).collect::<Vec<_>>();
        if instructions[instruction_index] == 'L'{
            next = neighbors[1];
        }
        else{
            next = neighbors[0];
        }
        current = next;
        count += 1;
        if(instruction_index < instructions.len() - 1){
            instruction_index += 1;
        }
        else{
            instruction_index = 0;
        }
    }
    println!("Part 1: {}", count);
}