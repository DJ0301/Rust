#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4(String::from("abc"));
    let six = IpAddrKind::V6(String::from("::1"));
    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));
    route(four);
    route(six);
    route(localhost);
    let some_number : Option<i32> = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let sum = some_number.unwrap_or(0) + absent_number.unwrap_or(0);
    println!("{:?}", sum);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(String) => {
            println!("V4  {:?}", String)
        },
        IpAddrKind::V6(String) => println!("V6"),
       
    }
}
