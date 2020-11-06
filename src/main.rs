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

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None
    }
}

// enumはコンパイラが自動で0から順に値を決めるが、自分で任意の整数を指定も可能
#[derive(PartialEq, Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404
}

fn main() {
    let _result = compare(3, 4);

    if Orca != Giraffe {
        println!("Orca is not Giraffe");
    }

    use std::mem::size_of;
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2); // 自分で割り当てた404はu8に収まらないため2バイト使う
    assert_eq!(HttpStatus::Ok as i32, 200); // 列挙型から整数型へのキャストはできるが逆方向はできない
    assert_eq!(HttpStatus::NotModified, http_status_from_u32(304).unwrap()); // よって、自分でチェックして整数型->列挙型へ変換する
}
