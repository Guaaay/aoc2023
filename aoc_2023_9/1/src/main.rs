fn main() {
    let lines = include_str!("../../bigboy9.txt").lines().collect::<Vec<_>>();
    let mut game_sum: i64 = 0;
    for line in lines {
        //println!("--------------");
        let mut measurements = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let mut done = false;
        let mut end = measurements.len()-1;
        while !done{
            let mut i = 0;
            //println!("measurements: {:?}", measurements);
            while i < end{
                measurements[i] = measurements[i+1] - measurements[i]; 
                i+=1;
            }
            done = measurements.iter().take(end).all(|&x| x == 0);
            if !done{
                end -= 1;
            }
        }
        let mut init: i64 = 0;
        while end < measurements.len(){
            
            measurements[end] = init + measurements[end];
            init = measurements[end];
            end += 1;
            //println!("measurements: {:?}", measurements);
        }
        let result: i64 = init;
        game_sum += result;
        //println!("result: {}", result);
        
    }
    println!("game_sum: {}", game_sum);
}
