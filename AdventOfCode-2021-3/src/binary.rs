pub fn binary_to_number(binary: &Vec<bool>) -> u32 {
    binary.iter().fold(0, |acc, &b| (acc << 1) | b as u32)
}

#[cfg(test)]
mod tests {
    use super::binary_to_number;
    #[test]
    fn binary_to_number_empty_is_zero() {
        assert_eq!(0, binary_to_number(&Vec::new()));
    }

    #[test]
    fn binary_to_number_zero_is_zero() {
        assert_eq!(0, binary_to_number(&vec![false, false, false, false]));
    }

    #[test]
    fn binary_to_number_one_is_one() {
        assert_eq!(1, binary_to_number(&vec![false, false, false, true]));
    }

    #[test]
    fn binary_to_number_nine_is_nine() {
        assert_eq!(9, binary_to_number(&vec![false, true, false, false, true]));
    }
}