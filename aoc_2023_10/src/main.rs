use std::thread::current;

extern crate petgraph;
extern crate num_integer;

fn find_start(lines: &Vec<&str>) -> (usize,usize) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut start: (usize,usize) = (0,0);
    let mut found = false;
    while i < lines.len() && !found{
        j = 0;
        while j < lines[0].len() && !found{
            if lines[i as usize].chars().nth(j as usize).unwrap() == 'S' {
                start = (i,j);
                found = true;
                break;
            }
            j+=1;
        }
        i+=1;
    }
    start
}

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

fn can_go(current: char, candidate: char, dir: &Direction) -> bool {
    let down_check = candidate == 'J' || candidate == 'L' || candidate == '|' || candidate == 'S';
    let up_check = candidate == 'F' || candidate == '7' || candidate == '|' || candidate == 'S';
    let left_check = candidate == 'L' || candidate == 'F' || candidate == '-' || candidate == 'S';
    let right_check = candidate == 'J' || candidate == '7' || candidate == '-' || candidate == 'S';

    if current == 'S' {
        if *dir == Direction::Down {
            return down_check;
        }
        if *dir == Direction::Right {
            return right_check;
        }
        if *dir == Direction::Left {
            return left_check;
        }
        if *dir == Direction::Up {
            return up_check;
        }
    }
    if current == '|' {
        if *dir == Direction::Up {
            return up_check;
        }
        if *dir == Direction::Down {
            return down_check;
        }
    }
    else if current == '7' {
        if *dir == Direction::Left {
            return left_check;
        }
        else if *dir == Direction::Down {
            return down_check;
        }
    }
    else if current == 'J' {
        if *dir == Direction::Left {
            return left_check;
        }
        else if *dir == Direction::Up {
            return up_check;
        }
    }
    else if current == 'L' {
        if *dir == Direction::Right {
            return right_check;
        }
        else if *dir == Direction::Up {
            return up_check;
        }
    }
    else if current == 'F' {
        if *dir == Direction::Right {
            return right_check;
        }
        else if *dir == Direction::Down {
            return down_check;
        }
    }
    else if current == '-' {
        if *dir == Direction::Left {
            return left_check;
        }
        else if *dir == Direction::Right {
            return right_check;
        }
    }
    false
}

fn reverse_dir(dir: &Direction) -> Direction {
    if *dir == Direction::Up {
        return Direction::Down;
    }
    if *dir == Direction::Down {
        return Direction::Up;
    }
    if *dir == Direction::Left {
        return Direction::Right;
    }
    if *dir == Direction::Right {
        return Direction::Left;
    }
    Direction::None
}

