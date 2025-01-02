pub fn loop_fn() {
    let mut c = 0;
    
    println!("-----------------------------------------------------");
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
    let mut number = 3;

    println!("-----------------------------------------------------");
    while number != 0 {
        println!("Counting down: {}", number);
        number -= 1;
    }
    println!("Liftoff!")
}

pub fn for_fn() {
    println!("-----------------------------------------------------");
    for num in 1..=5 {
        println!("For loop: {}", num);
    }
    println!("Done!");
}