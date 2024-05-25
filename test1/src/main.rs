fn main() {
    let mut vec: Vec<i32> = (1..5).collect();
    vec.push(6);
    let mut sum = 0;
    for i in vec.iter() {
        sum += i;
    }
    println!("Sum: {}", sum);
}
