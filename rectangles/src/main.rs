
// 構造体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 借用
// 所有権を保って、rect1を使用し続けるために関数シグネチャと関数呼び出し時に&を使っている

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "rect1 is {:#?}", rect1
    );

    println!(
        "The area of the rectangles is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}