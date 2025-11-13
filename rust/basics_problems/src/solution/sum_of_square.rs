#[allow(unused)]
pub fn one_sum_of_square_fn(n: i32) -> i32 {
    let mut result = 0;
    for i in 1..n+1 {
        result += i * i;
    }
    result
}

#[allow(unused)]
pub fn two_sum_of_square_fn(n: i32) -> i32 {
    (n*(n + 1)/2)*(2*n+1)/3
}