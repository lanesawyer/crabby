use std::error::Error;
use std::process;
use csv::Reader;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    text: String,
    lower_bound: usize,
    upper_bound: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Score {
    lower_bound: usize,
    upper_bound: usize,
    label: String,
}

fn parse_questions() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("questions.csv")?;
    for result in rdr.deserialize() {
        let record: Question = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn parse_scale() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("scale.csv")?;
    for result in rdr.deserialize() {
        let record: Score = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = parse_questions() {
        println!("error running question parsing: {}", err);
        process::exit(1);
    }
    if let Err(err) = parse_scale() {
        println!("error running scale parsing: {}", err);
        process::exit(1);
    }
}
