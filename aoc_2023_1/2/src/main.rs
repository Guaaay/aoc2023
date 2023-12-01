
fn str_vec () -> Vec<&'static str> {
    //Push all the digits in string form into a vector
    let mut result = Vec::new();
    result.push("one");
    result.push("two");
    result.push("three");
    result.push("four");
    result.push("five");
    result.push("six");
    result.push("seven");
    result.push("eight");
    result.push("nine");

    return result;
}

fn str_to_int(s: &str) -> u32 {
    let mut result = 0;
    if s == "one"{
        result = 1;
    }
    else if s == "two"{
        result = 2;
    }
    else if s == "three"{
        result = 3;
    }
    else if(s == "four"){
        result = 4;
    }
    else if(s == "five"){
        result = 5;
    }
    else if(s == "six"){
        result = 6;
    }
    else if(s == "seven"){
        result = 7;
    }
    else if(s == "eight"){
        result = 8;
    }
    else if(s == "nine"){
        result = 9;
    }
    else {
        result = 0;
    }
    return result;
}


fn main() -> Result<(), String> {
    
    let lines = include_str!("input2.txt").lines().collect::<Vec<_>>();
    let mut sum = 0;
    let digits = str_vec();
    let mut i = 1;
    for line in lines {
        println!("line = {}", i);
        i+=1;
        let mut min_index = 10000;
        let mut min_digit = 10000;
        let mut max_index = 0;
        let mut max_digit = 0;
        for digit in digits.iter() {
            //println!("digit = {}", digit);
            let mut start = 0;
        while let Some(index) = line[start..].find(digit) {
            let actual_index = start + index;
            if actual_index < min_index {
                min_index = actual_index;
                min_digit = str_to_int(digit);
            }
            if actual_index > max_index {
                max_index = actual_index;
                max_digit = str_to_int(digit);
            }
            start = actual_index + 1; // Update start for next search
        }
        }
        let mut i = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                //println!("c1 = {}", c);
                let digit_int = c.to_digit(10).unwrap_or(0);
                if i < min_index {
                    min_index = i;
                    min_digit = digit_int;
                }
                if i > max_index {
                    max_index = i;
                    max_digit = digit_int;
                }
                //println!("min_digit_index_c = {}", min_digit_index);
            }
            i+=1;
        }

        if(max_index == 0){
            max_digit = min_digit;
        }
        if(min_index == 10000){
            min_digit = max_digit;
        }




    
        let result = 10*min_digit + max_digit;
        sum += result;
        println!("value = {}", result);
    }
    
    println!("result = {sum:?}");
    Ok(())
}
