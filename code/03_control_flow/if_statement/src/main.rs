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
    #[allow(unused_parens)]
    if (num > 0) {
        println!("{} is positive number", num);
    }
}

// another example

fn with_without_parens() {
    let number = 5;

    // Both styles are valid
    if number > 0 {
        println!("No parentheses needed.");
    }

    // You will get warning here
    // you can suppress the warning though
    #[allow(unused_parens)]
    if (number > 0) {
        println!("Parentheses are optional.");
    }
}

// Erroneous code, non-boolean-evaluation

fn non_bool_eval() {
    let num = 7;

    if num {
        println!("{} is positive.", num);
    }
}
