
// Gernerics
// T as a Generics Type Parameter 
fn print_two_values<T: std::fmt::Debug>(value1: T, value2: T) {
    println!("first: {:?}", value1);
    println!("second: {:?}", value2);
    
}

pub fn generics_fn() {

    println!("----------------------------------> Generics");
    print_two_values(55, 45);
    print_two_values(1.2, 1.8);
}

// Gernerics w Struct
struct Point<T> {
    x:T,
    y:T
}

pub fn generics_w_s() {

    println!("----------------------------------> Generics with Struct");
    let point_int = Point{ x: 10, y:20};
    let point_float = Point { x: 1.5, y: 1.5};

    println!("Point (i32): ({}, {})", point_int.x, point_int.y);
    println!("Point (f64): ({}, {})", point_float.x, point_float.y);
}

// Gernerics w Enum
enum Option<T> {
    Some(T),
    None
}

pub fn generics_w_e() {

    println!("----------------------------------> Generics with Enum");
    let some_number= Option::Some(50);
    let no_value: Option<i32> = Option::None;

    match some_number {
        Option::Some(value) => println!("Value: {}", value),
        Option::None => println!("No value")
    }
}