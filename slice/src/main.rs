fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // sは参照として渡される？

    s.clear();

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Stringを要素ごとにみるために変換

    // iterはコレクション内の各要素を返すメソッド
    // enumerateがiterの結果を包んでタプルの一部として、各要素を返す
    // enumerateが返すのは、添字とコレクションへの参照
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

