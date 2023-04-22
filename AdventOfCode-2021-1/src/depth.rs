
pub fn count_depth_increments(depth_values: &Vec<i32>) -> i32 {
    // Uses folding to compare each depth to the previous, tracking running total of increments
    let depth_increments_state = depth_values.into_iter().fold((None, 0), |state, n| -> (Option<i32> /* Previous depth */, i32 /* Total # depth increments */) {
        match &state.0 {
            // None means this is the very first depth examined, which means the depth was not increased
            None => (Some(*n), 0),

            // This is at least the second depth examined, which means the depth may have increased
            Some(prev_n) => {
                if prev_n < n {
                    (Some(*n), state.1 + 1)
                } else {
                    (Some(*n), state.1)
                }
            }
        }
    });
    depth_increments_state.1
}

#[cfg(test)]
mod tests {
    use super::count_depth_increments;

    #[test]
    fn test_count_depth_increments_retrieves_accurate_count() {
        // arrange
        let depth_data = vec![3, 1, 4, 6, 3];
        // act
        let result = count_depth_increments(&depth_data);
        // assert
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_depth_gracefully_handles_empty_list() {
        // arrange
        let depth_data = vec![];
        // act
        let result = count_depth_increments(&depth_data);
        // assert
        assert_eq!(result, 0);
    }
}