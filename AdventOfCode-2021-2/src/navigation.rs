use crate::course::CourseDirection;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct SubCoordinates {
    depth: i32,
    horizontal: i32
}

impl fmt::Display for SubCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Depth: {}, Horizontal: {}", self.depth, self.horizontal)
    }
}

pub fn navigate(operations: &Vec<CourseDirection>, with_aim: bool) -> SubCoordinates {
    let mut d: i32 = 0;
    let mut h: i32 = 0;
    let mut aim: i32 = 0;
    for op in operations.iter() {
        match op {
            CourseDirection::Up(n) => {
                if with_aim {
                    aim -= n;
                } else {
                    d -= n;
                }
            },
            CourseDirection::Down(n) => {
                if with_aim {
                    aim += n;
                } else {
                    d += n;
                }
            },
            CourseDirection::Forward(n) => {
                h += n;
                if with_aim {
                    d += aim * n;
                }
            }
        }
    }

    SubCoordinates { depth: d, horizontal: h }
}

#[cfg(test)]
mod tests {
    use crate::course::CourseDirection;
    use super::navigate;
    use super::SubCoordinates;

    #[test]
    fn navigation_has_expected_output_without_aim() {
        // arrange
        let course = vec![
            CourseDirection::Forward(5),
            CourseDirection::Down(5),
            CourseDirection::Forward(8),
            CourseDirection::Up(3),
            CourseDirection::Down(8),
            CourseDirection::Forward(2),
        ];
            
        // act
        let result = navigate(&course, false);

        // assert 
        assert_eq!(result, SubCoordinates { depth: 10, horizontal: 15 });
    }

    #[test]
    fn navigation_has_expected_output_with_aim() {
        // arrange
        let course = vec![
            CourseDirection::Forward(5),
            CourseDirection::Down(5),
            CourseDirection::Forward(8),
            CourseDirection::Up(3),
            CourseDirection::Down(8),
            CourseDirection::Forward(2),
        ];
            
        // act
        let result = navigate(&course, true);

        // assert 
        assert_eq!(result, SubCoordinates { depth: 60, horizontal: 15 });
    }
}