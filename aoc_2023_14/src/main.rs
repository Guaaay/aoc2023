use core::num;
use std::collections::HashSet;

fn part2(lines: &Vec<&str>) {
    let mut free_spots: HashSet<(i32, i32)> = HashSet::new();
    let mut fixed_rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut moving_rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut i = 0;
    let mut j = 0;
    let mut direction = 0;
    let mut num_cycles = 3;
    let mut num_steps = 4;
    for line in lines {
        for c in line.chars() {
            if c == '#' {
                fixed_rocks.insert((i, j));
                
                
            } else if c == 'O' {
                moving_rocks.insert((i, j));
            } else {
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
    //println!("fixed_rocks: {:?}", fixed_rocks);
    //println!("moving_rocks: {:?}", moving_rocks);
    //println!("free_spots: {:?}", free_spots);

    for k in 0..num_cycles {
        if k == num_cycles - 1 {
            num_steps += 1;
        }
        for _ in 0..num_steps {
            free_spots.clear();
            println!("direction: {}", direction);
            let mut total = 0;
            

            
            match direction {
                0 => {
                    for j in 0..lines[0].len() {
                        free_spots.insert((0, j as i32));
                    }
                }
                1 => {
                    for i in 0..lines.len() {
                        free_spots.insert((i as i32, 0));
                    }
                }
                2 => {
                    for j in 0..lines[0].len() {
                        free_spots.insert((lines.len() as i32 - 1, j as i32));
                    }
                }
                3 => {
                    for i in 0..lines.len() {
                        free_spots.insert((i as i32, lines[0].len() as i32 -1));
                    }
                }
                _ => panic!("Invalid direction"),
            };

            for fixed in &fixed_rocks {
                free_spots.remove(fixed);
                match direction {
                    0 => {if fixed.0 < lines.len() as i32 - 1 && !fixed_rocks.contains(&(fixed.0 + 1, fixed.1)){
                        free_spots.insert((fixed.0 + 1, fixed.1))} 
                        else {false}},
                    1 => {if fixed.1 < lines[0].len() as i32 -1 && !fixed_rocks.contains(&(fixed.0, fixed.1 + 1))
                         {free_spots.insert((fixed.0, fixed.1 + 1))} else {false}},
                    2 => {if fixed.0 > 0 && !fixed_rocks.contains(&(fixed.0 - 1, fixed.1))
                        {free_spots.insert((fixed.0  - 1, fixed.1))} else {false}},
                    3 => {if fixed.1 > 0 && !fixed_rocks.contains(&(fixed.0, fixed.1 - 1))
                        {free_spots.insert((fixed.0, fixed.1 - 1))} else {false}},
                    _ => panic!("Invalid direction"),
                };
            }

            let mut start = 0;
            let mut limit = 0;
            match direction {
                0 | 2 => {
                    limit = lines.len() as i32;
                    start = 0;
                }
                1 | 3 => {
                    limit = lines[0].len() as i32;
                    start = 0;
                }

                _ => panic!("Invalid direction"),
            };

            let range = match direction {
                0 | 1 => (start..limit).collect::<Vec<_>>(),
                2 | 3 => (start..limit).rev().collect::<Vec<_>>(),
                _ => panic!("Invalid direction"),
            };

            //println!("range: {:?}", range);

            for i in range {
                for rock in &moving_rocks.clone() {
                    let mut belongs = false;
                    match direction {
                        0 => {
                            if rock.0 == i {
                                belongs = true;
                            }
                        }
                        1 => {
                            if rock.1 == i {
                                belongs = true;
                            }
                        }
                        2 => {
                            if rock.0 == i {
                                belongs = true;
                            }
                        }
                        3 => {
                            if rock.1 == i {
                                belongs = true;
                            }
                        }
                        _ => panic!("Invalid direction"),
                    };
                    if belongs {
                        //println!("Rock: {:?}", rock);
                        let mut new_moving_rocks = moving_rocks.clone();
                        let mut new_free_spots = free_spots.clone();
                        //moving_rocks_in_row.push(*rock);
                        //Find the nearest free spot in this column
                        let mut min_i = 0;
                        let mut min_dist = 100000;
                        for free_spot in &free_spots {
                            //println!("free_spot: {:?}", free_spot);
                            if match direction {
                                0 | 2 => free_spot.1 == rock.1,
                                1 | 3 => free_spot.0 == rock.0,
                                _ => panic!("Invalid direction"),
                            } {
                                let dist = match direction {
                                    0 | 2 => (free_spot.0 - rock.0).abs(),
                                    1 | 3 => (free_spot.1 - rock.1).abs(),
                                    _ => panic!("Invalid direction"),
                                };
                                if dist < min_dist
                                    && match direction {
                                        0 => free_spot.0 <= rock.0,
                                        1 => free_spot.1 <= rock.1,
                                        2 => free_spot.0 >= rock.0,
                                        3 => free_spot.1 >= rock.1,
                                        _ => panic!("Invalid direction"),
                                    }
                                {
                                    min_dist = dist;
                                    min_i = match direction {
                                        0 | 2 => free_spot.0,
                                        1 | 3 => free_spot.1,
                                        _ => panic!("Invalid direction"),
                                    };
                                }
                            }
                        }
                        //println!("min_i: {}", min_i);
                        if min_dist == 100000 {
                            min_i = match direction {
                                0 | 2 => rock.0,
                                1 | 3 => rock.1,
                                _ => panic!("Invalid direction"),
                            };
                        }
                        //println!("min_dist: {}", min_dist);
                        //println!("min_i: {}", min_i);

                        //Rock goes to the free spot with the smallest i in this column
                        
                        //new_moving_rocks.insert((min_i, rock.1));
                       
                        match direction {
                            0 => {if min_i < lines.len() as i32 - 1{
                                new_moving_rocks.remove(rock);
                                new_free_spots.insert((min_i + 1, rock.1));
                                new_moving_rocks.insert((min_i, rock.1));
                                new_free_spots.remove(&(min_i, rock.1));}
                            },
                            1 => {if min_i < lines[0].len() as i32 - 1{
                                new_moving_rocks.remove(rock);
                                new_free_spots.insert((rock.0, min_i + 1));
                                new_moving_rocks.insert((rock.0, min_i));
                                new_free_spots.remove(&(rock.0, min_i));}
                            },
                            2 => {if min_i > 0{
                                new_moving_rocks.remove(rock);
                                new_free_spots.insert((min_i - 1, rock.1));
                                new_moving_rocks.insert((min_i, rock.1));
                                new_free_spots.remove(&(min_i, rock.1));}
                            },
                            3 => {if min_i > 0{
                                new_moving_rocks.remove(rock);
                                new_free_spots.insert((rock.0, min_i - 1));
                                new_moving_rocks.insert((rock.0, min_i));
                                new_free_spots.remove(&(rock.1, min_i));}
                            },
                            _ => panic!("Invalid direction"),
                        };
                        moving_rocks = new_moving_rocks;
                        //println!("new_moving_rocks: {:?}", moving_rocks);
                        //println!("free_spots: {:?}", free_spots);
                        free_spots = new_free_spots;
                        //println!("new_free_spots: {:?}", free_spots);
                        //println!("lines.len(): {}, min_i: {} ", lines[0].len(), min_i);
                        let mut res = 0;
                        if direction == 0{
                            res = lines.len() - min_i as usize;
                        }
                                
                        total += res;
                                
                    }
                }
                
                
            

            }
            if true{
                //println!("moving_rocks: {:?}", moving_rocks.len());
                for line in 0..lines.len() {
                    for col in 0..lines[0].len() {
                        if fixed_rocks.contains(&(line as i32, col as i32)) {
                            print!("#");
                        } else if moving_rocks.contains(&(line as i32, col as i32)) {
                            print!("O");
                        } else if free_spots.contains(&(line as i32, col as i32)) {
                            print!(".");
                        } else {
                            print!(".");
                        }
                    }
                    println!("");
                }
                //println!("direction: {}", direction);
                println!("Total: {}", total);
            }
            //println!("Total: {}", total);
            
            
            
            //
            direction = (direction + 1) % 4;
            
            
        }
    }
}

fn part1(lines: &Vec<&str>) {
    let mut free_spots: HashSet<(i32, i32)> = HashSet::new();
    let mut fixed_rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut moving_rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut i = 0;
    let mut j = 0;
    for line in lines {
        for c in line.chars() {
            if c == '#' {
                free_spots.remove(&(i, j));
                fixed_rocks.insert((i, j));
                if (i < lines.len() as i32 - 1) {
                    free_spots.insert((i + 1, j));
                }
            } else if c == 'O' {
                moving_rocks.insert((i, j));
            } else if i == 0 {
                free_spots.insert((i, j));
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
    //println!("fixed_rocks: {:?}", fixed_rocks);
    //println!("moving_rocks: {:?}", moving_rocks);
    println!("free_spots: {:?}", free_spots);

    let mut total = 0;
    for row in 0..lines.len() {
        //get vec of moving rocks in this row
        //let mut moving_rocks_in_row: Vec<(i32,i32)> = Vec::new();
        for rock in &moving_rocks.clone() {
            if rock.0 == row as i32 {
                //println!("Rock: {:?}", rock);
                let mut new_moving_rocks = moving_rocks.clone();
                let mut new_free_spots = free_spots.clone();
                //moving_rocks_in_row.push(*rock);
                //Find the nearest free spot in this column
                let mut min_i = 0;
                let mut min_dist = 100000;
                for free_spot in &free_spots {
                    if free_spot.1 == rock.1 {
                        let dist = (free_spot.0 - rock.0).abs();
                        if dist < min_dist && free_spot.0 <= rock.0 {
                            min_dist = dist;
                            min_i = free_spot.0;
                        }
                    }
                }
                if min_dist == 100000 {
                    min_i = rock.0;
                }

                //Rock goes to the free spot with the smallest i in this column
                new_moving_rocks.remove(rock);
                new_moving_rocks.insert((min_i, rock.1));
                new_free_spots.remove(&(min_i, rock.1));
                new_free_spots.insert((min_i + 1, rock.1));
                moving_rocks = new_moving_rocks;
                free_spots = new_free_spots;
                //println!("lines.len(): {}, min_i: {} ", lines.len(), min_i);
                let res = lines.len() - min_i as usize;
                total += res;
                //println!("res: {}", res);
            }
        }
    }

    //Print everything in a grid
    for line in 0..lines.len() {
        for col in 0..lines[0].len() {
            if fixed_rocks.contains(&(line as i32, col as i32)) {
                print!("#");
            } else if moving_rocks.contains(&(line as i32, col as i32)) {
                print!("O");
            } else if free_spots.contains(&(line as i32, col as i32)) {
                print!(".");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("Total: {}", total);
    //println!("fixed_rocks: {:?}", fixed_rocks);
    //println!("moving_rocks: {:?}", moving_rocks);
    //println!("free_spots: {:?}", free_spots);
}

fn main() {
    let lines = include_str!("../example.txt")
        .lines()
        .collect::<Vec<&str>>();
    part2(&lines);
}
