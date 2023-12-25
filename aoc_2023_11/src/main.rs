fn part2(lines: Vec<&str>){
    //Convert lines into a Vec<String>
    let new_lines = lines.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let lines_len = new_lines.len();
    let line_len = new_lines[0].len();
    let mut i = 0;
    let mut j = 0;
    let mut galaxies = Vec::<(i32,i32,i32)>::new();
    let mut horizontals = Vec::<i32>::new();
    let mut verticals = Vec::<i32>::new();
    while i < lines_len {
        j = 0;
        let mut galaxy_found = false;
        while j < line_len {
            
            if new_lines[i].chars().nth(j).unwrap() != '.' {
                galaxy_found = true;
            }
            j+=1;
        }
        if !galaxy_found {
            horizontals.push(i as i32);
        }
        i += 1;
    }
    i = 0;
    j = 0;
    while j < line_len {
        let mut galaxy_found = false;
        i = 0;
        while i < lines_len {
            if new_lines[i].chars().nth(j).unwrap() != '.' {
                galaxy_found = true;
            }
            i+=1;
        }
        if !galaxy_found {
            verticals.push(j as i32);
        }
        j+=1;
    }
    i = 0;
    j = 0;
    let mut count: i32 = 0;
    while i < lines_len {
        j = 0;
        while j < line_len {
            if new_lines[i].chars().nth(j).unwrap() != '.' {
                galaxies.push((i as i32,j as i32,count));
                count += 1;
            }
            j+=1;
        }
        i+=1;
    }
    let mut pairs = Vec::<(i32,i32)>::new();
    i = 0;
    j = 0;
    let mut sum: u64 = 0;
    while i < galaxies.len(){
        j = 0;
        while j < galaxies.len(){
            //println!("Comparing {} and {}", galaxies[i].2, galaxies[j].2);
            let g_1 = galaxies[i].2;
            let g_2 = galaxies[j].2;
            if i != j && pairs.iter().find(|x| (x.0 == g_1 && x.1 == g_2) || (x.0 == g_2 && x.1 == g_1)).is_none(){
                let mut count = 0;
                for h in horizontals.iter(){
                    if (galaxies[i].0 < *h && galaxies[j].0 > *h ) || (galaxies[i].0 > *h && galaxies[j].0 < *h ){
                        count += 1;
                    } 
                }
                for v in verticals.iter(){
                    if (galaxies[i].1 < *v && galaxies[j].1 > *v ) || (galaxies[i].1 > *v && galaxies[j].1 < *v ){
                        count += 1;
                    } 
                }
                let shortest_path:u64 = (((galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs()) as u64 + count*999999 as u64).into();
                sum += shortest_path;
                pairs.push((g_1,g_2));
            }
            j+=1;
        }
        //println!("i: {}", i);
        i+=1;
    }
    println!("count: {}", count);
    println!("sum: {}", sum);
}


fn part1(mut lines: Vec<&str>){
    //Convert lines into a Vec<String>
    let mut new_lines = lines.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let mut lines_len = new_lines.len();
    let mut line_len = new_lines[0].len();
    let mut i = 0;
    let mut j = 0;
    let mut galaxies = Vec::<(i32,i32,i32)>::new();
    let mut horizontals = Vec::<i32>::new();
    let mut verticals = Vec::<i32>::new();
    while i < lines_len {
        j = 0;
        let mut galaxy_found = false;
        while j < line_len {
            
            if new_lines[i].chars().nth(j).unwrap() != '.' {
                galaxy_found = true;
            }
            j+=1;
        }
        if !galaxy_found {
            horizontals.push(i as i32);
        }
        i += 1;
    }
    i = 0;
    j = 0;
    while j < line_len {
        let mut galaxy_found = false;
        i = 0;
        while i < lines_len {
            if new_lines[i].chars().nth(j).unwrap() != '.' {
                galaxy_found = true;
            }
            i+=1;
        }
        if !galaxy_found {
            verticals.push(j as i32);
        }
        j+=1;
    }
    i = 0;
    j = 0;
    let mut count: i32 = 0;
    while i < lines_len {
        j = 0;
        while j < line_len {
            if new_lines[i].chars().nth(j).unwrap() != '.' {
                galaxies.push((i as i32,j as i32,count));
                count += 1;
            }
            j+=1;
        }
        i+=1;
    }
    let mut pairs = Vec::<(i32,i32)>::new();
    i = 0;
    j = 0;
    let mut sum = 0;
    while i < galaxies.len(){
        j = 0;
        while j < galaxies.len(){
            //println!("Comparing {} and {}", galaxies[i].2, galaxies[j].2);
            let g_1 = galaxies[i].2;
            let g_2 = galaxies[j].2;
            if i != j && pairs.iter().find(|x| (x.0 == g_1 && x.1 == g_2) || (x.0 == g_2 && x.1 == g_1)).is_none(){
                let mut count = 0;
                for h in horizontals.iter(){
                    if (galaxies[i].0 < *h && galaxies[j].0 > *h ) || (galaxies[i].0 > *h && galaxies[j].0 < *h ){
                        count += 1;
                    } 
                }
                for v in verticals.iter(){
                    if (galaxies[i].1 < *v && galaxies[j].1 > *v ) || (galaxies[i].1 > *v && galaxies[j].1 < *v ){
                        count += 1;
                    } 
                }
                let shortest_path = (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs() + count;
                sum += shortest_path;
                pairs.push((g_1,g_2));
            }
            j+=1;
        }
        //println!("i: {}", i);
        i+=1;
    }
    println!("count: {}", count);
    println!("sum: {}", sum);
}

fn main() {
    let owned_lines = include_str!("../input.txt")
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let borrowed_lines = owned_lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    //part1(borrowed_lines);
    part2(borrowed_lines);
}
