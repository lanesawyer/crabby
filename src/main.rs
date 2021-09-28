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

    let scale_label = parse_scale(scores.iter().sum())?;
    println!("{}", scale_label);

    println!("Any notes on this week?");
    let _notes = get_input();

    println!("Got it. See ya next time!");
    Ok(())
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
    buffer.trim().to_string()
}

fn parse_scale(user_answer: usize) -> Result<String, Box<dyn Error>> {
    let mut rdr = Reader::from_path("scale.csv")?;
    for result in rdr.deserialize() {
        let record: Score = result?;
        if user_answer >= record.lower_bound && user_answer <= record.upper_bound {
            return Ok(record.label);
        }
    }
    Ok("Didn't find score".to_string())
}

fn main() {
    if let Err(err) = ask_questions() {
        println!("error running question parsing: {}", err);
        process::exit(1);
    }
}
