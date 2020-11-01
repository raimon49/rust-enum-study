use std::cmp::Ordering;
use std::cmp::Ordering::*;
// 以下の定義を持つEnumとそのvariantを全てインポート
/*
enum Ordering {
    Less,
    Equal,
    Greater
}
*/
use self::Pet::*; // 現在のモジュールで宣言されているvariantを全てインポートするにはselfインポートを使う
#[derive(PartialEq)]
enum Pet {
    Orca,
    Giraffe
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

fn main() {
    let _result = compare(3, 4);

    if Orca != Giraffe {
        println!("Orca is not Giraffe");
    }
}
