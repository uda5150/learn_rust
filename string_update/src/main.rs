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
    // addの定義がseldの所有権を奪うかららしい
    // s3はここで破棄される
    let s5 = s3 + &s4; 
    // println!("{}", s3);
    println!("{}", s4);
    println!("{}", s5);

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");

    let s9 = s6 + "-" + &s7 + "-" + &s8;

    println!("{}", s9);

    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");

    let s13 = format!("{}-{}-{}", s10, s11, s12);

    println!("{}", s13);
}
