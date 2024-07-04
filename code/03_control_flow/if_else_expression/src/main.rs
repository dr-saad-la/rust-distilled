const MAX_TEMP: f64 = 85.0; // Maximum temperature threshold

enum HttpStatus {
    Ok,
    ServerError,
}

struct Cpu {
    temperature: f64,
}

fn main() {
    banner("*", 52, "The if-else Expression: Simple Example");
    simple_let_binding();

    banner("*", 52, "The if-else Expression: Advanced Example");
    let cpu = Cpu { temperature: 90.0 };
    println!("The maximum temperature: {}", MAX_TEMP);
    println!("The cpu temperature value: {}", cpu.temperature);

    let status = if cpu.temperature <= MAX_TEMP {
        HttpStatus::Ok
    } else {
        HttpStatus::ServerError
    };

    match status {
        HttpStatus::Ok => println!("System is running fine."),
        HttpStatus::ServerError => println!("Server melted down!"),
    }
}

// Simple example
fn simple_let_binding() {
    let number = 5;
    let is_positive = if number > 0 { true } else { false };

    println!("Is the number positive? {}", is_positive);
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); // Creating a separator line
    let message = format!("{:^width$}", message, width = nchar); // Centering the message
    println!("\n{}\n{}\n{}", sep, message, sep); // Printing the banner
}
