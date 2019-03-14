fn main() {
    let len = String::from("Hola").len();

    println!("{}", len);

    // UTF8の場合は、2byteなので、2個分アクセスしないといけない
    let hello = "Здравствуйте";
    let answer = &hello[0..4];

    println!("{}", answer);

    for c in "नमस्ते ".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते ".bytes() {
        println!("{}", b);
    }

    let xxx = "おはよう";
    let yyy = &xxx[0..3];

    println!("{}", yyy);
}
