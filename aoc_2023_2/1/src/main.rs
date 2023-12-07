fn main() -> Result<(), String> {
    let mut game_sum = 0;
    let lines = include_str!("input1.txt").lines().collect::<Vec<_>>();
    let num_red = 12;
    let num_green = 13;
    let num_blue = 14;
    //let mut i = 1;
    for line in lines {
        let gameid_outings = line.split(": ").collect::<Vec<_>>();
        let game_id = gameid_outings[0].split_once("Game ").map(|(_, after)| after).unwrap_or("").parse::<u32>().unwrap_or(0);
        //println!("game_id = {}", game_id);
        let outings = gameid_outings[1].split("; ").collect::<Vec<_>>();
        //println!("outings = {:?}", outings);
        let mut num_correct = 0;
        let outings_length = outings.len();
        for outing in outings{
            
            let plays = outing.split(", ").collect::<Vec<_>>();
            //println!("plays = {:?}", plays);
            let mut check_red = true;
            let mut check_green = true;
            let mut check_blue = true;
            for play in plays {
                let split_play = play.split(" ").collect::<Vec<_>>();
                let number = split_play[0].parse::<u32>().unwrap_or(0);
                let color = split_play[1];
                //println!("number = {}, color = {}", number, color);
                if color == "red" {
                    if number > num_red {
                        check_red = false;
                    }
                }
                if color == "green"{
                    if number > num_green{
                        check_green = false;
                    }
                }
                if color == "blue" {
                    if number > num_blue {
                        check_blue = false;
                    }
                }
            }
            if check_red && check_green && check_blue{
                num_correct += 1;
            }
        }
        //println!("game_id: {}", game_id);
        //println!("num_correct = {}", num_correct);
        //println!("outings_length = {}", outings_length);
        if num_correct == outings_length {
            
            game_sum += game_id;
        }
    }
    println!("game_sum = {}", game_sum);
    Ok(())
}

