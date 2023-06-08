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

pub fn navigate(operations: &Vec<CourseDirection>) -> SubCoordinates {
    let mut d = 0;
    let mut h = 0;
    for op in operations.iter() {
        match op {
            CourseDirection::Up(n) => {
                d -= n;
            },
            CourseDirection::Down(n) => {
                d += n;
            },
            CourseDirection::Forward(n) => {
                h += n;
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
    fn navigation_has_expected_output() {
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
        let result = navigate(&course);

        // assert 
        assert_eq!(result, SubCoordinates { depth: 10, horizontal: 15 });
    }
}