
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 { rect.width * rect.height }


#[derive(Debug)]
enum IpAddrKind {
        V4,
        V6,
}

#[derive(Debug)]
enum IpAddrDemo {
       V4(u8,u8,u8,u8),
       V6(String),
}

// メッセージをenumで定義する
#[derive(PartialOrd, PartialEq, Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
   }

// enumにメソッドを実装する
impl Message {
        fn call(&self) {
                    println!("call {:?}", self);
        }
}

#[derive(Debug)]
enum UsState {
        Alabama,
            Alaska,
}
enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
        match coin {
               Coin::Penny => 1,
               Coin::Nickel => 5,
               Coin::Dime => 10,
               Coin::Quarter(state) => { println!("{:?}", state); 25 },
      }
}
//------------------------------------------------------------ main
fn main() {

    let rect = Rectangle { width: 3, height: 4 };
    println!("w: {} h: {} area: {}", rect.width, rect.height, area(&rect));
    println!("rect: {:?}", rect);
    println!("rect: {:#?}", rect);

    println!("{:?}", IpAddrKind::V4);
    println!("{:?}", IpAddrKind::V6);


    println!("{:?}", IpAddrDemo::V4(127, 0, 0, 1));
    println!("{:?}", IpAddrDemo::V6(String::from("::1")));


    println!("{}", Message::Move{x:0,y:0} == Message::Move{x:0,y:0});
    println!("{}", Message::Move{x:0,y:0} == Message::Move{x:1,y:1});
    println!("{}", Message::Move{x:0,y:0} == Message::Quit);
    use std::mem;
    println!("{}", mem::discriminant(&Message::Move{x:0,y:0}) == mem::discriminant(&Message::Move{x:0,y:0}));
    println!("{}", mem::discriminant(&Message::Move{x:0,y:0}) == mem::discriminant(&Message::Move{x:1,y:1}));
    println!("{}", mem::discriminant(&Message::Move{x:0,y:0}) == mem::discriminant(&Message::Quit));

    let m = Message::Move{x:0,y:0};
    m.call();
    Message::Move{x:1,y:1}.call();
    Message::Quit.call();
    Message::Write(String::from("hello")).call();
    Message::ChangeColor(255,0,0).call();

    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

}

