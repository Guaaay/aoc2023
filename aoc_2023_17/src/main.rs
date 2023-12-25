use std::cmp;
use std::collections::{VecDeque, HashMap};
use priority_queue::PriorityQueue;

#[derive(PartialEq)]
#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn get_dir(ci: u32, cj: u32 , ni: u32, nj: u32) -> Direction {
    if ci == ni {
        if cj < nj {
            return Direction::Right;
        }
        else {
            return Direction::Left;
        }
    }
    else {
        if ci < ni {
            return Direction::Down;
        }
        else {
            return Direction::Up;
        }
    }
}



fn get_neighbors(lines: &Vec<&str>, ij: (u32,u32)) -> Vec<(u32,u32)> {
    //Find current neighbors
    let mut neighbors = Vec::new();
    if ij.0 != 0{
        //Check left
        neighbors.push((ij.0-1, ij.1));
    }
    if ij.0 != lines.len() as u32 -1{
        //Check right
        neighbors.push((ij.0+1, ij.1));
    }
    if ij.1 != 0{
        //Check up
        neighbors.push((ij.0, ij.1-1));
    }
    if ij.1 != lines[0].len() as u32 -1{
        //Check down
        neighbors.push((ij.0, ij.1+1));
    }
    neighbors
}


fn check_consecutives(predecessor: &HashMap<(u32, u32), (u32, u32)>, current: (u32, u32)) -> bool {
    let mut current = current;
    let mut consecutive = 0;
    let mut prev_dir = Direction::None;
    while let Some(&prev) = predecessor.get(&current) {
        let dir = get_dir(prev.0, prev.1, current.0, current.1);
        if dir != Direction::None && prev_dir != Direction::None {
            if dir == prev_dir {
                consecutive += 1;
            }
            else {
                return false;
            }
        }
        if consecutive > 1 {
            return true;
        }
        prev_dir = dir;
        current = prev;
    }
    false
}

fn see_final_path(lines: &Vec<&str>,predecessor: &HashMap<(u32, u32), (u32, u32)>, current: (u32, u32)) {
    let mut current = current;
    let mut path = Vec::new();
    let mut total_weight = 0;
    while let Some(&prev) = predecessor.get(&current) {
        total_weight += lines[current.0 as usize].chars().nth(current.1 as usize).unwrap().to_digit(10).unwrap();
        path.push(current);
        current = prev;
    }
    path.reverse();
    println!("Path: {:?}", path);
    println!("Total Weight: {}", total_weight);
}

fn djikstra_search(lines: &Vec<&str>, start: (u32,u32), end: (u32,u32)) -> u32 {
    let mut distances = vec![vec![u32::max_value(); lines[0].len()]; lines.len()];
    distances[start.0 as usize][start.1 as usize] = 0;
    let mut queue = PriorityQueue::new();
    queue.push(start, 0);
    let mut predecessor: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    while let Some((current, _)) = queue.pop() {
        if current == end {
            //see_final_path(&lines,&predecessor, current);
            //return distances[current.0 as usize][current.1 as usize];
        }
        let neighbors = get_neighbors(lines, current);
        for neighbor in neighbors {
            let n_weight: u32 = lines[neighbor.0 as usize].chars().nth(neighbor.1 as usize).unwrap().to_digit(10).unwrap();
            let mut new_distance = distances[current.0 as usize][current.1 as usize] + n_weight;
            let mut new_pred = predecessor.clone();
            new_pred.insert(neighbor, current);
            if check_consecutives(&new_pred, neighbor) {
                //new_distance += 100;
                continue;
            }
            if new_distance <= distances[neighbor.0 as usize][neighbor.1 as usize] {
                predecessor.insert(neighbor, current);
                distances[neighbor.0 as usize][neighbor.1 as usize] = new_distance;
                queue.push(neighbor, new_distance);
            }
        }
        //println!("Pred: {:?}", predecessor);
    }
    see_final_path(&lines,&predecessor, end);
    return distances[end.0 as usize][end.1 as usize];

    
    
}

fn part1(lines: &Vec<&str>) {
    let ans = djikstra_search(lines, (0,0),(lines.len() as u32 -1 , lines[0].len() as u32-1));
    println!("Part 1: {}", ans);
}


fn main() {
    let lines = include_str!("../example.txt").lines().collect::<Vec<&str>>();
    part1(&lines);
}
