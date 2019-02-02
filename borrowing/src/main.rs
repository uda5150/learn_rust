fn main() {
    println!("Hello, world!");
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &記号が参照で所有権をもらうことなく、値を参照できる
    
    println!("The length of '{}' is {}", s1, len);
}

// sはs1を参照しているだけ
fn calculate_length(s: &String) -> usize {
    s.len()
}