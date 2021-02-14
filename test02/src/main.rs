//ジェネリクス

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}

#[derive(Debug)]
struct Point2<T,U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 { x: self.x, y: other.y }
    }
}
//------------------------------------------------------------ main
fn main() {

    let integer = Point { x: 5, y: 10 }; // i32型
    let float = Point { x: 1.0, y: 4.0 }; // f64型
    println!("{:?}",integer);
    println!("{:?}",float);

    let integer2 = Point2 { x: 5, y: 10 }; // i32型
    let float2 = Point2 { x: 1.0, y: 4.0 }; // f64型
    let int_f2 = Point2 { x: 5, y: 4.0 }; // i32,f64型
    println!("{:?}",integer2);
    println!("{:?}",float2);
    println!("{:?}",int_f2);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("{:?}", p3); // { x: 5, y: 'c' }

}

