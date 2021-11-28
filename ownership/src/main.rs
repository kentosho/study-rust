fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{}, world!", s2);
    // let s = String::from("hello");
    // takes_ownership(s);
    //
    // let s1 = s;

    let s1 = gives_ownership();
    let s2 = String::from("world!");
    let s3 = takes_and_gives_back(s2);
    let len = calculate_length(&s3);
    // let x = 5;
    // makes_copy(x);
    println!("{}, {}", s1, s3);
    println!("The length of '{}' is {}", s3, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
// fn takes_ownership(some_string: String) {
//    println!("{}", some_string);
// }
// fn makes_copy(some_integer: i32) {
//    println!("{}", some_integer);
//}
