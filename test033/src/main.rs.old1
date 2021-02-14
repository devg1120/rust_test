/*
ライフタイム境界
Rust は型パラメータの境界にライフタイムを指定することができる
Rust では「ライフタイム境界」と呼んでいるようだ
(1) T: 'a
  T 内の全ての参照は 'a よりも長生きでなくてはならない
(2) T: Trait + 'a
  (1) に加えてTはTraitという名のトレイトを実装してなくてはならない
*/

#[macro_use]
extern crate rental;

pub struct  Parent {
    i:  i32,
}

impl Parent {
     fn add(&mut self) {
        self.i +=  1;
    }
}

pub struct Child<'a> {
    parent: &'a Parent,
}

impl<'a> Child<'a> {
    fn new(parent: &'a  Parent) -> Self {
        Child {
            parent
        }
    }
    fn get(&self) -> i32 {
        self.parent.i
    }
    //fn add(&mut self)  {
    //    self.parent.add();
    //}
}

// マクロを使ってMyRentalを定義する
rental! {
    mod my_rentals {
        use super::*;

        #[rental]
        pub struct MyRental {
            parent: Box<Parent>,
            child: Child<'parent>,
        }
    }
}

// MyRentalを返す関数も記述できる
fn func() -> my_rentals::MyRental {
    // ParentをBoxの中に作る
    let f = Box::new( Parent { i: 42 });
    // クロージャ内でParentからChildを生成する
    my_rentals::MyRental::new(f, |parent: &Parent| {
        Child::new(parent)
    })

}

fn main() {
    // MyRentalは普通にムーブ可能
    let r = func();

    // フィールドにはクロージャを介してアクセスする
    r.rent_all(|rent| {
        let child: &Child = &rent.child;
        assert_eq!(child.parent.i, 42);
        assert_eq!(child.get(), 42);
        //child.add();
        //assert_eq!(child.get(), 43);
    });
}
