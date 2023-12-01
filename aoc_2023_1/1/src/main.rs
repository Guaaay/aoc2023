fn main() -> Result<(), String> {
    let lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    let mut sum = 0;
    for line in lines {
        let mut first_digit = 0;
        let mut second_digit = 0;
        let mut done_first = false;
        let mut done_second = false;
        for c in line.chars() {
            if(c.is_digit(10) && !done_first) {
                first_digit = c.to_digit(10).unwrap_or(0);
                done_first = true;
            }
            else if(c.is_digit(10) && done_first) {
                second_digit = c.to_digit(10).unwrap_or(0);
                done_second = true;
            }
        }
        if(!done_second){
            second_digit = first_digit;
        }
        let result = 10*first_digit + second_digit;
        sum += result;
        println!("value = {}", result);
    }
    
    println!("result = {sum:?}");
    Ok(())
}
