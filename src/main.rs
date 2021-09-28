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
    let mut csv_reader = Reader::from_path("questions.csv")?;
    
    let scores = csv_reader.deserialize().map(|result| -> usize {
        let record: Question = result.expect("malformed question encountered");
        println!("{:?} ({} to {})", record.text, record.lower_bound, record.upper_bound);
        loop {
            let answer = get_console_input();
            let parsed_answer = answer.parse::<usize>();
            if parsed_answer.is_err() {
                println!("Please enter a number");
            } else if let Ok(parsed) = parsed_answer {
                if parsed >= record.lower_bound && parsed <= record.upper_bound {
                    return parsed;
                }
                println!("Please enter a number in the range");
            }
        }
    }).collect::<Vec<usize>>();

    println!("Your answers: {:?}", scores);
    let total = scores.iter().sum();

    let scale_label = find_answer_on_scale(total)?;
    println!("Total: {} - {}", total, scale_label);

    println!("Any notes on this week?");
    let _notes = get_console_input();

    println!("Got it. See ya next time!");
    Ok(())
}

fn get_console_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
    buffer.trim().to_string()
}

fn find_answer_on_scale(user_answer: usize) -> Result<String, Box<dyn Error>> {
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
        println!("Error asking questions: {}", err);
        process::exit(1);
    }
}
