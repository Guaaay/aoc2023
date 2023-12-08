#[derive(Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    _type: u32,
}
impl Hand {
    fn is_bigger_than(&self,hand2:&Self) -> bool{
        if self._type > hand2._type{
            return true;
        }
        if self._type < hand2._type{
            return false;
        }
        if self._type == hand2._type{
            let mut i = 0;
            while i < 5{
                let card1 = self.cards[i];
                let card2 = hand2.cards[i];
                
                if card1_bigger(card1,card2){
                    return true;
                }
                if card1 != card2{
                    return false;
                }
                i += 1;
            }
            return false;
        }
        return false;
    }
}
fn card1_bigger(card1:char,card2:char) -> bool{
    if card1 == 'A' && card2 != 'A'{return true;}
    if card1 != 'A' && card2 == 'A'{return false;}
    if card1 == 'K' && card2 != 'K'{return true;}
    if card1 != 'K' && card2 == 'K'{return false;}
    if card1 == 'Q' && card2 != 'Q'{return true;}
    if card1 != 'Q' && card2 == 'Q'{return false;}
    if card1 == 'T' && card2 != 'T'{return true;}
    if card1 != 'T' && card2 == 'T'{return false;}
    //Joker is the worst card numerically
    if card1 == 'J' && card2 != 'J'{return false;}
    if card1 != 'J' && card2 == 'J'{return true;}
    if card1 > card2{
        return true;
    }
    return false;
}


fn get_type(hand: &Vec<char>) -> u32 {
    struct Rep {
        card: char,
        count: u32,
    }
    let mut reps: Vec<Rep> = Vec::new();
    let mut num_j = 0;
    for card in hand {
        if card == &'J'{
            num_j += 1;
            continue; //Five of a kind with joker
        }
        let mut found = false;
        for rep in reps.iter_mut() {
            if rep.card == *card{
                rep.count += 1;
                found = true;
                break;

            }
        }

        if !found {
            reps.push(Rep { card: *card, count: 1 });
        }
    }
    //Add jokers to every rep
    for rep in reps.iter_mut(){
        rep.count += num_j;
    }

    if reps.len() == 1 || num_j == 5{
        return 7; //Five of a kind
    }
    if reps.len() == 2{
        for rep in reps{
            if rep.count == 4{
                return 6; //Four of a kind
            }
        }
        return 5; //Full house
    }
    if reps.len() == 3{
        for rep in reps{
            if rep.count == 3{
                return 4; //Three of a kind
            }
        }
        return 3; //Two pair
    }
    if reps.len() == 4{
        return 2; //One pair
    }
    else{
        return 1; //High card
    }
}


fn main() {
    let mut game_sum: u64 = 0;
    let lines = include_str!("../../megainput.txt").lines().collect::<Vec<_>>();
    let mut hands: Vec<Hand> = Vec::new();
    for line in &lines {
        //println!("{}",line);
        let split = line.split(" ").collect::<Vec<_>>();
        let cards = split[0].chars().collect::<Vec<_>>();
        let bid = split[1].parse::<u32>().unwrap();
        let _type = get_type(&cards);
        hands.push(Hand { cards, bid, _type });
    }
    //Sort hands
    hands.sort_by(|a,b| {
        if a.is_bigger_than(b) {
            return std::cmp::Ordering::Greater;
        } 
        else if b.is_bigger_than(a) {
            std::cmp::Ordering::Less
        } 
        else {
            std::cmp::Ordering::Equal
        }
    });
    let mut i = 0;
    while i < hands.len(){
        //println!("Rank: {}, Hand: {:?}, bid: {}, type: {}",i+1,hands[i].cards,hands[i].bid,hands[i]._type);
        game_sum += hands[i].bid as u64 * (i+1) as u64;
        i += 1;
    }
    println!("Game sum: {}",game_sum);
}
