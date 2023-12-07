fn main() -> Result<(), String> {
    let mut game_sum: u64 = 0;
    let lines = include_str!("../../input.txt").lines().collect::<Vec<_>>();
    let mut i = 0;
    let mut new_line = false;
    while i < lines.len(){
        let mut j = 0;
        let line_len = lines[i].len();
        while j < line_len{
            let mut chara = lines[i].chars().nth(j).unwrap();
            if chara == '*'{
                println!("Gear found");
                let mut num_1 = 0;
                let mut num_2 = 0;
                //We search around the gear
                let mut i_32 = i as i32;
                let mut j_32 = j as i32;
                let mut k = 0;
                while k < 3{
                    let k_32 = k as i32;
                    let mut indexk: i32 = i_32+k_32-1;
                    let mut l = 0;
                    while l < 3{
                        let l_32 = l as i32;
                        let indexl: i32 = j_32+l_32-1;
                        if (indexk < lines.len().try_into().unwrap() && indexk >= 0) && (indexl < line_len.try_into().unwrap() && indexl >= 0){
                            let indexk_usize = indexk as usize;
                            let indexl_usize = indexl as usize;
                            let adj_char = lines[indexk_usize].chars().nth(indexl_usize).unwrap();
                            if adj_char.is_digit(10){

                                println!("number found");
                                let mut ret_l = indexl_usize;
                               
                                while lines[indexk_usize].chars().nth(ret_l).unwrap().is_digit(10){
                                    //println!("ret_l: {}", ret_l);
                                    if(ret_l < line_len-1){
                                        ret_l += 1;
                                    }
                                    else{
                                        break;
                                    }
                                        
                                }
                                if(ret_l != 0 && ret_l < line_len && !lines[indexk_usize].chars().nth(ret_l).unwrap().is_digit(10)){
                                    ret_l -= 1;
                                }
                                let mut curr_num = 0;
                                let mut count = 0;
                                //println!("char: {}", lines[indexk_usize].chars().nth(ret_l).unwrap());
                                //println!("ret_l: {}", ret_l);
                                while (lines[indexk_usize].chars().nth(ret_l).unwrap().is_digit(10)){
                                    curr_num += lines[indexk_usize].chars().nth(ret_l).unwrap().to_digit(10).unwrap() * 10_u32.pow(count);
                                    //println!("char: {}", lines[indexk_usize].chars().nth(ret_l).unwrap());
                                    if(ret_l != 0){
                                        count += 1;
                                        ret_l -= 1;
                                    }
                                    else{
                                        break;
                                    }
                                }
                                if num_1 == 0{
                                    num_1 = curr_num;
                                }
                                else{
                                    num_2 = curr_num;
                                }
                            }
                        }
                        l += 1;
                    }
                    k += 1;
                }
                println!("Num 1: {}", num_1);
                println!("Num 2: {}", num_2);
                if(num_1 == num_2){
                    num_2 = 0;
                }
                let mult = num_1 * num_2;
                game_sum += mult as u64;
            }
            
            j += 1;
        }
        i += 1;
    }
    println!("Game sum: {}", game_sum);

    
    
    Ok(())
}

