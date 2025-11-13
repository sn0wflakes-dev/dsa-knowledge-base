#[allow(unused)]
pub fn is_even_odd_fn(number: i32) -> String {
    let is_odd: &str = if number % 2 == 1 { "is odd" } else { "is even" };
    return String::from(is_odd);
}
