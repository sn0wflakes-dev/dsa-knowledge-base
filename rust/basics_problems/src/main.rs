mod solution;
fn main() {
}

#[allow(unused)]
#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn test_is_even_odd() {
        assert_eq!(solution::is_even_odd::is_even_odd_fn(10), "is even");
    }

    #[test]
    fn test_multiplication_table() {
        let output = "1 x 1 = 1 \n1 x 2 = 2 \n1 x 3 = 3 \n1 x 4 = 4 \n1 x 5 = 5 \n1 x 6 = 6 \n1 x 7 = 7 \n1 x 8 = 8 \n1 x 9 = 9 \n1 x 10 = 10 \n";
        assert_eq!(solution::table_multiple::table_multiple_fn(1), output);
    }

    #[test]
    fn test_sum_natural() {
        assert_eq!(solution::sum_natural::sum_natural_fn(5), 15);
    }

    #[test]
    fn test_sum_of_square(){
        assert_eq!(solution::sum_of_square::one_sum_of_square_fn(5), 55);
        assert_eq!(solution::sum_of_square::two_sum_of_square_fn(5), 55);
        assert_eq!(solution::sum_of_square::one_sum_of_square_fn(10), solution::sum_of_square::two_sum_of_square_fn(10));
    }
}
