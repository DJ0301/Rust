use std::fs::{self,File};
use std::io::ErrorKind;
use std::io::Read;
use std::io;
fn main() {
    // a();
    //This is the way to handle the error propogation in rust using expect
    // let f = File::open("hello.txt").expect("Failed to open the file"); 
    //This is the way to handle the error propogation in rust using unwrap or else
    // let f = File::open("hello.txt").unwrap_or_else( | error | {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             println!("File does not exist , created successfully");
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    //This is the way to handle the error propogation in rust using match
    // let f = File::open("hello.txt");
    // let f = match f { //match is used to handle the error propogation , shadowing the f variable
    //     Ok(file) => {println!("File opened successfully");
    //         file},
    //     Err(error) => match error.kind(){
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };
    match read_username_from_file() {
        Ok(s) => println!("The content of the file is : {} Dhananjay", s),
        Err(e) => println!("Error reading the file : {:?}", e),
    }

}
//This is the way to handle the error propogation in rust using ? operator , the ? operator can be used only in the function that returns Result or Option
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s) // Return the content of the file
}    //alternate way to write the above code
    // fs::read_to_string("hello.txt")


// fn a()
// {
//     b();
// }

// fn b()
// {
//     c();
// }

// fn c()
// {
//     d();
// }

// fn d()
// {
//     panic!("Crash and burn");
// } //run this with backtrace enabled , RUST_BACKTRACE=1 cargo run. It will show the backtrace of the error propogation.