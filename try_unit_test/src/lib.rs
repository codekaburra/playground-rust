pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_success() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn should_be_fail() {
        let result = add(2, 2);
        assert_eq!(result, 6);
    }

    #[test]
    #[ignore]
    fn should_be_ignore() {
        let result = add(2, 2);
        assert_eq!(result, 6);
    }

    #[test]
    fn many_cases() {
        for ((input0, input1), output) in vec![((1, 1), 2), ((0, 0), 0), ((0, 2), 2), ((1, 15), 16)]
        {
            assert_eq!(crate::add(input0, input1), output);
        }
    }
}
