#[derive(Debug, PartialEq)]
pub enum CourseDirection {
    Forward(i32),
    Up(i32),
    Down(i32)
}

#[derive(Debug, PartialEq)]
pub enum ParseCourseError {
    // When the line has the wrong number of tokens. First is line number, second is the full string
    UnexpectedNumberOfTokens(usize, String),
    // When the line doesn't match one of the known course names (up, down, forward). Second value is the invalid name.
    InvalidCourseString(usize, String),
    // When the number following the course name can't be parsed. Second value is the unparseable number.
    UnparseableNumber(usize, String)
}

pub fn parse_course(input: &Vec<String>) -> Result<Vec<CourseDirection>, ParseCourseError> {
    input
        .iter()
        .enumerate()
        .map(|(line_num, line_str)| parse_course_string(line_num, &line_str))
        .collect()
}

fn parse_course_string(line_num: usize, input: &String) -> Result<CourseDirection, ParseCourseError> {
    let split_string: Vec<&str> = input.split_whitespace().collect();
    if split_string.len() != 2 {
        Err(ParseCourseError::UnexpectedNumberOfTokens(line_num, input.to_string()))
    } else {
        let parsed_num = split_string[1].parse::<i32>().map_err(|_| ParseCourseError::UnparseableNumber(line_num, split_string[1].to_string()))?;
        match split_string[0] {
            "up" => Ok(CourseDirection::Up(parsed_num)),
            "down" => Ok(CourseDirection::Down(parsed_num)),
            "forward" => Ok(CourseDirection::Forward(parsed_num)),
            _ => Err(ParseCourseError::InvalidCourseString(line_num, split_string[0].to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CourseDirection;
    use super::ParseCourseError;
    use super::parse_course_string;
    use super::parse_course;

    #[test]
    fn parse_course_parses_list() {
        // arrange
        let courses = vec!["up 3".to_string(), "forward 5".to_string(), "down 6".to_string()];
        // act
        let result = parse_course(&courses);
        // assert 
        match result {
            Err(_) => panic!(),
            Ok(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v[0], CourseDirection::Up(3));
                assert_eq!(v[1], CourseDirection::Forward(5));
                assert_eq!(v[2], CourseDirection::Down(6));
            }
        }
    }

    #[test]
    fn parse_course_parses_invalid_list_with_error() {
        // arrange
        let courses = vec!["up 3".to_string(), "forward oops".to_string(), "down 6".to_string()];
        // act
        let result = parse_course(&courses);
        // assert 
        match result {
            Err(ParseCourseError::UnparseableNumber(n, v)) => {
                assert_eq!(n, 1);
                assert_eq!(v, "oops".to_string());
            },
            Err(_) => panic!(),
            Ok(_) => panic!() 
        }
    }



    #[test]
    fn parse_course_string_parses_course_up() {
        // arrange
        let str = "up 5".to_string();
        // act
        let result = parse_course_string(0, &str);
        // assert
        match result {
            Err(_) => panic!(),
            Ok(v) => {
                assert_eq!(CourseDirection::Up(5), v);
            }
        }
    }

    #[test]
    fn parse_course_string_parses_course_down() {
        // arrange
        let str = "down 352".to_string();
        // act
        let result = parse_course_string(5, &str);
        // assert
        match result {
            Err(_) => panic!(),
            Ok(v) => {
                assert_eq!(CourseDirection::Down(352), v);
            }
        }
    }

    #[test]
    fn parse_course_string_parses_course_forward() {
        // arrange
        let str = "forward 2".to_string();
        // act
        let result = parse_course_string(5, &str);
        // assert
        match result {
            Err(_) => panic!(),
            Ok(v) => {
                assert_eq!(CourseDirection::Forward(2), v);
            }
        }
    }

    #[test]
    fn parse_course_string_parses_wildly_invalid_string_with_error() {
        // arrange
        let str = "foobar".to_string();
        // act
        let result = parse_course_string(5, &str);
        // assert
        match result {
            Err(v) => assert_eq!(ParseCourseError::UnexpectedNumberOfTokens(5, "foobar".to_string()), v),
            Ok(_) => panic!()
        }
    }

    #[test]
    fn parse_course_string_parses_invalid_course_operation_with_error() {
        // arrange
        let str = "badop 2".to_string();
        // act
        let result = parse_course_string(9, &str);
        // assert
        match result {
            Err(v) => assert_eq!(ParseCourseError::InvalidCourseString(9, "badop".to_string()), v),
            Ok(_) => panic!()
        }
    }

    #[test]
    fn parse_course_string_parses_unparseable_number_with_error() {
        // arrange
        let str = "up no".to_string();
        // act
        let result = parse_course_string(9, &str);
        // assert
        match result {
            Err(v) => assert_eq!(ParseCourseError::UnparseableNumber(9, "no".to_string()), v),
            Ok(_) => panic!()
        }
    }
}