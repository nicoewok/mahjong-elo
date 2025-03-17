use std::error::Error;
use std::collections::HashMap;
use csv::Reader;

struct Player {
    name: String,
    score: f64,
}

fn add_score (player: &mut Player, score: f64) {
    player.score += score;
}

fn read_csv(file_path: &str) -> Result<HashMap<String, Player>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut players: HashMap<String, Player> = HashMap::new();

    for result in rdr.records() {
        let mut sanma = false;
        let record = result?;
        
        //check if last column is empty
        if record[record.len()-1].is_empty() {
            sanma = true;
        }
        
        //Start from column 1 to skip the date
        //collect the player name and score in list of pairs
        let mut player_score: Vec<(&str, f64)> = Vec::new();

        for chunk in record.iter().skip(1).collect::<Vec<&str>>().chunks(2) {
            if let [player, score] = chunk {
                if player.is_empty() || score.is_empty() {
                    continue; // Skip empty entries for sanma
                }

                let score: f64 = score.parse().unwrap_or(0.0); // Convert to f64, default 0.0 on error
                player_score.push((player, score));
            }
        }

        //sort player_score by score
        player_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (i, (player, score)) in player_score.iter_mut().enumerate() {

            if !sanma {
                //default score reduction
                *score -= 25.0;

                //switch statement on i
                match i {
                    0 => *score += 30.0,
                    1 => *score += 10.0,
                    2 => *score -= 10.0,
                    _ => *score -= 30.0,
                }
            } else {
                *score -= 30.0;
                
                match i {
                    0 => *score += 30.0,
                    1 => *score += 0.0,
                    //no match for 1 as this player is left neutral
                    _ => *score -= 30.0,
                }
            }

            //add the score to the player
            //check if player exists in the hashmap
            if let Some(player) = players.get_mut(*player) {
                add_score(player, *score);
            } else {
                players.insert((*player).to_string(), Player { name: (*player).to_string(), score: *score });
            }
        }

    }

    Ok(players)
}

fn print_players(players: &HashMap<String, Player>) {
    println!("{:<10} | Score", "Player");
    println!("{}", "-".repeat(25));

    //sort players by score
    let mut players: Vec<(&String, &Player)> = players.iter().collect();
    players.sort_by(|a, b| b.1.score.partial_cmp(&a.1.score).unwrap());

    for (_, player) in players {
        println!("{:<10} | {}", player.name, player.score);
    }
}

fn main() {
    let file_path = "games.csv";
    match read_csv(file_path) {
        Ok(players) => print_players(&players),
        Err(err) => eprintln!("Error: {}", err),
    }
}
