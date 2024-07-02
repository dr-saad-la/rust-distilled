fn main() {
    println!("{}", "*".repeat(32));
    simple_if_statement();
    println!("{}", "*".repeat(32));

    if_with_parentheses();
}

// Simple if statement
fn simple_if_statement() {
    let number = 5;
    if number > 0 {
        println!("The number is positive.");
    }
}

// Bad code; This code will not run

// fn bad_if_statement(){
//  let number = 5;
//     if number > 0
//         println!("The number is positive.");
// }

// Warning code

fn if_with_parentheses() {
    let num = 7;
    if (num > 0) {
        println!("{} is positive number", num);
    }
}
