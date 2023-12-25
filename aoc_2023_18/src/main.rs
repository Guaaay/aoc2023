use std::collections::{HashMap as Map};



fn part1_test(lines: &Vec<&str>){
    let mut total = 0;
    let mut rows: Map<i32,Vec<i32>> = Map::new();
    let start: (i32,i32)= (0,0);
    rows.insert(0, vec![0]);
    let mut current = start;
    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let distance = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let mut i = current.0;
        let mut j = current.1;
        let mut count = 0;
        for _ in 0..distance {
            if direction == 'U'{
                //Miro si tengo edges a la derecha
                i-=1;
                if let Some(vec) = rows.get_mut(&i) {
                    vec.push(j);
                } else {
                    let mut vec = Vec::new();
                    vec.push(j);
                    rows.insert(i, vec);
                }
                let max = rows.get(&i).unwrap().iter().max().unwrap();
                if max > &j {
                    total += max - j;
                }
                current = (current.0-1, current.1);
            }
            else if direction == 'D'{
                //Miro si tengo edges a la izquierda
                i += 1;
                if let Some(vec) = rows.get_mut(&i) {
                    vec.push(j);
                } else {
                    let mut vec = Vec::new();
                    vec.push(j);
                    rows.insert(i, vec);
                }
                let min = rows.get(&i).unwrap().iter().min().unwrap();
                if min < &j {
                    total += j - min;
                }
                current = (current.0+1, current.1);
            }
            else if direction == 'L'{
                count += 1;
                if count == distance{
                    let max = rows.get(&i).unwrap().iter().max().unwrap();
                    if max > &j {
                        println!("LastLeft: {}", max - j);
                        total += max - j;
                    }
                }
                j -= 1;
                if let Some(vec) = rows.get_mut(&i) {
                    vec.push(j);
                } else {
                    let mut vec = Vec::new();
                    vec.push(j);
                    rows.insert(i, vec);
                }
                current = (current.0, current.1-1);
            }
            else if direction == 'R'{
                j += 1;
                count += 1;
                if count == distance {
                    let min = rows.get(&i).unwrap().iter().min().unwrap();
                    if min < &j {
                        println!("LastRight: {}", j - min);
                        total += j - min;
                    }
                    
                }
                if let Some(vec) = rows.get_mut(&i) {
                    vec.push(j);
                } else {
                    let mut vec = Vec::new();
                    vec.push(j);
                    rows.insert(i, vec);
                }
                current = (current.0, current.1+1);
            }
            
        }
    }
    println!("{:?}", total);
}

fn part1(lines: &Vec<&str>) {
    let mut total = 0;
    let mut rows: Map<i32,Vec<i32>> = Map::new();
    let start: (i32,i32)= (0,0);
    rows.insert(0, vec![0]);
    let mut current = start;
    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let distance = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let mut i = current.0;
        let mut j = current.1;
        for _ in 0..distance {
            if direction == 'U'{
                i -= 1;
                current = (current.0-1, current.1);
            }
            else if direction == 'D'{
                i += 1;
                current = (current.0+1, current.1);
            }
            else if direction == 'L'{
                j -= 1;
                current = (current.0, current.1-1);
            }
            else if direction == 'R'{
                j += 1;
                current = (current.0, current.1+1);
            }
            if let Some(vec) = rows.get_mut(&i) {
                vec.push(j);
            } else {
                let mut vec = Vec::new();
                vec.push(j);
                rows.insert(i, vec);
            }
        }
    }
    for (i, edge) in &rows {
        println!("i: {}", i);
        let mut gap = false;
        let mut gaps = 0;
        let start = *edge.iter().min().unwrap();
        let end = *edge.iter().max().unwrap();
        total += 1;
        for j in start..end {
            
            //println!("j: {}", j);
            if edge.contains(&j) {
                total += 1;
                gap = false;
            }
            else {
                if gaps % 1 == 0{
                    total += 1;
                }
                if !gap {
                    gap = true;
                    gaps += 1;
                }
                
            }
            //
        }
        println!("total: {}", total);
        
    }
    //println!("{:?}", rows);
    println!("Part 1: {}", total);
    


}



fn main() {
    let lines = include_str!("../example.txt").lines().collect::<Vec<&str>>();
    part1(&lines);
}
