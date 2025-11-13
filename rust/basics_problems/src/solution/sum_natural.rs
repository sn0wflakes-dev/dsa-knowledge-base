#[allow(unused)]
pub fn sum_natural_fn(n: i32) -> i32 {
    let mut result = 0;
    for i in 1..n+1 {
        result += i;
    }
    println!("{}", result);
    result
}