
// 構造体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle構造体にareaメソッドを定義した
// impl(implementations):実装
// 所有権はいらず、構造体のデータを読み込みたいだけなので、&selfを使っている
// メソッドの一部でメソッドを呼び出したインスタンスを変更したい場合は、
// 第1引数に&mut selfを使用する
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// 借用
// 所有権を保って、rect1を使用し続けるために関数シグネチャと関数呼び出し時に&を使っている
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "rect1 is {:#?}", rect1
    );

    println!(
        "The area of the rectangles is {} square pixels.",
        rect1.area()
    );

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}