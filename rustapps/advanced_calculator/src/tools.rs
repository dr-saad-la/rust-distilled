/// Creates a formatted banner with a specified separator, number of characters, and message.
///
/// # Arguments
///
/// * `sep` - The separator string to use for the banner.
/// * `nchar` - The number of characters for the width of the banner.
/// * `message` - The message to display within the banner.
///
//// # Examples
///
/// ```
/// use advanced_calculator::tools::banner;
/// banner("*", 30, "Welcome to Advanced Calculator");
/// ```
///
/// The above example will print:
///
/// ```text
/// ******************************
///    Welcome to Advanced Calculator
/// ******************************
/// ```
pub fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
