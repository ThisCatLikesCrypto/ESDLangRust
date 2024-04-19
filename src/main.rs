use std::fs::File; // Import File for file operations
use std::io::{self, BufRead}; // Import io and BufRead for reading lines from files
use std::collections::HashMap; // Import HashMap for storing variables
use rand::Rng; // Import the rand crate for random number generation

fn main() -> io::Result<()> {
    println!("Executing... (ESDLang 4.0)");
    let filename = "trun.txt";
    let file = File::open(filename)?; // Open the file specified by filename
    let mut variables: HashMap<String, f64> = HashMap::new(); // Create a HashMap to store variables
    let mut lines = io::BufReader::new(file).lines(); // Create a buffered reader to read lines from the file
    while let Some(Ok(user_input)) = lines.next() {
        // Read each line from the file
        let input_list: Vec<&str> = user_input.split_whitespace().collect(); // Split the line into a vector of words
        match input_list[0] {
            // Match the first word of the line
            "var" => handle_variable(&mut variables, &input_list[1..]), // Call handle_variable function for variable assignment
            "quit" => return Ok(()), // Exit the program if "quit" is encountered
            "write" => print_values(&variables, &input_list[1..]), // Print variable values if "write" is encountered
            "if" => check_condition_for_if_statement(&mut lines, &mut variables, &input_list[1..]), // Handle if statement
            "else" => skip_until_end_statement(&mut lines), // Skip lines until "endstat" if "else" is encountered
            _ => {} // Do nothing for unrecognized commands
        }
    }
    Ok(()) // Return Ok if the program execution is successful
}

fn random_number(min: f64, max: f64) -> f64 {
    // Function to generate a random number between min and max
    rand::thread_rng().gen_range(min..=max) // Use the rand crate to generate the random number
}

fn input_string(prompt: &str) -> String {
    // Function to get user input as a string
    let mut input = String::new(); // Create a new empty string
    println!("{}", prompt); // Print the prompt to the console
    io::stdin().read_line(&mut input).expect("Failed to read line"); // Read input from the console
    input.trim().to_string() // Trim whitespace and convert to String
}

fn parse_float(string: &str) -> Option<f64> {
    // Function to parse a string into a floating-point number
    string.parse().ok() // Attempt to parse the string into a f64, return None if parsing fails
}

fn handle_variable(variables: &mut HashMap<String, f64>, input_list: &[&str]) {
    // Extract variable name and value from input list
    let variable_name = input_list[0].to_string();
    let variable_value = input_list[1].to_string();
    let variable_value_string = input_list[1..].join(" ");

    // Check if the variable already exists in the hashmap
    if let Some(&val) = variables.get(&variable_name) {
        let val_f64 = val;
        // Parse the variable value string to f64
        let var_val_f64 = match parse_float(&variable_value) {
            // If successful parsing, use the parsed value
            Some(parsed_val) => parsed_val,
            // If parsing fails, handle special cases (input or random) or panic
            None => {
                if variable_value_string.starts_with("input") {
                    input_string(&variable_value_string).parse().expect("Failed to parse input as f64")
                } else if variable_value_string.starts_with("random") {
                    let min_max: Vec<&str> = variable_value_string.split_whitespace().skip(1).collect();
                    random_number(min_max[0].parse().expect("Failed to parse min as f64"), 
                                  min_max[1].parse().expect("Failed to parse max as f64"))
                } else {
                    panic!("Failed to parse variable value as f64");
                }
            }
        };
        // Update the variable value based on the operator
        if variable_value.starts_with("-") {
            variables.insert(variable_name, val_f64 - var_val_f64);
        } else if variable_value.starts_with("+") {
            variables.insert(variable_name, val_f64 + var_val_f64);
        } else {
            variables.insert(variable_name, var_val_f64);
        }
    } else {
        // If the variable does not exist, parse the value and insert it into the hashmap
        let var_val_f64 = match parse_float(&variable_value) {
            Some(parsed_val) => parsed_val,
            None => {
                if variable_value_string.starts_with("input") {
                    input_string(&variable_value_string).parse().expect("Failed to parse input as f64")
                } else if variable_value_string.starts_with("random") {
                    let min_max: Vec<&str> = variable_value_string.split_whitespace().skip(1).collect();
                    random_number(min_max[0].parse().expect("Failed to parse min as f64"), 
                                  min_max[1].parse().expect("Failed to parse max as f64"))
                } else {
                    panic!("Failed to parse variable value as f64");
                }
            }
        };
        variables.insert(variable_name, var_val_f64);
    }
}


fn print_values(variables: &HashMap<String, f64>, input_list: &[&str]) {
    // Function to print variable values
    for part in input_list {
        if let Some(val) = variables.get(*part) {
            // Check if the variable exists in the HashMap
            print!("{} ", val); // Print the variable value if it exists
        } else {
            print!("{} ", part); // Otherwise, print the variable name
        }
    }
    println!(); // Print a newline character
}

fn check_condition_for_if_statement(lines: &mut io::Lines<io::BufReader<File>>, variables: &mut HashMap<String, f64>, input_list: &[&str]) {
    // Function to handle if statements
    if input_list.len() != 3 {
        println!("An if statement requires 3 arguments, {} are given", input_list.len()); // Print an error message if the if statement has incorrect arguments
    } else {
        let val1 = variables.get(input_list[0]).cloned().unwrap_or_else(|| parse_float(input_list[0]).unwrap_or(0.0)); // Get the value of the first variable or parse it as f64
        let val2 = variables.get(input_list[2]).cloned().unwrap_or_else(|| parse_float(input_list[2]).unwrap_or(0.0)); // Get the value of the second variable or parse it as f64
        let condition = input_list[1]; // Get the condition (e.g., ">", "<=", "=")
        let mut user_input = String::new(); // Create a new empty string to store user input
        if check_condition(val1, condition, val2) {
            // Check the condition using the check_condition function
            while user_input != "else" {
                lines.next(); // Skip lines until "else" is encountered
                user_input.clear(); // Clear the user input string
                if let Some(Ok(line)) = lines.next() {
                    user_input = line; // Read the next line from the file
                }
            }
        } else {
            skip_until_end_statement(lines); // Skip lines until "endstat" if the condition is not met
        }
    }
}

fn check_condition(val1: f64, condition: &str, val2: f64) -> bool {
    // Function to check a condition
    match condition {
        ">=" => val1 >= val2, // Check if val1 is greater than or equal to val2
        "<=" => val1 <= val2, // Check if val1 is less than or equal to val2
        "=" => val1 == val2,  // Check if val1 is equal to val2
        "x" => val1 != val2,  // Check if val1 is not equal to val2
        ">" => val1 > val2,    // Check if val1 is greater than val2
        "<" => val1 < val2,    // Check if val1 is less than val2
        _ => false,            // Return false for unrecognized conditions
    }
}

fn skip_until_end_statement(lines: &mut io::Lines<io::BufReader<File>>) {
    // Function to skip lines until "endstat" is encountered
    let mut user_input = String::new(); // Create a new empty string to store user input
    while user_input != "endstat" {
        user_input.clear(); // Clear the user input string
        if let Some(Ok(line)) = lines.next() {
            user_input = line; // Read the next line from the file
        }
    }
}