mod util;

fn main() {
}

#[cfg(test)]
#[allow(unused)]
pub mod test{
    use super::*;

    #[test]
    fn test_sum_of_two_digits() {
        assert_eq!(util::sum_of_two_digits::digit_extraction(12345), 15);
        assert_eq!(util::sum_of_two_digits::recursion(12345), 15);
        assert_eq!(util::sum_of_two_digits::convert_to_string(12345), 15);
    }
}
