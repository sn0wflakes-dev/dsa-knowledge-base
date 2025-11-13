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

    #[test]
    fn test_swipe_data() {
        let output_artihmetic = "before : a = 10, b = 100 | after a = 100, b = 10";
        let output_bitwise = "before : a = 80, b = 78 | after a = 78, b = 80";
        assert_eq!(solution::swipe_data::arithmetic_swipe_data_fn(10, 100), output_artihmetic);
        assert_eq!(solution::swipe_data::arithmetic_swipe_data_fn(80, 78), output_bitwise);
    }

    #[test]
    fn test_close_number() {
        assert_eq!(solution::close_number::one_close_number_fn(13, 4), 12);
        assert_eq!(solution::close_number::two_close_number_fn(-15, 6), -18);
    }

    #[test]
    fn test_dice() {
        assert_eq!(solution::face_dice::face_dice_fn(4), 3);
        assert_eq!(solution::face_dice::face_dice_fn(6), 1);
        assert_eq!(solution::face_dice::face_dice_fn(5), 2);
    }

    #[test]
    fn test_arithmetic_series() {
        assert_eq!(solution::arithmetic_series::one_arithmetic_series(2, 3, 4), 5);
        assert_eq!(solution::arithmetic_series::two_arithmetic_series(2, 3, 4), 5);
    }
}
