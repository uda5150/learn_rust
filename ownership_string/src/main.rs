fn main() {
    let mut s = String::from("hello"); //String型直下のfrom関数を使用

    s.push_str(", world!");

    println!("{}", s);

}
