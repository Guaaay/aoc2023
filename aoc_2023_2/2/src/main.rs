fn main() -> Result<(), String> {
    let mut game_sum = 0;
    let lines = include_str!("input1.txt").lines().collect::<Vec<_>>();
    let mut power_sum = 0;
    //let mut i = 1;
    for line in lines {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let gameid_outings = line.split(": ").collect::<Vec<_>>();
        let game_id = gameid_outings[0].split_once("Game ").map(|(_, after)| after).unwrap_or("").parse::<u32>().unwrap_or(0);
        //println!("game_id = {}", game_id);
        let outings = gameid_outings[1].split("; ").collect::<Vec<_>>();
        //println!("outings = {:?}", outings);
        let mut num_correct = 0;
        let outings_length = outings.len();
        for outing in outings{
            
            let plays = outing.split(", ").collect::<Vec<_>>();

            for play in plays {
                let split_play = play.split(" ").collect::<Vec<_>>();
                let number = split_play[0].parse::<u32>().unwrap_or(0);
                let color = split_play[1];
                //println!("number = {}, color = {}", number, color);
                if color == "red" {
                    if number > max_red {
                        max_red = number;
                    }
                }
                if color == "green"{
                    if number > max_green{
                        max_green = number;
                    }
                }
                if color == "blue" {
                    if number > max_blue {
                        max_blue = number;
                    }
                }
            }
        }
        //println!("game_id: {}", game_id);
        let game_power = max_red * max_green * max_blue;
        power_sum += game_power;
    }
    println!("power_sum = {}", power_sum);
    Ok(())
}

