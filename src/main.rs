use std::error::Error;
use std::process;
use csv::Reader;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    text: String,
    answers: String,
}

fn parse_questions() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("questions.csv")?;
    for result in rdr.deserialize() {
        let record: Question = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = parse_questions() {
        println!("error running question parsing: {}", err);
        process::exit(1);
    }
}
