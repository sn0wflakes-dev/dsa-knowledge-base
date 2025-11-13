#[allow(unused)]
pub fn table_multiple_fn(n: i32) -> String{
    let mut result = String::new(); 
    for i in 1..11 {
        result.push_str(&format!("{0} x {1} = {2} \n", n, i, n * i));
    }
    println!("{}", result);
    result
}
