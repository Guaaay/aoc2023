fn calc_symmetry_vertical(pattern: &Vec<&str>, line: u32, part2: bool) -> u32{
    let mut j = 0;
    let mut num_matching = 0;
    let mut total = 0;
    while j < line{
        let mut i = 0;
        while i < pattern.len(){
            let reflect = (line + (line-j)) -1;
            //println!("i: {}, j: {}, reflect: {}", i,j,reflect);
            if reflect >= pattern[0].len() as u32{
                break;
            }
            total += 1;
            if pattern[i as usize].chars().nth(j as usize) != pattern[i as usize].chars().nth(reflect as usize){
                if !part2{
                    return 0;
                }
                i += 1;
                continue;
            }
            num_matching +=1;
            i += 1;
        }
        j+=1;
    }
    if !part2{
        return line;
    }
    if num_matching == total - 1{
        return line;
    }
    return 0;
}

fn calc_symmetry_horizontal(pattern: &Vec<&str>, line: u32, part2: bool) -> u32{
    let mut i = 0;
    let mut num_matching = 0;
    let mut total = 0;
    while i < line{
        let mut j = 0;
        while j < pattern[0].len(){
            let reflect = (line + (line-i)) -1;
            //println!("i: {}, j: {}, reflect: {}", i,j,reflect);
            if reflect >= pattern.len() as u32{
                break;
            }
            total += 1;
            if pattern[i as usize].chars().nth(j as usize) != pattern[reflect as usize].chars().nth(j as usize){
                if !part2{
                    return 0;
                }
                j += 1;
                continue;
            }
            num_matching +=1;
            j += 1;
        }
        i+=1;

    }
    //println!("num_matching: {}, total: {}", num_matching, total);
    if !part2{
        return line;
    }
    if num_matching == total - 1{
        return line;
    }
    return 0;
}

fn part2(patterns: &Vec<&str>){
    //println!("{:?}", part1_vec);
    let mut sum = 0;
    let mut k = 0;
    for p in patterns{
        //println!("{}", p);
        //println!("Pattern:\n {}", p);
        let pattern_lines = p.lines().collect::<Vec<&str>>();
        let mut i = 1;
        let mut res = 0;
        let mut v_found = false;
        while i < pattern_lines[0].len(){
            
            //println!("probando v: {}", i);
            res = calc_symmetry_vertical(&pattern_lines, i as u32, true);
            //println!("vertical line: {}, res {}", i,res);
            sum += res;
            i += 1;
            if res != 0{
                v_found = true;
                break;
            }
        }
        if v_found{
           continue; 
        }
        let mut j = 1;
        //println!("vres: {}\n", res);
        while j < pattern_lines.len(){
            //println!("probando h: {}", j);
            res = calc_symmetry_horizontal(&pattern_lines, j as u32, true)*100;
            //println!("horizontal line: {}, res: {}", j,res);
            //println!("horizontal line: {}, res {}", j,res);
            sum += res;
            j += 1;
            if res != 0{
                break;
            }
        }
        k+=1;
    }
    println!("Sum: {}", sum);
}



fn part1(patterns: &Vec<&str>){
    let mut sum = 0;
    
    for p in patterns{
        //println!("{}", p);
        //println!("Pattern:\n {}", p);
        let pattern_lines = p.lines().collect::<Vec<&str>>();
        let mut i = 1;
        let mut res = 0;
        let mut v_found = false;
        while i < pattern_lines[0].len(){
            res = calc_symmetry_vertical(&pattern_lines, i as u32, false);
            //println!("vertical line: {}, res {}", i,res);
            sum += res;
            i += 1;
            if res != 0{
                v_found = true;
                break;
            }
        }
        if v_found{
           continue; 
        }
        let mut j = 1;
        //println!("vres: {}\n", res);
        while j < pattern_lines.len(){
            
            res = calc_symmetry_horizontal(&pattern_lines, j as u32, false)*100;
            //println!("horizontal line: {}, res: {}", j,res);
            //println!("horizontal line: {}, res {}", j,res);
            sum += res;
            j += 1;
            if res != 0{
                //println!("horizontal line: {}, res: {}", j,res);
                break;
            }
        }
        //println!("hres: {}\n", res);
    }
    println!("Sum: {}", sum);
}

fn main() {
    let patterns = include_str!("../input.txt").split("\n\n").collect::<Vec<&str>>();
    //part1(&patterns);
    part2(&patterns);
    
    //println!("{:?}",patterns.len());
}