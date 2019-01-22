fn main() {
    let celsius : f64 = -2.0;

    let fahrenheit : f64 = (celsius * 9.0 / 5.0 + 32.0).into();

    println!("華氏{}度のとき、摂氏は{}度", celsius, fahrenheit)

}
