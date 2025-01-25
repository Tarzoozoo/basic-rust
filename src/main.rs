mod data_type;
mod control_flow;
mod loop_;
mod ownership;
mod common_collection;
mod generics;
mod trait_interface;
mod trait_w_generics;
mod memory;
mod smart_pointers;
mod thread;
mod thread_x_smartptr;
mod asyncro;

fn main() {

    println!("#################################### Common Programming Concept ####################################");
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++ Data Type");
    data_type::scalar_type();
    data_type::compound_type();
    data_type::string_type();
    data_type::slice_type();
    data_type::vector_type();
    
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++ Function");
    let result = data_type::sqare(5, 10);
    println!("Result of function: {}", result);

    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++ Control Flow");
    control_flow::match_fn();

    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++ Loop");
    loop_::loop_fn();
    loop_::while_fn();
    loop_::for_fn();

    println!("");
    println!("#################################### Ownership, Borrowing and Refferences ####################################");
    ownership::owner_fn();
    ownership::borrow_fn();
    ownership::reference_fn();

    println!("");
    println!("#################################### Common Collections ####################################");
    common_collection::vector_fn();
    common_collection::string_fn();
    common_collection::hash_map_fn();

    println!("");
    println!("#################################### Generics ####################################");
    generics::generics_fn();
    generics::generics_w_s();
    generics::generics_w_e();

    println!("");
    println!("#################################### Traits ####################################");
    trait_interface::trait_interface_fn();
    trait_w_generics::trait_fn();
    trait_w_generics::trait_associated_fn();

    println!("");
    println!("#################################### Memmory ####################################");
    memory::stack_mem_fn();
    memory::heap_mem_fn();

    println!("");
    println!("#################################### Smart Pointer ####################################");
    smart_pointers::box_fn();
    smart_pointers::rc_fn();
    smart_pointers::refcell_fn();

    
    println!("");
    println!("#################################### Threads ####################################");
    thread::thread_fn();
    thread::handle_thread_fn();

    println!("");
    println!("#################################### Threads x Smart Pointer ####################################");
    thread_x_smartptr::arc_fn();
    thread_x_smartptr::mutex_fn();

    println!("");
    println!("#################################### Asyncronous ####################################");
    asyncro::async_fn();
}

