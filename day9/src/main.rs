
fn solve(players: usize, last_marble: u32) -> u32 {
    let mut scores: Vec<u32> = vec![0; players];
    let mut circle: Vec<u32> = vec![0];

    let marbles = 1..=last_marble;
    let players_iter = (0..players).into_iter().cycle();
    
    for (marble, player) in marbles.zip(players_iter) {
        if marble % 23 == 0 {
            let (index, value) = circle.iter().enumerate().rev().cycle().skip(6).next().unwrap();
            let to_add = marble + circle.remove(index);
            scores[player] += to_add;
            circle.rotate_left(index);
        } else {
            let index = circle.iter().enumerate().cycle().skip(2).next().unwrap().0;
            circle.insert(index, marble);
            circle.rotate_left(index);
        }
        
    }

    *scores.iter().max().unwrap()
}

fn main() {
    println!("Result: {:#?}", solve(459, 71320));
}
