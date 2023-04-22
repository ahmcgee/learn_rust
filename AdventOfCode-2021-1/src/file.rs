use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_number_txt_to_vector(path: &String) -> Result<Vec<i32>, std::io::Error> {
    let mut numbers = Vec::<i32>::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let number: i32 = line.trim().parse().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to parse line: {:?}", e))
        })?;
        numbers.push(number);
    }

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::load_number_txt_to_vector;

    #[test]
    fn test_load_number_txt_to_vector_can_load_data_file() {
        // arrange
        let file_path = String::from("./data/test_depths.txt");
        // act
        let result = load_number_txt_to_vector(&file_path);
        // assert
        match result {
            Err(_) => panic!(),
            Ok(v) => assert_eq!(5, v.len())
        }
    }

    #[test]
    fn test_load_number_text_to_vector_gracefully_handles_non_number() {
        // arrange
        let file_path = String::from("./data/test_bad_depths.txt");
        // act
        let result = load_number_txt_to_vector(&file_path);
        // assert
        match result {
            Ok(_) => panic!(),
            Err(e) => {
                assert_eq!(std::io::ErrorKind::InvalidData, e.kind());
            }
        }
    }

    #[test]
    fn test_load_number_text_to_vector_gracefully_handles_missing_file() {
        // arrange
        let file_path = String::from("./data/does_not_exist.txt");
        // act
        let result = load_number_txt_to_vector(&file_path);
        // assert
        match result {
            Ok(_) => panic!(),
            Err(e) => {
                assert_eq!(std::io::ErrorKind::NotFound, e.kind())
            }
        }
    }
}