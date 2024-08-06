// Structs are a way to create custom types. They are useful when you want to create a type that is more complex than a simple integer or string.

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { //This is a method of the struct that takes a reference to the struct as the first parameter
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool { //This is also a method of the struct that takes a reference to the struct as the first parameter
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle { //This is an associated function of the struct that does not take a reference to the struct as the first parameter
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("abc@xyz.com"),
        username: String::from("abc"),
        sign_in_count: 1,
        active: true,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rect1 is {} square pixels.", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::square(60);
    println!("rect3 is {:?}", rect3);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Area of rect3 is {}", rect3.area());
    let name = user1.username;
    println!("user1 name : {}", name);
    let user2 = build_user(String::from("def@xyz.com"), String::from("def"));
    println!("user2 email : {}", user2.email);
    println!("user2 active status : {}", user2.active);
    let user3 = User {
        email: String::from("dj@xyz.com"),
        username: String::from("dj"),
        ..user2
    };
    println!("user3 email : {}", user3.email);  

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black color : {}, {}, {}", black.0, black.1, black.2);
    struct Point {
        x: i32,
        y: i32,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}