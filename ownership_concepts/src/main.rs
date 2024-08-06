fn main() {
    let slice = "hello";
    let mut s = String::from(slice);
    let s2 = s.clone();
    let s2slice = &s2; //slices do not take ownership of the data they point to
    println!("This is a sliced string {}", s2slice);
    println!("{}", slice);
    println!("{}", s);
    does_not_take_ownership(&s);
    let r = &mut s;
    does_not_take_ownership_mut(&mut s); // s is borrowed mutably
    let s1 = s; // s1 takes ownership of s , s scope ends here
    // println!("{}", s); // This will throw an error because s has been moved to s1
    takes_ownership(s1); //s1 scope ends here
    // println!("{}", s1); // This will throw an error because s1 has been moved to takes_ownership
    // let reference_to_nothing = dangle(); // This will throw an error because the reference is to a variable that has gone out of scope 
}

fn does_not_take_ownership(some_string: &str) {
    println!("Does not take ownership {}", some_string);
}

fn does_not_take_ownership_mut(some_string: &mut str) {
    println!("Does not take ownership mut {}", some_string);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope here and is dropped. Its memory goes away. Danger!