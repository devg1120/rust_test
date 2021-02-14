//Option


/* std::option::Option
 *

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/

fn get_value_good(v: bool) -> Option<usize> {
    if v {
        Some(100)
    } else {
        None
    }
}
//------------------------------------------------------------ main
fn main() {

    match get_value_good(true) {
        Some(result) => println!("success: {}", result),
        None => println!("failure"),
    };
    match get_value_good(false) {
        Some(result) => println!("success: {}", result),
        None => println!("failure"),
    };

    //--------------------------------------
    // Some(v): 値がある場合
    // None: 値が無い場合
    let some_value: Option<usize> = Some(1);
    let none_value: Option<usize> = None;

    // Optionの中身はパターンマッチで取り出す
    match some_value {
        Some(v) => println!("some value = {}", v),
        None => println!("none value"),
    };

    // Someの場合だけ処理したいならif letの方が便利
    if let Some(v) = some_value {
        println!("some value = {}", v);
    }

    // 比較もできる
    // ただし、Option<T>のT型がEqやOrdトレイトを実装している必要がある
    assert!(some_value != none_value);

    // Option<T>には便利メソッドがいろいろある
    // Some(v) or Noneを判定
    assert!(some_value.is_some());
    assert!(none_value.is_none());

    // unwrap: (&self) -> T
    // Someの場合は中身を返し、Noneの場合はpanicして終了
    assert_eq!(some_value.unwrap(), 1);
    // assert_eq!(none_value.unwrap(), 1); // panicする

    // expect: (&self, &str) -> T
    // unwrapと同じだが、panicするときにメッセージを指定できる
    assert_eq!(some_value.expect("panic!"), 1);
    // assert_eq!(none_value.expect("panic!"), 1); // panicする

    // unwrap_or: (&self, T) -> T
    // unwrapと同じだが、Noneの場合にpanicする代わりに別の値を返す
    assert_eq!(some_value.unwrap_or(0), 1);
    assert_eq!(none_value.unwrap_or(0), 0);

    // unwrap_or_else: (&self, FnOnce() -> T) -> T
    // unwrap_orと同じだが、Noneの場合の値をClosureで指定する
    // 遅延評価したいときに使う
    assert_eq!(some_value.unwrap_or_else(|| 0), 1);
    assert_eq!(none_value.unwrap_or_else(|| 0), 0);

    // map: (&self, FnOnce(T) -> U) -> Option<U>
    // Someの場合は関数を適用し、Noneの場合はNoneまま返す
    assert_eq!(some_value.map(|v| format!("value is {}", v)), Some("value is 1".to_string()));
    assert_eq!(none_value.map(|v| format!("value is {}", v)), None);

    // map_or: (&self, U, FnOnce(T) -> U) -> U
    // unwrap_orのmap版
    assert_eq!(some_value.map_or("value is default".to_string(), |v| format!("value is {}", v)), "value is 1".to_string());
    assert_eq!(none_value.map_or("value is default".to_string(), |v| format!("value is {}", v)), "value is default".to_string());

    // map_or_else: (&self, FnOnce() -> U, FnOnce(T) -> U) -> U
    // unwrap_or_elseのmap版
    assert_eq!(some_value.map_or_else(|| "value is default".to_string(), |v| format!("value is {}", v)), "value is 1".to_string());
    assert_eq!(none_value.map_or_else(|| "value is default".to_string(), |v| format!("value is {}", v)), "value is default".to_string());

    // 他にもいろいろあります！
    // https://doc.rust-lang.org/std/option/enum.Option.html
}