fn get_dir(ci: usize, cj: usize , ni: usize, nj: usize) -> Direction {
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


fn find_loop(lines: &Vec<&str>) -> Vec::<(usize,usize)> {
    let mut visited = Vec::<(usize,usize)>::new();
    let mut start: (usize,usize) = (0,0);
    start = find_start(&lines);
    let mut current_pos: (usize,usize) = start;
    let mut done = false;
    //let mut d = 0;
    let mut last_dir: Direction = Direction::None;
    while !done {
        //d+=1; //DEBUGGING
        //Look around the current position
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut curr_char: char = lines[current_pos.0].chars().nth(current_pos.1).unwrap();
        //println!("curr_char: {}", curr_char);
        let mut jump = false;
        while i < 3 && !jump{
            j = 0;
            while j < 3 && !jump{
                if i == 1 && j == 1 || i == 0 && j == 0 || i == 0 && j == 2 || i == 2 && j == 0 || i == 2 && j == 2{
                    j+=1;
                    continue;
                }
                if(i == 0 && current_pos.0 == 0) || (i == 2 && current_pos.0 == lines.len()-1) || (j == 0 && current_pos.1 == 0) || (j == 2 && current_pos.1 == lines[0].len()-1) {
                    j+=1;
                    continue;
                }
                let dir: Direction = get_dir(current_pos.0,current_pos.1,current_pos.0+i-1,current_pos.1+j-1);
                
                let candidate = lines[current_pos.0+i-1].chars().nth(current_pos.1+j-1).unwrap();
                
                //println!("candidate: {:?} dir: {:?}, reverse dir: {:?}",candidate, dir, reverse_dir(&last_dir));
                if dir == reverse_dir(&last_dir) {
                    j+=1;
                    continue;
                }
                if can_go(curr_char,candidate,&dir) {
                    //println!("can go");
                    current_pos = (current_pos.0+i-1,current_pos.1+j-1);
                    curr_char = candidate;
                    last_dir = dir;
                    visited.push(current_pos);
                    jump = true;
                }

                j+=1;
            }
            i+=1;
        }
        if curr_char == 'S' {
            done = true;
        }
    }
    return visited;
    
}

fn check_special_case(char1: char, char2:char) -> bool {
    if char1 == 'F'{
        if char2 == 'J'{
            return false;
        }
        if char2 == '7' {
            return true;
        }
    }
    else if char1 == 'L' {
        if char2 == 'J'{
            return true;
        }
        if char2 == '7'{
            return false;
        }
    }
    false
}

fn part1(lines: Vec<&str>) {
    println!("{:?}",find_loop(&lines).len()/2);

}

fn part2(lines: Vec<&str>){
    let loop_members = find_loop(&lines);
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    //Replace S character with '|'
    
    while i < lines.len() {
        println!("progress: {}%", (i as f32 / lines.len() as f32)*100.0);
        j = 0;
        while j < lines[0].len() {
            //print!("n");
            //If we are on a loop member, skip
            if loop_members.iter().find(|x| x.0 == i && x.1 == j).is_some() {
                //print!("#");
                j+=1;
                continue;
            }


            //Cast ray to end of line --------------------------
            let ray_i = i;
            let mut ray_j = j+1;
            let mut num_collisions = 0;
            let mut last_char = '.';
            let mut on_loop = false;
            let mut evaluating_special_case = false;
            let mut special_case_char = '.';
            //println!("Evaluating point {},{}--------",i+1,j+1);
            while ray_j < lines[0].len(){
                let mut special_case_hit = false;
                let mut current_char = lines[ray_i].chars().nth(ray_j).unwrap();
                let mut next_char = lines[ray_i].chars().nth(ray_j+1).unwrap_or('.');

                //Hack para quitarme el caso de la S xd
                if current_char == 'S' {
                    current_char = '|';
                }
                if next_char == 'S' {
                    next_char = '|';
                }


                if current_char == 'F' || current_char == 'L' {
                    evaluating_special_case = true;
                    special_case_char = current_char;
                }
                if evaluating_special_case && (current_char == '.' || current_char == '|'){
                    evaluating_special_case = false;
                    special_case_char = '.'
                }
                if evaluating_special_case && current_char == '7' || current_char == 'J'{
                    
                    special_case_hit = check_special_case(special_case_char, current_char);
                    //println!("special case: {}",special_case_hit);
                }


                let is_loop_char = loop_members.iter().find(|x| x.0 == ray_i && x.1 == ray_j).is_some();
                let is_first_in_loop = is_loop_char && !loop_members.iter().find(|x| x.0 == ray_i && x.1 == ray_j-1).is_some();
                on_loop = is_loop_char && can_go(current_char, next_char, &Direction::Right) && can_go(current_char, last_char, &Direction::Left);
                if is_loop_char && !on_loop || is_first_in_loop{
                    if evaluating_special_case{
                        if special_case_hit{
                            //println!("SPECIAL: Point: {},{} collided with loop at {},{}",i,j,ray_i,ray_j);
                            num_collisions +=1;
                            special_case_hit = false;
                            evaluating_special_case = false;
                        }
                        else if current_char == 'F' || current_char == 'L'{
                            //println!("NOSPECIAL: Point: {},{} collided with loop at {},{}",i,j,ray_i,ray_j);
                            num_collisions += 1;
                        }
                        //evaluating_special_case = false;
                    }
                    else{
                        //println!("Point: {},{} collided with loop at {},{}",i,j,ray_i,ray_j);
                        num_collisions+=1;
                    }
                    
                    //println!("Point: {},{} collided with loop at {},{}",i,j,ray_i,ray_j);
                }
                last_char = current_char;
                ray_j += 1;
            }
            
            if num_collisions % 2 == 1{
                //print!("*");
                //println!("Point: {},{} inside loop",i,j);
                //println!("num_collisions: {}", num_collisions);
                count+=1;
            }
            else{
                //print!(".");
            }
            // --------------------------


            j+=1;
        }
        //println!();
        i+=1;
    }
    println!("count: {}", count);
}


fn main() {
    let lines = include_str!("../example2.txt").lines().collect::<Vec<_>>();
    part2(lines);
}
