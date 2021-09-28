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

fn ask_questions() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("questions.csv")?;
    let mut scores = Vec::<usize>::new();
    
    for result in rdr.deserialize() {
        let record: Question = result?;
        println!("{:?} ({} to {})", record.text, record.lower_bound, record.upper_bound);
        let answer = get_input();
        scores.push(answer.parse::<usize>()?);
    }

    println!("Your score: {:?}", scores);

    println!("Any notes on this week?");
    let notes = get_input();

    println!("{}", notes);
    Ok(())
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
    buffer.trim().to_string()
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
    if let Err(err) = ask_questions() {
        println!("error running question parsing: {}", err);
        process::exit(1);
    }
    if let Err(err) = parse_scale() {
        println!("error running scale parsing: {}", err);
        process::exit(1);
    }
}
