fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v1[2];
    // v1.push(6); // error: cannot borrow `v1` as mutable because it is also borrowed as immutable
    println!("{:?}", third);

}
