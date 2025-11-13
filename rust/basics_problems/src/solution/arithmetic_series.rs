#[allow(unused)]
pub fn one_arithmetic_series(a1: i32, a2: i32, n: i32) -> i32 {
    let difference: i32 = a2 - a1;
    let mut result: i32 = a1;
    for i in 1..n {
        result += difference;
    }
    result
}

#[allow(unused)]
pub fn two_arithmetic_series(a1: i32, a2: i32, n: i32) -> i32 {
    a1 + (n - 1) * (a2 - a1)
}