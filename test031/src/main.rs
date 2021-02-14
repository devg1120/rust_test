
//リスト : ライフタイムパラメータの指定

// 配列の探索
//fn find<'a, 'b, T: PartialEq>(item: &'b T, xs: &'a [T]) -> Option<&'a T> {
fn find<'a, 'b, T: PartialEq>(item: &'a T, xs: &'b [T]) -> Option<&'b T> {
    for x in xs {
        if x == item { return Some(x); }
    }
    None
}

//構造体のライフタイム
struct Foo<'a> {
        x: &'a i32
}

//struct Foo {
//        x: & i32   //<Err> xpected named lifetime parameter
//}

//メソッドのライフタイム
impl <'a> Foo<'a> {
        //メソッド foo() の返り値のライフタイムは引数と同じになる
        fn foo(&self) ->&i32 { self.x }

        //メソッド foo1()
        //のように、複数の引数があってライフタイムパラメータの指定がない場合、返り値のライフタイムは
        //self と同じになる
        fn foo1(&self, y: &i32) ->&i32 {
                      println!("{},{}", self.x, y);
                               self.x
                      }    

        //メソッド foo2() のように、self
        //と異なるライフタイムを指定したい場合はライフタイムパラメータを明示する
        fn foo2<'b>(&self, y: &'b i32) ->&'b i32 {
                      println!("{},{}", self.x, y);
                               y
                      }    
}

fn main() {
    let xs = [1,2,3,4,5,6,7,8];
    for x in 0 .. 10 {
        match find(&x, &xs) {
            Some(v) => println!("{}", v),
            None => println!("None")
        }
    }

    let y = 123;
    let z = Foo { x: &y };
    println!("{}", z.x);

    //let z1;  コンパイルエラー
    // {
    //     let y1 = 456;
    //     z1 = Foo { x: &y1 };
    // }
    //println!("{}", z1.x);

    let y = 123;
    let z = Foo { x: &y };
    println!("{}", z.foo());
    let y1 = 456;
    println!("{}", z.foo1(&y1));
    println!("{}", z.foo2(&y1));

}
