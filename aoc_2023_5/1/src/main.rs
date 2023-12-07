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
    //highest possible int value
    let mut lowest_location = u64::MAX;
    for seed in seeds{
        
        let mut seed_val = seed;
        for map in &maps{
            for range in map{
                //We found the seed's range
                if seed_val >= range[1] && seed_val < range[1] + range[2]{
                    //find the position in the source range:
                    let mut pos = seed_val - range[1];
                    seed_val = range[0] + pos;
                    break;
                }
            }
        }
        if seed_val < lowest_location{
            lowest_location = seed_val;
        }
    }
    println!("{:?}", lowest_location);
}
