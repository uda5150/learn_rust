fn main() {
    // let mut s = String::new();
    // println!("{}", s);

    // 文字列リテラルからStringの生成
    let data = "initial contents";
    // let s = data.to_string();
    let s = String::from(data);
    println!("{}", s);
}
