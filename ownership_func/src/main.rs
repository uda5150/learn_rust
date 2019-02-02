fn main() {
    let s = String::from("hello");  

    takes_ownership(s);  // sの値がmoveされる
    // ここでは、sがstringのため、もうsはdropされている

    let x = 5;

    makes_copy(x);
    // xはmoveされているが、i32がcopyなのでxは呼び出せる
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

