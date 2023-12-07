fn main() -> Result<(), String> {
    let mut game_sum = 0;
    let lines = include_str!("../../input.txt").lines().collect::<Vec<_>>();
    let mut i = 0;
    let mut reading_num = false;
    let mut symbol_adjacent = false;
    let mut num_digits: Vec<u32> = Vec::new();
    let mut new_line = false;
    while i < lines.len(){
        new_line = true;
        let mut j = 0;
        let line_len = lines[i].len();
        while j < line_len{
            let mut chara = lines[i].chars().nth(j).unwrap();
            //println!("chara: {}", chara);
            if new_line {
                chara = '.';
            }
            if chara.is_digit(10){
                reading_num = true;
                num_digits.push(chara.to_digit(10).unwrap());
                //Check for symbols around the digit
                let mut i_32 = i as i32;
                let mut j_32 = j as i32;
                for k in 0..3{
                    let k_32 = k as i32;
                    let mut indexk: i32 = i_32+k_32-1;
                    for l in 0..3{
                        let l_32 = l as i32;
                        let mut indexl: i32 = j_32+l_32-1;
                        if (indexk < lines.len().try_into().unwrap() && indexk >= 0) && (indexl < line_len.try_into().unwrap() && indexl >= 0){
                            let mut indexk_usize = indexk as usize;
                            let mut indexl_usize = indexl as usize;
                            let mut adj_char = lines[indexk_usize].chars().nth(indexl_usize).unwrap();
                            if(!adj_char.is_digit(10)){
                                //println!("Adjacent char: {}", adj_char);
                            }
                            
                            if adj_char != '.' && !adj_char.is_digit(10){
                                //println!("Symbol adjacent");
                                symbol_adjacent = true;
                            }
                        }
                    }
                }
            }
            else {
                //println!("Not digit");
                if reading_num{
                    reading_num = false;
                    let mut num = 0;
                    let mut k = 0;
                    while k < num_digits.len(){
                        num += num_digits[k] * 10_u32.pow((num_digits.len() - k - 1) as u32);
                        k += 1;
                    }
                    println!("Num: {}", num);
                    if symbol_adjacent{
                        println!("Symbol adjacent");
                        game_sum += num;
                    }
                    symbol_adjacent = false;
                    num_digits.clear();
                }
            }
            if new_line{
                new_line = false;
            }
            else{
                j += 1;
            }
        }
        i += 1;
    }
    println!("Game sum: {}", game_sum);

    
    
    Ok(())
}

