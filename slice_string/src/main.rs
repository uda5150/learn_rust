fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // 0〜4まで
    let world = &s[6..11]; // 6〜10まで
    let slice = &s[..2]; // 0〜1

    let len = s.len();

    let xxx = &s[3..len];
    let yyy = &s[3..];

    let al = &s[0..len];
    let all = &s[..];
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}