fn main() {

    let use_example = false;
    let mut game_sum = 1;
    let times_ex: Vec<u64> = vec![71530];
    let dist_ex: Vec<u64> = vec![940200];
    let times_in: Vec<u64> = vec![44899691];
    let dist_in: Vec<u64> = vec![277113618901768];

    let mut times: Vec<u64> = Vec::new();
    let mut dist: Vec<u64> = Vec::new();

    if use_example{
        for i in 0..times_ex.len(){
            times.push(times_ex[i]);
            dist.push(dist_ex[i]);
        }
    }
    else{
        for i in 0..times_in.len(){
            times.push(times_in[i]);
            dist.push(dist_in[i]);
        }
    }
    for i in 0..times.len(){
        let time = times[i];
        let distance = dist[i];
        let mut ways_to_win = 0;
        for press_time in 1..time {
            let calc_distance = press_time * (time - press_time);
            if calc_distance > distance {
                ways_to_win += 1;
            }
        }
        println!("Time: {}, Distance: {}, Ways to win: {}", time, distance, ways_to_win);
        game_sum = game_sum * ways_to_win;
    }
    println!("Game sum: {}", game_sum);

}
