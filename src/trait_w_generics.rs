// Trait with Generics
trait Summable {
    fn sum(&self) -> i32;
}

struct Numbers(Vec<i32>);
struct String_s(String); // Lifetime parameter for the borrowed strings

impl Summable for Numbers {
    fn sum(&self) -> i32 {
        self.0.iter().sum() // บวกกันทั้งหมด
    }
}

// Function นี้รับเฉพาะตัวที่มีคุณสมบัติ Summable เท่านั้น
// T: Summable = **Trait Bound**
fn print_sum<T: Summable>(item: T) {
    println!("The sum is: {}", item.sum());
}

pub fn trait_fn() {

    println!("----------------------------------> Trait with Generics");
    let nums = Numbers(vec![1, 2, 3, 4, 5]);
    let str_eiei = String_s(String::from("Hello, world!"));
    print_sum(nums);

    // Example error below
    // print_sum(str_eiei);
    }


// Associated Types
// จากเดิมจะเป็นการ fix type เลยใน trait
// สามารถกำหนด type ที่ใช้ใน Triat ได้จาก implementer
// เพิ่มความยืดหยุ่นของ Trait
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
} 
impl Iterator for Counter {
    type Item = u32; // กำหนด Associated Type

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn trait_associated_fn() {

    println!("----------------------------------> Trait with Associate type");
    let mut counter = Counter { count: 0 };

    while let Some(value) = counter.next() {
        println!("{}", value);
    }
}