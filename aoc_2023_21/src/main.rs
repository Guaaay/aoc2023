use std::collections::{HashSet,HashMap,VecDeque};
use rand::random;

fn get_neighbors(lines: &Vec<&str>, ij: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4); // Most nodes will have 4 or fewer neighbors

    if ij.0 > 0 && lines[ij.0 - 1].as_bytes()[ij.1] != b'#' {
        neighbors.push((ij.0 - 1, ij.1));
    }
    if ij.0 < lines.len() - 1 && lines[ij.0 + 1].as_bytes()[ij.1] != b'#' {
        neighbors.push((ij.0 + 1, ij.1));
    }
    if ij.1 > 0 && lines[ij.0].as_bytes()[ij.1 - 1] != b'#' {
        neighbors.push((ij.0, ij.1 - 1));
    }
    if ij.1 < lines[ij.0].len() - 1 && lines[ij.0].as_bytes()[ij.1 + 1] != b'#' {
        neighbors.push((ij.0, ij.1 + 1));
    }
    neighbors
}


fn print_visited(visited: &HashSet<(usize, usize)>, lines: &Vec<&str>){
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if visited.contains(&(i, j)) {
                print!("X");
            } else {
                print!("{}", c);
            }
        }
        println!("");
    }
    println!("\n\n\n");
}
fn recursive_search(current: (usize, usize), lines: &Vec<&str>, remaining_steps: u32, visited: &mut HashSet<(usize, usize)>){
    //println!("Visited: {:?}", visited.len());
    //println!("Remaining steps: {:?}", remaining_steps);
    if remaining_steps == 0 {
        visited.insert(current);
        //Roll a dice and only print if 1
        if rand::random::<u32>() % 100000 == 0 {
            print_visited(visited, lines);
        }
        //print_visited(visited, lines);
        return;
    }
    
    let neighbors = get_neighbors(lines, current);
    for neighbor in neighbors {
        
        recursive_search(neighbor, lines, remaining_steps - 1, visited)
    }
}

fn iterative_search(start: (usize, usize), lines: &Vec<&str>, max_steps: usize, visited: &mut HashSet<(usize, usize,usize)>, end_nodes: &mut HashSet<(usize, usize)>) {
    let mut stack = VecDeque::new();
    stack.push_back((start, max_steps));

    while let Some((current, remaining_steps)) = stack.pop_back() {
        if visited.contains(&(current.0, current.1, remaining_steps)) {
            continue;
        }

        if remaining_steps == 0 {
            end_nodes.insert(current);
            //visited.insert((current.0,current.1,remaining_steps));
            if rand::random::<u32>() % 100000 == 0 {
                //print_visited(visited, lines);
            }
            continue;
        }

        visited.insert((current.0,current.1,remaining_steps));
        let neighbors = get_neighbors(lines, current);
        for neighbor in neighbors {
            stack.push_back((neighbor, remaining_steps - 1));
        }
    }
}

fn part1(lines: &Vec<&str>) {
    let mut visited: HashSet<(usize, usize,usize)> = HashSet::new();
    let mut end_nodes: HashSet<(usize, usize)> = HashSet::new();
    println!("Visited: {:?}", visited.len());
    let mut start = (0,0);
    for (i, line) in lines.iter().enumerate() {
        if line.contains('S') {
            start = (i, line.find('S').unwrap() );
            break;
        }
    }
    println!("Start: {:?}", start);
    iterative_search(start, lines, 64, &mut visited, &mut end_nodes);
    println!("Visited: {:?}", end_nodes.len());
}


fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    part1(&lines);
}
