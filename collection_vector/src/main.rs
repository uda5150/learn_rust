

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100];
    let does_not_exist1 = v.get(100);

    // println!("{:?}", third);
    // println!("{:?}", does_not_exist);
    println!("{:?}", does_not_exist1);
}
