pub fn match_fn() {
    let number: i32 = 2;

    let result: &str = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other"
    };
    println!("-----------------------------------------------------");
    println!("The result is: {}", result);
}