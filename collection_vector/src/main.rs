

fn main() {
    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100];
    // let does_not_exist1 = v.get(100);

    // println!("{:?}", third);
    // println!("{:?}", does_not_exist);
    // println!("{:?}", does_not_exist1);


    // p155
    // コンパイラがうまく処理しているのか、
    // 宣言しただけではコンパイルされないっぽい
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("{:?}", first);
    // println!("{:?}", v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
     println!("{:?}", v);
}
