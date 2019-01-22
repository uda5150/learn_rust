fn main() {
    let number = 40;
    println!("{}",fibonacci(number));
}


fn fibonacci(x: i32) -> i32 {

    if x == 1 {
        0
    } else if x == 2 {
        1
    } else {
        fibonacci(x-2) + fibonacci(x-1)
    }
}