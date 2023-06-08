use clap::{arg, command};
use advent_of_code_2021_2::file::load_string_txt_to_vector;
use advent_of_code_2021_2::navigation::navigate;
use advent_of_code_2021_2::course::parse_course;

fn main() {
    let matches = command!()
        .arg(
            arg!([filename] "The input file containing the course information")
                .required(true)
                .index(1)
        )
        .get_matches();

    let filename = matches
        .get_one::<String>("filename")
        .expect("Expecting an input file");

    match load_string_txt_to_vector(&filename) {
        Ok(lines) => {
            process_input_lines(&lines);
        },
        Err(e) => {
            println!("Failed to load file: {}", e.to_string());
        }
    }
}

fn process_input_lines(lines: &Vec<String>) {
    match parse_course(lines) {
        Ok(parsed_lines) => {
            println!("Calculated coordinates: {}", navigate(&parsed_lines).to_string());
        },
        Err(_) => {
            // TODO: More nuanced handling
            println!("Failed to parse lines");
        }
    }
}