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

#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

// enumもstructと同様にメソッドを実装できる
impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years"
        }
    }

    fn singlar(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

// タプル型variantを持つ列挙型
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32)
}

fn rough_time_to_english(rt: RoughTime) -> String {
    // 列挙型フィールドのデータに安全にアクセスするにはパターンマッチを用いる
    match rt {
        RoughTime::InThePast(units, count) =>
            format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow =>
            format!("Just now"),
        RoughTime::InTheFuture(units, count) =>
            format!("{} {} from now", count, units.plural())
    }
}

use std::collections::HashMap;
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>) // Box<HashMap>はヒープ上で取られたデータへのポインタのためJsonを小さくできる
}

// ジェネリック列挙型 + ジェネリック構造体で任意個数の型Tを保持するBinaryTreeを定義
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
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

    assert_eq!(TimeUnit::singlar(TimeUnit::Seconds), "second");
    assert_eq!(TimeUnit::singlar(TimeUnit::Minutes), "minute");
    assert_eq!(TimeUnit::singlar(TimeUnit::Hours), "hour");
    assert_eq!(TimeUnit::singlar(TimeUnit::Days), "day");
    assert_eq!(TimeUnit::singlar(TimeUnit::Months), "month");
    assert_eq!(TimeUnit::singlar(TimeUnit::Years), "year");

    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4*20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    assert_ne!(four_score_and_seven_years_ago, RoughTime::JustNow);
    assert_ne!(three_hours_from_now, RoughTime::JustNow);

    use self::BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty
    }));
    let _mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: Empty
    }));
}
