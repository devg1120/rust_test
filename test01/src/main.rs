fn print1(s: &str) {
    println!("print1:{}", s);
}

/*
fn print2(s: str) {               //<Err> the size for values of type `str` cannot be known at compilation time
    println!("print2:{}", &s);
}
*/

fn print3(s: &String) {
    println!("print3:{}", s);
}

fn print4(s: &String) {
    println!("print4:{}", s);
}

fn print5(s: &String) {
    println!("print5:{}", s);
}

fn print6(s: &String) {
    println!("print6:{}", s);
}

fn str_push(s: &mut String) {  //関数 &mut  更新
    s.push_str(", world!");
    println!("str_push:{}", *s);
    println!("str_push:{}", s);
    println!("str_push:{}", &s);
}

fn str_add(s1: &mut String, s2: &String) {  
      s1.push_str(s2.as_str());
     
}

//fn str_longer(s1: &String, s2: &String) -> &String{              /* expected named lifetime parameter */
fn str_longer<'a>(s1: &'a String, s2: &'a String) -> &'a String{

    if s1.len() > s2.len() {
         s1
    } else {
        s2
   }
     
}

//------------------------------------------------------------ main
fn main() {
    println!("Hello, world!");
                                                                //Move
    let a = String::from("AAA"); // ヒープ領域のメモリ確保
    let b = a; // ムーブ（ヒープ領域の所有権がポインタ変数`a`から`b`へ移った。ヒープ領域の所有は必ず1つのポインタ変数のみ）
    println!("b = {}", b); // OK
    //println!("a = {}", a); // エラー

    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //可変参照は1つのみ
    //r2.push_str("_k");
    r1.push_str("_m");
    println!("{}", s);
    
    let mut s2 = String::from("hello");
    {
         let r1 = &mut s2;
    } // r1はここでスコープを抜けて無効化（メモリ解放）されるので、以降はsの可変参照を作れる
    let r2 = &mut s2;

    let mut s3 = String::from("hello");
    let r1 = &s3; // OK
    let r2 = &s3; // OK
    //let r3 = &mut s3; // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //不変で借用されているのに可変で借用しようとしたらエラー

//-- hard code
    let s1: &str = "hello";   
    print1(&s1);
    //print2(s1);                    //<Err>

//-- String heep
    let s2: String = String::from("hello");
    print3(&s2);

    let mut s3 = String::from("hello");
    print3(&s3);
    s3.push_str(", world!"); 
    print3(&s3);

    let mut s4 = String::from("hello");   //関数 str_push
    str_push(&mut s4);
    print4(&s4);

    let mut s5 = String::from("hello");  // str_add
    let s6 = String::from(", world!");
    str_add(&mut s5 , &s6);
    print5(&s5);

    let s7 = String::from("hello");     // str_longer
    let s8 = String::from(", world!");
    let s9 = str_longer(&s7 , &s8);
    print6(&s9);

    let s7 = String::from("hello");     // str_longer
    let s8 = String::from(", world!");
    {
     let s9 = str_longer(&s7 , &s8);
     print6(&s9);
    }


}
