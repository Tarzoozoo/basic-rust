mod data_type;
mod control_flow;
mod loop_;
mod ownership;
mod common_collection;
fn main() {

    println!("Hello, Tarzoozoo!");
    data_type::scalar_type();
    data_type::compound_type();
    data_type::string_type();
    data_type::slice_type();
    data_type::vector_type();
    
    let result = data_type::sqare(5, 10);
    println!("Result of function: {}", result);

    // --------------------------------------------
    control_flow::match_fn();

    // --------------------------------------------
    loop_::loop_fn();
    loop_::while_fn();
    loop_::for_fn();

    // --------------------------------------------
    ownership::owner_fn();
    ownership::borrow_fn();
    ownership::reference_fn();

    // --------------------------------------------
    common_collection::vector_fn();
    common_collection::string_fn();
    common_collection::hash_map_fn();
}

