use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_string_txt_to_vector(path: &String) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let strings: Result<Vec<_>, _> = reader
        .lines()
        .map(|line| line.map(|l| l.trim().to_string()))
        .collect();

    strings
}

#[cfg(test)]
mod tests {
    use super::load_string_txt_to_vector;

    #[test]
    fn test_load_string_txt_to_vector_can_load_data_file() {
        // arrange
        let file_path = String::from("./test_data/test_file.txt");
        // act
        let result = load_string_txt_to_vector(&file_path);
        // assert
        match result {
            Err(_) => panic!(),
            Ok(v) => {
                assert_eq!(3, v.len());
                assert_eq!("alpha".to_string(), v[0]);
                assert_eq!("beta".to_string(), v[1]);
                assert_eq!("gamma".to_string(), v[2]);
            }
        }
    }

    #[test]
    fn test_load_string_text_to_vector_gracefully_handles_missing_file() {
        // arrange
        let file_path = String::from("./test_data/does_not_exist.txt");
        // act
        let result = load_string_txt_to_vector(&file_path);
        // assert
        match result {
            Ok(_) => panic!(),
            Err(e) => {
                assert_eq!(std::io::ErrorKind::NotFound, e.kind())
            }
        }
    }
}