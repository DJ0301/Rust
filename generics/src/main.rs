use std::fmt::Display;

fn main() {

    let var_name = vec![34, 50, 25, 100, 65, 6000];
    let number_list = var_name;
    let char_name = vec!['y', 'm', 'a', 'q' , '&' , 'z'];
    let char_list = char_name;
    // let largest = find_largest_number(&number_list);

    // println!("The largest number is {}", largest.unwrap(_or_else(|| {
    //     println!("The list is empty");
    //     &0
    // }));)

    let ans = largest(&number_list);
    let char_ans = largest(&char_list);
    println!("The largest char is {}", char_ans);
    println!("The largest number is {}", ans);

}

fn largest<T: PartialOrd + Copy>(number_list: &[T]) -> T { // PartialOrd is a trait that is used to compare values , Copy is used to copy the value
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
fn find_largest_number(number_list: &[i32]) -> Option<&i32> {
    let mut largest = number_list.get(0)?;

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    Some(largest)
}
