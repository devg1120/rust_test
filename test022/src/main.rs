//Result


/* std::Result
 *

 Result<T,E>は失敗するかもしれない処理の結果を表現する列挙型です。

enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/

fn get_value_good(v: bool) -> Result<usize,&'static str> {
    if v {
        Ok(100)
    } else {
        Err("error message")
    }
}

//------------------------------------------------------------ main
fn main() {
    match get_value_good(true) {
        Ok(result) => println!("success: {}", result),
        Err(msg) => println!("failure: {}", msg),
    }

    //----------------------------------------------
    // Ok(v): 処理に成功した場合。返す値が無い場合はUnit型()を返すとよい
    // Err(e): 処理に失敗した場合。
    let ok_value: Result<usize, &'static str> = Ok(1);
    let er_value: Result<usize, &'static str> = Err("error message");

    // Resultの中身はパターンマッチで取り出す
    match ok_value {
        Ok(v) => println!("ok value = {}", v),
        Err(e) => println!("err value = {}", e),
    };

    // どちらか片方の場合だけ処理したいならif letが便利
    if let Ok(v) = ok_value {
        println!("ok value = {}", v);
    }
    if let Err(e) = er_value {
        println!("err value = {}", e);
    }

    // 比較もできる
    // ただし、Reulst<T,E>のTとE両方がEqやOrdトレイトを実装している必要がある
    assert!(ok_value != er_value);

    // Result<T,E>には便利メソッドがいろいろある
    // Ok(v) or Err(e)を判定
    assert!(ok_value.is_ok());
    assert!(er_value.is_err());

    // ok: (&self)
    // Option型に変換する。Okの場合はSomeに、Errの場合はNoneになる
    assert_eq!(ok_value.ok(), Some(1));
    assert_eq!(er_value.ok(), None);

    // err: (&self)
    // okメソッドの逆バージョン。Okの場合はNoneに、Errの場合はSomeになる
    assert_eq!(ok_value.err(), None);
    assert_eq!(er_value.err(), Some("error message"));

    // unwrap<T,E>: (&self) -> T
    // Okの場合は中身を返し、Errの場合はpanicする
    assert_eq!(ok_value.unwrap(), 1);
    // assert_eq!(er_value.unwrap(), 1); // panic

    // expect<T,E>: (&self, &str) -> T
    // unwrapと同じだが、panic時のエラーメッセージを指定できる
    assert_eq!(ok_value.expect("panic"), 1);
    // assert_eq!(er_value.expect("panic"), 1); // panic

    // unwrap_or<T,E>: (&self, T) -> T
    // unwrapと同じだが、panicする代わりに別の値を返す
    assert_eq!(ok_value.unwrap_or(0), 1);
    assert_eq!(er_value.unwrap_or(0), 0);

    // unwrap_or_else<T,E>: (&self, FnOnce(E) -> T) -> T
    // unwrap_orと同じだが、Errの場合の値をClosureで指定する
    // 遅延評価したいときに使う
    // Optionと違ってErrの値を引数に取るので注意
    assert_eq!(ok_value.unwrap_or_else(|_e| 0), 1);
    assert_eq!(er_value.unwrap_or_else(|_e| 0), 0);

    // unwrap_err<T,E>: (&self) -> E
    // expect_err<T,E>: (&self, &str) -> E
    // unwrap, expectのokとerrを逆にしたバージョン。okだとpanicする
    // assert_eq!(ok_value.unwrap_err(), "error message"); // panic
    assert_eq!(er_value.unwrap_err(), "error message");
    // assert_eq!(ok_value.expect_err("panic"), "error message"); // panic
    assert_eq!(er_value.expect_err("panic"), "error message");

    // map<T,E,U>: (&self, FnOnce(T) -> U) -> Result<U,E>
    // map_err<T,E,U>: (&self, FnOnce(E) -> U) -> Result<T,U>
    // Resultの中身を関数を適用してmapする
    // Okの場合に適用するmapとErrの場合に適用するmap_errがある
    assert_eq!(ok_value.map(|v| format!("{}", v)), Ok("1".to_string()));
    assert_eq!(er_value.map(|v| format!("{}", v)), Err("error message"));
    assert_eq!(ok_value.map_err(|s| s.len()), Ok(1));
    assert_eq!(er_value.map_err(|s| s.len()), Err(13));

}

