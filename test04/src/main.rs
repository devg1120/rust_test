//===================================================
trait SayHello : std::fmt::Display {
    fn say_hello(&self) {
        println!("Hello. This is {}.", self);
    }
}

impl SayHello for i32 {}
impl SayHello for &str {}
impl SayHello for str {
    fn say_hello(&self) {
        println!("Hi. I'm {}.", self);
    }
}

//--------------------------------------
trait SayGoodbye : std::fmt::Display {
    fn say_goodbye(&self) {
        println!("Goodby {}.", self);
    }
}

impl SayGoodbye for i32 {}
impl SayGoodbye for &str {}
impl SayGoodbye for str {
    fn say_goodbye(&self) {
        println!("Goodby {}.", self);
    }
}

//--------------------------------------
//ジェネリックの型変数がトレイトを実装していることを表したいときは、
//トレイト制約を使う。
//
fn greeting<T: SayHello>(x: T) {  //トレイト制約
    x.say_hello();
}

fn greeting2<T: SayHello + SayGoodbye>(x: T) {
    x.say_hello();
    x.say_goodbye();
}

fn greeting3<T>(x: T) where T: SayHello + SayGoodbye {
    x.say_hello();
    x.say_goodbye();
}
//===================================================

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
}
impl Summary for NewsArticle {
   fn summarize(&self) -> String {
       format!("{}, by {} ({})", self.headline, self.author, self.location)
   }
}


//トレイト境界
//　ジェネリックな型のうち、特定の型をもつものだけに限定する。

fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
}

//------------------------------------------------------------ main
fn main() {
    42.say_hello();
    "Alice".say_hello();
    let s = "TEST";
    s.say_hello();
    &s.say_hello();


    greeting(100);
    greeting("Bob");

    greeting2(101);
    greeting2("Bob");

    greeting3(102);
    greeting3("Bob");

    let na = NewsArticle {
            headline: "head".to_string(),
            location: "loc".to_string(),
            author: "auth".to_string(),
            content: "cont".to_string(),
    };
    println!("{}",na.summarize());
    notify(na);

}

