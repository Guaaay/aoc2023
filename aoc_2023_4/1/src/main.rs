fn main() -> Result<(), String> {
    let mut game_sum = 0;
    let lines = include_str!("../../input.txt").lines().collect::<Vec<_>>();
    //let mut i = 1;
    let mut cards: Vec<u32> = Vec::new();
    for line in &lines {
        let card_id_games = line.split(": ").collect::<Vec<_>>();
        cards.push(1);
    }
    let mut i = 0;
    for line in &lines {
        let card_id_games = line.split(": ").collect::<Vec<_>>();
        let card_id = card_id_games[0].split_once("Card ").map(|(_, after)| after).unwrap_or("").parse::<u32>().unwrap_or(0);
        let card_numbers = card_id_games[1].split(" | ").collect::<Vec<_>>();
        let winners = card_numbers[0].split(" ").collect::<Vec<_>>();
        let numbers = card_numbers[1].split(" ").collect::<Vec<_>>();
        let mut card_points = 0;
        for winner in &winners {
            let winner_i = winner.parse::<u32>().unwrap_or(0);
            for number in &numbers {
                let number_i = number.parse::<u32>().unwrap_or(0);
                if(number_i == 0 || winner_i == 0){
                    continue;
                }
                if winner_i == number_i {
                    card_points += 1;
                    break;
                }
            }
        }
        let mut k = 0;
        while k < cards[i]{
            let mut j = 0;
            while j < card_points {
                if(i+1+j >= cards.len()){
                    break;
                }
                cards[i+1+j]+=1;
                j += 1;
            }
            k+=1;
        }
        
        

        i += 1;
    }
    let solution: u32 = cards.iter().sum();
    println!("solution = {}", solution);
    Ok(())
}

