fn main() {
    return_value_from_loop();
    break_labeled_loop();
    break_labeled_loop_detailed();
    find_target_in_matrix();
    find_pattern_in_image();
}

// Returning values from a loop
fn return_value_from_loop() {
    banner("*", 52, "Returning Values from a loop");
    let mut num = 1;

    let result = loop {
        num += 1;

        if num > 10 && num % 2 == 0 && num % 3 == 0 {
            break num;
        }
    };

    println!("The number is: {}", result);
    println!("{}", "*".repeat(52));
}

// Breaking labeled loop
#[allow(unused_labels)]
#[allow(unreachable_code)]
fn break_labeled_loop() {
    banner("*", 52, "Breaking Loop by labels");
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // Condition to break the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
    println!("{}", "*".repeat(52));
}

//
fn break_labeled_loop_detailed() {
    banner("*", 52, "Understanding nested loop and labels");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!("{}", "*".repeat(52));
}

// Searching using nested loop

fn find_target_in_matrix() {
    banner("*", 52, "Practical Example of Nested loop with labels");
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let target = 5;
    let mut found = false;

    'outer: for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == target {
                println!("Found {} at position ({}, {})", target, i, j);
                found = true;
                break 'outer;
            }
        }
    }

    if !found {
        println!("{} not found in the matrix", target);
    }

    println!("{}", "*".repeat(52));
}

// Example 3D image matrix (2x2 pixels for simplicity)
fn find_pattern_in_image() {
    banner(
        "*",
        52,
        "Example 3D image matrix (2x2 pixels for simplicity)",
    );
    let image = [
        [[255, 0, 0], [0, 255, 0]],   // Row 1: [Red, Green]
        [[0, 0, 255], [255, 255, 0]], // Row 2: [Blue, Yellow]
    ];

    // Target pattern (Yellow)
    let target = [255, 255, 0];
    let mut found = false;

    'outer: for (i, row) in image.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            if pixel == &target {
                println!("Found the pattern at position ({}, {})", i, j);
                found = true;
                break 'outer;
            }
        }
    }

    if !found {
        println!("Pattern not found in the image");
    }

    println!("{}", "*".repeat(52))
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
