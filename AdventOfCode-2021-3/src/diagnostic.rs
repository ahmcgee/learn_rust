use crate::grid::DynamicHeightGrid;
use crate::grid::Transpose;
use crate::binary::binary_to_number;

pub struct DiagnosticResult {
    pub gamma_rate: u32,
    pub epsilon_rate: u32
}

#[derive(PartialEq, Eq, Debug)]
pub struct DiagnosticParseError {
    pub line_number: usize,
    pub invalid_line: String,
    pub why: String
} 

impl DiagnosticResult {
    pub fn power_consumption(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

pub fn diagnostic_grid_from_lines(lines: &Vec<String>) -> Result<DynamicHeightGrid<bool>, DiagnosticParseError> {
    if lines.len() == 0 {
        return Ok(DynamicHeightGrid::new(0))
    }

    let mut grid = DynamicHeightGrid::new(lines[0].len());

    for (i, line) in lines.iter().enumerate() {
        let bool_line_with_errs: Vec<Result<bool, DiagnosticParseError>> = line
            .chars()
            .map(|c| char_to_bool(c).ok_or(DiagnosticParseError {
                line_number: i + 1,
                invalid_line: line.to_string(),
                why: "Invalid boolean character".to_string()
            }))
            .collect();
        
        let bool_line: Vec<bool> = bool_line_with_errs
            .into_iter()
            .collect::<Result<Vec<bool>, DiagnosticParseError>>()?;

        grid.push(bool_line).map_err(|_| DiagnosticParseError {
            line_number: i + 1,
            invalid_line: line.to_string(),
            why: "Line length mismatches first line".to_string()
        })?;
    }

    return Ok(grid);
}

pub fn char_to_bool(c: char) -> Option<bool> {
    match c {
        '0' => {
            Some(false)
        },
        '1' => {
            Some(true)
        },
        _ => {
            None
        }
    }
}

pub fn calculate_diagnostic_result(values: &DynamicHeightGrid<bool>) -> DiagnosticResult {
    if values.len() == 0 {
        // We get nonsense if we're passed an empty grid,
        // so handle this case explicitly with result of 0
        DiagnosticResult {
            gamma_rate: 0,
            epsilon_rate: 0
        }
    } else {
        let values_transposed = values.transpose();
        let collapsed_gamma: Vec<bool> = values_transposed
            .iter()
            .map(most_common_bool)
            .collect();

        let collapsed_epsilon: Vec<bool> = collapsed_gamma
            .iter()
            .map(|b| !b)
            .collect();

        DiagnosticResult {
            gamma_rate: binary_to_number(&collapsed_gamma),
            epsilon_rate: binary_to_number(&collapsed_epsilon)
        }
    }
}

fn most_common_bool(booleans: &Vec<bool>) -> bool {
    let l = booleans.len();
    let l_true = booleans
        .iter()
        .filter(|b| **b)
        .count();

    l_true >= l / 2
}

#[cfg(test)]
mod tests {
    use super::DiagnosticResult;
    use super::calculate_diagnostic_result;
    use super::diagnostic_grid_from_lines;
    use crate::grid::DynamicHeightGrid;

    #[test]
    fn diagnostic_result_power_consumption_multiplies() {
        // arrange
        let diag = DiagnosticResult {
            gamma_rate: 3,
            epsilon_rate: 4
        };
        // act
        let result = diag.power_consumption();
        // assert
        assert_eq!(12, result);
    }

    // This test verifies behaviour against the advent of code example
    #[test]
    fn calculate_diagnostic_result_matches_example() {
        // arrange 
        let mut grid = DynamicHeightGrid::<bool>::new(5);
        grid.push(vec![false, false, true, false, false]).unwrap();
        grid.push(vec![true, true, true, true, false]).unwrap();
        grid.push(vec![true, false, true, true, false]).unwrap();
        grid.push(vec![true, false, true, true, true]).unwrap();
        grid.push(vec![true, false, true, false, true]).unwrap();
        grid.push(vec![false, true, true, true, true]).unwrap();
        grid.push(vec![false, false, true, true, true]).unwrap();
        grid.push(vec![true, true, true, false, false]).unwrap();
        grid.push(vec![true, false, false, false, false]).unwrap();
        grid.push(vec![true, true, false, false, true]).unwrap();
        grid.push(vec![false, false, false, true, false]).unwrap();
        grid.push(vec![false, true, false, true, false]).unwrap();
        // act
        let result = calculate_diagnostic_result(&grid);
        // assert
        assert_eq!(22, result.gamma_rate);
        assert_eq!(9, result.epsilon_rate);
        assert_eq!(198, result.power_consumption());
    }

    #[test]
    fn calculate_empty_grid_is_zero() {
        // arrange/act
        let result = calculate_diagnostic_result(&DynamicHeightGrid::<bool>::new(5));
        // assert
        assert_eq!(0, result.gamma_rate);
        assert_eq!(0, result.epsilon_rate);
    }

    #[test]
    fn diagnostic_grid_from_lines_parses_valid() {
        // arrange
        let data = vec!["0100".to_string(), "1110".to_string()];
        // act
        let result = diagnostic_grid_from_lines(&data).unwrap();
        // assert
        assert_eq!(2, result.len());
        assert_eq!(vec![false, true, false, false], result[0]);
        assert_eq!(vec![true, true, true, false], result[1]);
    }

    #[test]
    fn diagnostic_grid_from_empty_lines() {
        // arrange
        let data: Vec<String> = Vec::new();
        // act
        let result = diagnostic_grid_from_lines(&data).unwrap();
        // assert
        assert_eq!(0, result.len());
    }

    #[test]
    fn diagnostic_grid_from_mismatched_lines_fails() {
        // arrange
        let data = vec!["0100".to_string(), "111".to_string()];
        // act
        let result = diagnostic_grid_from_lines(&data);
        // assert
        match result {
            Ok(_) => {
                panic!("Expected failure");
            },
            Err(e) => {
                assert_eq!(2, e.line_number);
                assert_eq!("111".to_string(), e.invalid_line);
                assert_eq!("Line length mismatches first line".to_string(), e.why);
            }
        }
    }

    #[test]
    fn diagnostic_grid_from_invalid_char_fails() {
        // arrange
        let data = vec!["0102".to_string()];
        // act
        let result = diagnostic_grid_from_lines(&data);
        // assert
        match result {
            Ok(_) => {
                panic!("Expected failure");
            },
            Err(e) => {
                assert_eq!(1, e.line_number);
                assert_eq!("0102".to_string(), e.invalid_line);
                assert_eq!("Invalid boolean character".to_string(), e.why);
            }
        }
    }
}
