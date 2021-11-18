/**
 * A calculator to evaluate arithmetic expression. 
 * Supported operators: +, -, *, /, % (remainder), ^ (power).
 * Rust crate meval is used.
 */

use std::io;

fn main() {
    
    // Declaration of the variable
    let mut input = String::new();

    // Print a welcome message
    println!("Start");
    println!("--------------------------");

    // Read the input line; match the input
    match io::stdin().read_line(&mut input) {
        Ok(_) => {      // Print the expression entered
            print!("{} =", input);
        }
        Err(error) => println!("error {}", error),
    }
        
    // Evaluate the input
    let result = meval::eval_str(input).unwrap();
    // Print the result of calculation
    println!("{}", result);  
}
