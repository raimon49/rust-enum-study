use std::cmp::Ordering;
use std::cmp::Ordering::*;
// 以下の定義を持つEnumを全てインポート
/*
enum Ordering {
    Less,
    Equal,
    Greater
}
*/

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
}
