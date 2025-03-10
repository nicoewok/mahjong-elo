use std::error::Error;
use std::fs::File;
use csv::Reader;

fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv("games.csv") {
        eprintln!("Error reading CSV: {}", err);
    }
}
