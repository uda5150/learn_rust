pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// 関数呼び出しの短縮化
// use a::series::of;

// fn main() {
//     of::nested_modules()
// }

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}