use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use csv::Reader;

struct Player {
    name: String,
    score: f64,
}

fn read_csv(file_path: &str) -> Result<HashMap<String, Player>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut players: HashMap<String, Player> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        
        // Start from column 1 to skip the date
        for chunk in record.iter().skip(1).collect::<Vec<_>>().chunks(2) {
            if let [player, score] = chunk {
                let score: f64 = score.parse().unwrap_or(0.0); // Convert to f64, default 0.0 on error

                // Add player to hashmap if not present, otherwise accumulate score
                players.entry(player.to_string())
                    .and_modify(|p| p.score += score)
                    .or_insert(Player { name: player.to_string(), score });
            }
        }
    }

    Ok(players)
}

fn print_players(players: &HashMap<String, Player>) {
    println!("{:<10} | Score", "Player");
    println!("{}", "-".repeat(25));

    for player in players.values() {
        println!("{:<10} | {:.2}", player.name, player.score);
    }
}

fn main() {
    let file_path = "games.csv";
    match read_csv(file_path) {
        Ok(players) => print_players(&players),
        Err(err) => eprintln!("Error: {}", err),
    }
}
