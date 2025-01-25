pub fn loop_fn() {

    println!("----------------------------------> Loop");
    let mut c = 0;

    loop {
        c += 1;
        println!("Count is: {}", c);

        if c == 5 {
            break;
        }
    }
    println!("Loop finish!")
}

pub fn while_fn() {
    
    println!("----------------------------------> While loop");
    let mut number = 3;

    while number != 0 {
        println!("Counting down: {}", number);
        number -= 1;
    }
    println!("Liftoff!")
}

pub fn for_fn() {

    println!("----------------------------------> For loop");
    for num in 1..=5 {
        println!("For loop: {}", num);
    }
    println!("Done!");
}