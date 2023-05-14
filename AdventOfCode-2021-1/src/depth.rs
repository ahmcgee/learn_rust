pub fn count_depth_increments(depth_values: &[i32]) -> usize {
    // Uses folding to compare each depth to the previous, tracking running total of increments
    let depth_increments = depth_values
        .windows(2)
        .filter(|window| -> bool { window.len() == 2 && window[0] < window[1] })
        .count();
    return depth_increments;
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