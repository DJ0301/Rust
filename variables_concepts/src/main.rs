use std::any::type_name;


fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let a = 5;
    let b : f32 = 10.020;
    let mut c : u8 = 0;
    c = c.wrapping_sub(1);

    let tup = (500, 6, 1);
    let (mut x, y, z) = tup;
    let a1 = [1, 2, 3, 4, 5];
    println!("{}",x+y+z);
    println!("The value of c is: {}", c);
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The type of a is: {}", type_of(&a));
    println!("The type of b is: {}", type_of(&b));
    // println!("{}",another_function(a, c as i32));
    let mut counter = 1;
    // let result = loop {
        
    //     counter += 1;
    //     if counter == 10 {
    //         break counter;
    //     }
    // };
    // println!("The result is: {}", result);
    for val in a1.iter() {
        println!("The value is: {}", val);
    }
    

}

fn another_function(x: i32 , y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("Another function.");
    x+y
}