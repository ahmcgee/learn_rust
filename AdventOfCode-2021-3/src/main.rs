use clap::{arg, command};
use advent_of_code_2021_3::file::load_string_txt_to_vector;
use advent_of_code_2021_3::diagnostic::{calculate_diagnostic_result, diagnostic_grid_from_lines};

fn main() {
    let matches = command!()
        .arg(
            arg!([filename] "The input file containing the diagnostic information")
                .required(true)
                .index(1)
        )
        .get_matches();

    let filename = matches
        .get_one::<String>("filename")
        .expect("Expecting an input file");

    match load_string_txt_to_vector(&filename) {
        Ok(lines) => {
            process_input_lines(lines);
        },
        Err(e) => {
            println!("Failed to load file: {}", e.to_string());
        }
    }
}

fn process_input_lines(lines: Vec<String>) {
    match diagnostic_grid_from_lines(&lines) {
        Ok(grid) => {
            let diagnostic_result = calculate_diagnostic_result(&grid);
            println!("Power consumption: {}", diagnostic_result.power_consumption());
        },
        Err(e) => {
            println!("Parse of diagnostic grid failed on line {}, reason {}", e.line_number, e.why);
        }
    }

}
