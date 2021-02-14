/*
ライフタイム境界
Rust は型パラメータの境界にライフタイムを指定することができる
Rust では「ライフタイム境界」と呼んでいるようだ
(1) T: 'a
  T 内の全ての参照は 'a よりも長生きでなくてはならない
(2) T: Trait + 'a
  (1) に加えてTはTraitという名のトレイトを実装してなくてはならない
*/

struct Foo<'a, T: 'a> {
    value: &'a T
}

impl<'a, T> Foo<'a, T> {
    fn new(val: &'a T) -> Foo<'a, T> {
        Foo { value: val }
    }
}

fn main() {
    let x = 10;
    let a = Foo::new(&x);
    println!("{}", a.value);

    //let b: Foo<i32>;
    //{
    //    let y = 20;  コンパイルエラー `y` does not live long enough
    //    b = Foo::new(&y);
    //}
}

/*
 構造体 Foo のフィールド value は参照を格納するのでライフタイムパラメータ 'a の指定が必要
value の参照先 (T の値) が Foo よりも先に解放されてはいけない
つまり、T の参照 (&T) のライフタイムは Foo よりも短くなることはない
この条件を T の境界にライフタイムパラメータ 'a を指定することで表す
Foo にライフタイムパラメータが必要なので、impl でもライフタイムパラメータを指定する
変数 b は変数 y よりもライフタイムが長い
したがって、y の参照を格納した Foo を変数 b にセットすることはできない (コンパイルエラー)
*/
