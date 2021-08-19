// enum Option<T> {
//     None, // The value doesn't exist
//     Some(T), // The value exists
// }

fn main() {
    let fruits = vec!["banana", "apple", "coconut", "kiwi"];

    // pick first item
    let first = fruits.get(0);
    println!("{:?}", first); // Some<&str>

    // pick third item
    let third = fruits.get(2);
    println!("{:?}", third); // Some<&str>

    // pick non-existent item
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent); // None

    // how to access data in Some<T>?
    for &index in [0, 2, 99].iter() {
        // Use match!
        match fruits.get(index) {
            Some(fruit_name) => println!("{}", fruit_name),
            None => println!("There is no fruit!")
        }
    }

    // You can materialization match expression
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"banana") => println!("I like banana!"),
            Some(fruit_name) => println!("{}", fruit_name),
            None => println!("There is no fruit!")
        }
    }

    // if let
    let a_number: Option<i32> = Some(42);
    
    match a_number {
        Some(42) => println!("The answer to life the universe and everything"),
        _ => {}, // underbar is wildcard pattern
    }

    // instead above match expression
    if let Some(42) = a_number {
        println!("The answer to life the universe and everything")
    }

    // You can use access data in option use unwrap
    // Caution: if you access to None, method go to panic.
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    // let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "candy"); // this will panic

    // you can use pattern matching or unwrap_or
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat"); // It's not panic
}
