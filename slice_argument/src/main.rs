fn main() {

    let my_string = String::from("hello world");

    // Stringのスライスに対して動く
    let word = first_word(&my_string[..]);

    // 文字列リテラルのスライスに対して動く
    let my_string_literal = "hello world";
    let word_1 = first_word(&my_string_literal[..]);

    // 文字列リテラルは、すでに文字列スライスなので、
    // スライス記法無しでも動く
    let word_2 = first_word(my_string_literal);
}

// Stringから&strにすると、String値と&str両方に使える
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}