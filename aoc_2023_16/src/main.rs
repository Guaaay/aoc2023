
use std::{collections::{HashSet, HashMap}, thread::current, vec};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
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

fn get_next_tile(i_limit:i32,j_limit:i32,ij: &(i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => {
            if ij.0 == 0 {
                return (-1, -1);
            }
            else {
                return (ij.0-1, ij.1);
            }
        },
        Direction::Down => {
            if ij.0 == i_limit-1 {
                return (-1, -1);
            }
            else {
                return (ij.0+1, ij.1);
            }
        },
        Direction::Left => {
            if ij.1 == 0 {
                return (-1, -1);
            }
            else {
                return (ij.0, ij.1-1);
            }
        },
        Direction::Right => {
            if ij.1 == j_limit-1 {
                return (-1, -1);
            }
            else {
                return (ij.0, ij.1+1);
            }
        },
        Direction::None => {
            return (ij.0, ij.1);
        },
    }
}

fn get_new_dir(current_dir: &Direction, obstacle: char) -> Vec<&Direction>{
    //print!("{} ", obstacle);
   if obstacle == '|'{
    if current_dir == &Direction::Up || current_dir == &Direction::Down {
        //Return current dir
        return vec![current_dir];
    }
    else{
        return vec![&Direction::Up, &Direction::Down];
    }
   }
   if obstacle == '-'{
    if current_dir == &Direction::Left || current_dir == &Direction::Right {
        //Return current dir
        return vec![current_dir];
    }
    else{
        return vec![&Direction::Left, &Direction::Right];
    }
   }
    if obstacle == '/'{
        if current_dir == &Direction::Up {
            return vec![&Direction::Right];
        }
        if current_dir == &Direction::Down {
            return vec![&Direction::Left];
        }
        if current_dir == &Direction::Left {
            return vec![&Direction::Down];
        }
        if current_dir == &Direction::Right {
            return vec![&Direction::Up];
        }
    }
    if obstacle == '\\'{
        if current_dir == &Direction::Up {
            return vec![&Direction::Left];
        }
        if current_dir == &Direction::Down {
            return vec![&Direction::Right];
        }
        if current_dir == &Direction::Left {
            return vec![&Direction::Up];
        }
        if current_dir == &Direction::Right {
            return vec![&Direction::Down];
        }
    }
   return vec![current_dir];
}

