fn main() {

    let s1 = gives_ownership(); // s1に戻り値をmoveする

    let s2 = String::from("hello"); // s2がスコープに入る

    let s3 = takes_and_gives_back(s2); // s2はtakes__にmoveし、takes__の返り値がs3にmoveする


    let x1 = String::from("hello");

    let (x2, len) = calculate_length(x1);

    println!("The length fo '{}' is {}.", x2, len);
}


fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返す

    (s, length)
}

