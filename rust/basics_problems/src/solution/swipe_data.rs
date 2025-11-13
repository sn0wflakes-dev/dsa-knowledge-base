#[allow(unused)]
pub fn arithmetic_swipe_data_fn(a: i32, b: i32) -> String {
    let mut result: String = String::new();
    let mut data_a: i32 = a;
    let mut data_b: i32 = b;

    data_a = data_a + data_b;
    data_b = data_a - data_b;
    data_a = data_a - data_b;

    result.push_str(&format!("before : a = {0}, b = {1} | after a = {2}, b = {3}", a, b, data_a, data_b));
    println!( "{}", result);
    return result;
}

#[allow(unused)]
pub fn bitwise_swipe_data(a: i32, b: i32) -> String {
    let mut result: String = String::new();
    let mut data_a: i32 = a;
    let mut data_b: i32 = b;

    data_a = data_a ^ data_b;
    data_b = data_a ^ data_b;
    data_a = data_a ^ data_b;


    result.push_str(&format!("before : a = {0}, b = {1} | after a = {2}, b = {3}", a, b, data_a, data_b));
    println!( "{}", result);
    return result;
}