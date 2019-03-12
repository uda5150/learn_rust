fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_strはs2の所有権を奪わない
    s1.push_str(s2);

    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    println!("{}", s);

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");

    // s3が借用できないのは、型のせいか？
    // s3はここで破棄される
    let s5 = s3 + &s4; 
    // println!("{}", s3);
    println!("{}", s4);
    println!("{}", s5);
}