fn part2(lines: &Vec<&str>){
    //Energized tiles set
    
    let mut max_energized_tiles = 0;
    let mut i = 0;
    let mut j = 0;
    let mut face = 0;
    let mut end = false;
    let mut rep = false;
    while !end {
        let mut energized_tiles: HashSet<(i32, i32)> = HashSet::new();
        let mut beam_heads: HashMap<(i32, i32),Direction> = HashMap::new();
        

        
        if i == 0 && j == 0 && face == 3 {
            end = true;
        }

        if face == 0 {
            
            if i == lines.len() as i32 - 1 {
                if rep{
                    println!("hola");
                    face = 1;
                    rep = false;
                }
                else{
                    println!("hola2");
                    rep = true;
                
                    beam_heads.insert((i,j), Direction::Right);
                }
            }
            else {
                i += 1;
                beam_heads.insert((i,j), Direction::Right);
            }
            
        }
        if face == 1 {
            if j == lines[0].len() as i32 - 1 {
                if rep{
                    face = 2;
                    rep = false;
                }
                else{
                    rep = true;

                
                    beam_heads.insert((i,j), Direction::Up);
                }
            }
            else {
                j += 1;
                beam_heads.insert((i,j), Direction::Up);
            }
            
        }
        if face == 2 {
            if i == 0 {
                if rep{
                    face = 3;
                    rep = false;
                }
                else{
                    rep = true;
                    
                    beam_heads.insert((i,j), Direction::Left);
                }
            }
            else {
                i -= 1;
                beam_heads.insert((i,j), Direction::Left);
            }
            
        }
        if face == 3 {
            if j == 0 {
                if rep{
                    face = 0;
                    rep = false;
                }
                else{
                    rep = true;
                    
                    beam_heads.insert((i,j), Direction::Down);
                }
            }
            else {
                j -= 1;
                beam_heads.insert((i,j), Direction::Down);
            }
            
        }
        //println!("face: {}", face);
        //println!("ij: {:?}", (i,j));
        
        
        energized_tiles.insert((i,j));
        let mut energized_tiles_count = 0;
        let mut prev_energized_tiles_count = 0;
        let  mut tries = 0;
        loop {
            let mut new_beam_heads: HashMap<(i32, i32),Direction> = HashMap::new();
            for (ij, dir) in beam_heads.iter() {
                //let next_tile = get_next_tile(lines.len() as i32,lines[0].len()as i32 ,ij , dir);
                energized_tiles.insert(*ij);
                
                let this_char = lines[ij.0 as usize].chars().nth(ij.1 as usize).unwrap();
                let new_dirs = get_new_dir(dir, this_char);
                for new_dir in new_dirs {
                    let next_tile = get_next_tile(lines.len() as i32,lines[0].len()as i32 ,ij , new_dir);
                    if !(next_tile == (-1, -1)) {
                        new_beam_heads.insert(next_tile, new_dir.clone());
                    }
                }

            }
            energized_tiles_count = energized_tiles.len();
            if energized_tiles_count == prev_energized_tiles_count {
                tries += 1;
            }
            if tries == 50 {
                break;
            }
            
            prev_energized_tiles_count = energized_tiles_count;

            beam_heads = new_beam_heads;
                
        }
        if energized_tiles_count > max_energized_tiles {
            max_energized_tiles = energized_tiles_count;
        }
        println!("ij: {:?}", (i,j));
        //println!("face: {}", face);
        println!("Energized tiles: {}", energized_tiles.len());
        //Print the tiles
        // for k in 0..lines.len() {
        //     for l in 0..lines[0].len() {
        //         if (k,l) == (i as usize, j as usize) {
        //             print!("X");
        //             continue;
        //         }
        //         if energized_tiles.contains(&(k as i32, l as i32)) {
        //             print!("#");
        //         }
        //         else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }
    }
    
    println!("Max energized tiles: {}", max_energized_tiles);
    
}

fn part1(lines: &Vec<&str>){
    //Energized tiles set
    let mut energized_tiles: HashSet<(i32, i32)> = HashSet::new();
    let mut beam_heads: HashMap<(i32, i32),Direction> = HashMap::new();
    beam_heads.insert((0,3), Direction::Down);
    energized_tiles.insert((0,3));
    let mut energized_tiles_count = 0;
    let mut prev_energized_tiles_count = 0;
    let  mut tries = 0;
    loop {
        let mut new_beam_heads: HashMap<(i32, i32),Direction> = HashMap::new();
        for (ij, dir) in beam_heads.iter() {
            //let next_tile = get_next_tile(lines.len() as i32,lines[0].len()as i32 ,ij , dir);
            energized_tiles.insert(*ij);
            
            let this_char = lines[ij.0 as usize].chars().nth(ij.1 as usize).unwrap();
            let new_dirs = get_new_dir(dir, this_char);
            for new_dir in new_dirs {
                let next_tile = get_next_tile(lines.len() as i32,lines[0].len()as i32 ,ij , new_dir);
                if !(next_tile == (-1, -1)) {
                    new_beam_heads.insert(next_tile, new_dir.clone());
                }
            }

        }
        energized_tiles_count = energized_tiles.len();
        if energized_tiles_count == prev_energized_tiles_count {
            tries += 1;
        }
        if tries == 2 {
            break;
        }
        prev_energized_tiles_count = energized_tiles_count;

        beam_heads = new_beam_heads;
            
    }
    println!("Energized tiles: {}", energized_tiles.len());
    //Print the tiles
    // for i in 0..lines.len() {
    //     for j in 0..lines[0].len() {
    //         if energized_tiles.contains(&(i as i32, j as i32)) {
    //             print!("#");
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }


}


fn main() {
    let lines = include_str!("../example.txt").lines().collect::<Vec<&str>>();
    part1(&lines);
    //part1(&lines);
}