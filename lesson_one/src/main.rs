// import the var_type function from the /utils/variables_types module
mod utils;

fn main() {
    // utils::variables_types::var_primitive_types();
    // utils::variables_types::var_composite_types();
    //utils::variables_types::var_collections_types();
    //utils::variables_types::_info();
    let resul = utils::conditions::if_statment();
    println!("Result of if_statment function is: {}", resul);
    utils::conditions::if_else_statment();
}
