fn main() {
    let lines = include_str!("../../input.txt").lines().collect::<Vec<_>>();
    //let mut i = 1;
    let mut seeds = lines[0].split("seeds: ").collect::<Vec<_>>()[1].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
    println!("{:?}", seeds);
    let mut maps = Vec::new();
    let mut map_count = 0;
    let mut count = 0;
    for line in lines {
        //if line is line break
        //println!("line: {}", line);
        if line == "" || line.contains("seeds"){
            continue;
        }
        else if line.contains("map"){
            let new_map = Vec::new();
            maps.push(new_map);
            map_count += 1;
            count = 0;
        }
        else{
            let split_line = line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
            let new_ranges = Vec::new();
            maps[map_count-1].push(new_ranges);
            for number in split_line{
                maps[map_count-1][count].push(number);
            }
            count += 1;
        }
    }
    let mut lowest_location = 0;
    let mut found = false;
    while !found{
        //println!("lowest_location: {}", lowest_location);
        let mut val = lowest_location;
        for map in maps.clone().into_iter().rev(){
            //println!("map: {:?}", map);
            for range in map {
                if val >= range[0] && val < range[0] + range[2]{
                    let mut pos = val - range[0];
                    val = range[1] + pos;
                    break;
                }
            }
        }
        let mut i = 0;
        while i < seeds.len(){
            if val >= seeds[i] && val < seeds[i] + seeds[i+1]{
                //println!("found: {}", val);
                found = true;
                break;
            }
            i += 2;
        }
        if found{
            break;
        }
        if !found{
            lowest_location += 1;
        }
        if lowest_location % 100000 == 0{
            println!("lowest_location: {}", lowest_location);
        }
    }
    
    println!("{:?}", lowest_location);
}
