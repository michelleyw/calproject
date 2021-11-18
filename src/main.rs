/**
 * A simple calculator that can only evaluate expressions with three terms.
 * Supported operators: +, -, *, /.
 */

use std::io::{stdin, stdout, Write};

// Ensure the output is emitted immediately
// Read the input
fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {    
    
    // Print a welcome message
    println!("This calculator can evaluate expressions with three terms.");
    println!("Start");
    println!("-----------------------------------------------------------");

    // Declaration of three terms with data type
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();

    // Declaration of two operators with data type
    let mut ope1 = String::new();
    let mut ope2 = String::new();

    // Notification to insert the first term
    print!("Enter number: ");
    read(&mut num1);

    // Notification to insert the first operator
    print!("Enter operator: [+-*/]");
    read(&mut ope1);

    // Notification to insert the second term
    print!("Enter number: ");
    read(&mut num2);

    // Notification to insert the second operator
    print!("Enter operator: [+-*/]");
    read(&mut ope2);

    // Notification to insert the third term
    print!("Enter number: ");
    read(&mut num3);

    // Re-define variables' data types
    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();
    let num3: f32 = num3.trim().parse().unwrap();
    let ope1: char = ope1.trim().chars().next().unwrap();
    let ope2: char = ope2.trim().chars().next().unwrap();

    // Calculate the expression 
    if ope1 == '*' {
        if ope2 == '+' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 * num2 + num3);
        } else if ope2 == '-' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 * num2 - num3);
        } else if ope2 == '*' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 * num2 * num3);
        } else if ope2 == '/' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 * num2 / num3);
        }
    } else if ope1 == '/' {
        if ope2 == '+' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 / num2 + num3);
        } else if ope2 == '-' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 / num2 - num3);
        } else if ope2 == '*' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 / num2 * num3);
        } else if ope2 == '/' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 / num2 / num3);
        }
    } else if ope1 == '+' {
        if ope2 == '+' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 + num2 + num3);
        } else if ope2 == '-' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 + num2 - num3);
        } else if ope2 == '*' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 + num2 * num3);
        } else if ope2 == '/' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 + num2 / num3);
        }
    } else if ope1 == '-' {
        if ope2 == '+' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 - num2 + num3);
        } else if ope2 == '-' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 - num2 - num3);
        } else if ope2 == '*' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 - num2 * num3);
        } else if ope2 == '/' {
            println!("{} {} {} {} {} = {}", num1, ope1, num2, ope2, num3, num1 - num2 / num3);
        }
    }
}
        
            
            
