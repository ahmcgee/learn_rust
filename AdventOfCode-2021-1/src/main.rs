use advent_of_code_2021_1::file::{load_number_txt_to_vector};
use advent_of_code_2021_1::depth::{count_depth_increments};
use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(
            arg!([filename] "The input file containing the depths")
                .required(true)
                .index(1)
        )
        .get_matches();

    let filename = matches
        .get_one::<String>("filename")
        .expect("Expecting an input file");

    match load_number_txt_to_vector(&filename) {
        Ok(depth_numbers) => {
            let increments_count = count_depth_increments(&depth_numbers);
            println!("There were {} depth increments.", increments_count);
        },
        Err(e) => {
            println!("Failed to load file: {}", e.to_string());
        }
    }
}
