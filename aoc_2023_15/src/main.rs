

fn hash_algorithm(s: &str) -> u32 {
    let mut curr_val = 0;
    for c in s.chars() {
        if c == '\n' {
            continue;
        }
        curr_val += c as u32;
        curr_val *= 17;
        curr_val %= 256;
    }
    curr_val
}



fn part2(input: &str) {
    let sequences = input.split(',').collect::<Vec<&str>>();
    let mut game_sum = 0;
    let mut boxes = vec![Vec::<(&str,usize)>::new(); 256];
    for s in &sequences {
        let box_num;
        let lens_name: &str;
        if s.contains('-'){
            let seq = s.split('-').collect::<Vec<&str>>();
            lens_name = seq[0];
            box_num = hash_algorithm(lens_name);
            if !boxes[box_num as usize].iter().any(|&x| x.0 == lens_name) {
                continue;
            } 
            let pos = boxes[box_num as usize].iter().position(|&x| x.0 == lens_name).unwrap();
            boxes[box_num as usize].remove(pos);
        }
        else {
            let seq = s.split('=').collect::<Vec<&str>>();
            lens_name = seq[0];
            box_num = hash_algorithm(lens_name);
            let lens_focal_length = seq[1].parse::<usize>().unwrap();
            
            if let None = boxes[box_num as usize].iter_mut().find_map(|(key, value)| {
                if key == &lens_name {
                    *value = lens_focal_length;
                    Some(())
                } else {
                    None
                }
            }) {
                boxes[box_num as usize].push((lens_name,lens_focal_length));
            }
        }
    }

    //Calculate the result
    let mut i = 0;
    while i < 256 {
        let mut j = 1;
        for lens in &boxes[i] {
            game_sum += (i+1) * j * lens.1;
            j+=1;
        }
        i+=1;
    }
    println!("{}", game_sum);
}

fn part1(input: &str) {
    let sequences = input.split(',').collect::<Vec<&str>>();
    let mut game_sum = 0;
    sequences.iter().for_each(|seq| {
        let seq_sum = hash_algorithm(seq);
        game_sum += seq_sum;
    });

    println!("{}", game_sum);
}

fn main() {
    let line = include_str!("../input.txt");
    part2(line);
}
