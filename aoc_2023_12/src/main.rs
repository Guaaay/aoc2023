#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum Element {
    Gap,
    Spring,
    Unknown,
}

//Found: [Gap, Gap, Spring, Spring, Gap, Gap, Gap, Gap, Gap, Gap, Spring] 10
fn check_springs(springs: &Vec<Element>, nums: &Vec<u32>) -> bool{
    let mut i = 0;
    let mut consec_springs = 0;
    let mut consec = false;
    for e in springs{
        if i == nums.len() {
            return true;
        }
        if *e == Element::Spring {
            consec_springs += 1;
            consec = true;
            
        } else {
            if consec && nums[i] != consec_springs {
                return false;
            }
            else if nums[i] == consec_springs {
                i += 1;
            }
            consec_springs = 0;
            consec = false;
        }
    }
    if consec  {
        if nums[i] != consec_springs{
                return false;
        }
        else {
            i += 1;
        }
    }
    return i == nums.len();
}

fn recursive_spring_search(springs: Vec<Element>, current_pos: u32, nums: &Vec<u32>, springs_to_place: u32) -> u32 {
    //Debug info:
    //println!("{:?} {} {} {}", springs, current_pos, nums.len(), springs_to_place);
    if current_pos as usize == springs.len() {
        return 0;
    }
    if springs_to_place == 0 && current_pos as usize == springs.len() - 1{
        //If we are in an unknown, automatically place gap
        let mut new_springs = springs.clone();
        if springs[current_pos as usize] == Element::Unknown {
            new_springs[current_pos as usize] = Element::Gap;
        }

        //println!("Checking: {:?}", new_springs);
        if check_springs(&new_springs, nums) {
            //println!("Found: {:?}", new_springs);
            return 1;
        } else {
            //println!("Not found");
            return 0;
        }
    }
    else if current_pos as usize == springs.len() - 1 {
        if springs[current_pos as usize] == Element::Unknown {
            let mut new_springs = springs.clone();
            new_springs[current_pos as usize] = Element::Spring;
            //println!("Checking: {:?}", new_springs);
            if check_springs(&new_springs, nums) {
                //println!("Found: {:?}", new_springs);
                return 1;
            } else {
                return 0;
            }
        }
    }
    
    let mut total: u32 = 0;
    if springs[current_pos as usize] != Element::Unknown{
        return recursive_spring_search(springs, current_pos+1, nums, springs_to_place);
    }
    let mut new_springs = springs.clone();
    if springs_to_place > 0 {
        //First we try to place a spring here
        new_springs[current_pos as usize] = Element::Spring;
        total += recursive_spring_search(new_springs, current_pos+1, nums, springs_to_place-1);
    }
    //Try to place a gap here
    let mut new_springs = springs.clone();
    new_springs[current_pos as usize] = Element::Gap;
    total += recursive_spring_search(new_springs, current_pos+1, nums, springs_to_place);
    return total;
}

fn part2(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        //println!("{}", line);
        let line_split = line.split(" ").collect::<Vec<&str>>();
        let mut nums = line_split[1].split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut modified_line_vec = Vec::<Element>::new();
        let mut gap = false;
        for c in line.chars() {
            if c == '#' {
                modified_line_vec.push(Element::Spring);
                gap = false;
            } else if c == '?' {
                modified_line_vec.push(Element::Unknown);
                gap = false;
            } else if c == '.'{
                if !gap{
                    modified_line_vec.push(Element::Gap);
                    gap = true;
                }
            }
        }
        let mut nums_clone = nums.clone();
        let mut modified_line_vec_clone = modified_line_vec.clone();
        for i in 0..4{

            let mut nums_clone_2: Vec<u32> = nums.clone();
            let mut modified_line_vec_clone_2: Vec<Element> = modified_line_vec.clone();
            modified_line_vec_clone.push(Element::Unknown);
            modified_line_vec_clone.append(&mut modified_line_vec_clone_2);
            
            nums_clone.append(&mut nums_clone_2);
            
        }
        //modified_line_vec_clone.pop();
        //println!("{:?}", modified_line_vec_clone);
        //println!("{:?}", nums_clone);
        //Check all possible spring configurations within constraints
        let total_springs = nums_clone.iter().sum::<u32>();
        let placed_springs = line.chars().filter(|&x| x == '#').count() as u32*5;
        //println!("{} {}", total_springs, placed_springs);
        let springs_to_place = total_springs - placed_springs;
        let res = recursive_spring_search(modified_line_vec_clone, 0, &nums_clone, springs_to_place);
        sum += res;
        println!("{}",res);
    }
    println!("{}", sum);
}



fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        //println!("{}", line);
        let line_split = line.split(" ").collect::<Vec<&str>>();
        let nums = line_split[1].split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let total_springs = nums.iter().sum::<u32>();
        let placed_springs = line.chars().filter(|&x| x == '#').count() as u32;
        let springs_to_place = total_springs - placed_springs;
        let mut modified_line_vec = Vec::<Element>::new();
        let mut gap = false;
        for c in line.chars() {
            if c == '#' {
                modified_line_vec.push(Element::Spring);
                gap = false;
            } else if c == '?' {
                modified_line_vec.push(Element::Unknown);
                gap = false;
            } else if c == '.'{
                if !gap{
                    modified_line_vec.push(Element::Gap);
                    gap = true;
                }
            }
        }
        //Check all possible spring configurations within constraints
        let res = recursive_spring_search(modified_line_vec.clone(), 0, &nums, springs_to_place);
        sum += res;
        println!("{}",res);
    }
    println!("{}", sum);
}

fn main() {
    let lines = include_str!("../example").lines().collect::<Vec<&str>>();
    part2(&lines);
}
