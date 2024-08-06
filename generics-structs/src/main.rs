struct Point<T,U> {
    x: T,
    y: U,
}

impl Point<i32, f64> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

impl<T,U> Point<T,U> {
    fn mismatch<P,Q>(self , other: Point<P,Q>) -> Point<T,Q> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    println!("Hello, world!");
    let p1 = Point { x: 5, y: 10.4 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    let p2 = Point { x: "Hello", y: 'c' };
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
    let p3 = Point { x: 5, y: 10.4 };
    let xp3 = p3.x();
    println!("p3.x = {}", xp3);
    let p4 = p3.mismatch(p2);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
}
