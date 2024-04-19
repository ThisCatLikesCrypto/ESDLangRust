use std::fs::File;
use std::io::{self, BufRead, Write}; // Import Write trait for flush method
use std::collections::HashMap;

// Enum to represent variable types
enum Variable {
    Num(f64),    // Numeric value
    Str(String), // String value
}

// Struct to handle variable storage and retrieval
struct VarHandler {
    variables: HashMap<String, Variable>, // HashMap to store variables
}

impl VarHandler {
    // Constructor to create a new VarHandler instance
    fn new() -> Self {
        VarHandler {
            variables: HashMap::new(), // Initialize HashMap
        }
    }

    // Method to set a variable
    fn set_variable(&mut self, name: &str, value: Variable) {
        self.variables.insert(name.to_string(), value); // Insert variable into HashMap
    }

    // Method to get a variable
    fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name) // Retrieve variable from HashMap
    }
}

// Function to process "write" command
fn writer(var_handler: &VarHandler, input_list: &[&str]) {
    for part in input_list {
        if let Some(val) = var_handler.get_variable(*part) { // Check if variable exists
            match val {
                Variable::Num(value) => print!("{} ", value), // Print numeric value
                Variable::Str(value) => print!("{} ", value), // Print string value
            }
        } else {
            print!("{} ", part); // Print variable name if it doesn't exist
        }
    }
    println!(); // Print new line
}

// !FIX LATER, BROKEN
// Function to process "input" command
fn input_handler(var_handler: &mut VarHandler, input_list: &[&str]) {
    if input_list.len() == 3 && input_list[0] == "input" {
        let var_name = input_list[1];
        let prompt_text = input_list[2..].join(" ");
        
        print!("{}: ", prompt_text);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = user_input.trim().to_string();
        
        // Update variables in VarHandler
        var_handler.set_variable(var_name, Variable::Str(user_input.clone()));
    } else {
        println!("Invalid input syntax for 'input' command. (well, likely not but this is broken atm and needs to be fixed later)");
    }
}

fn main() -> io::Result<()> {
    println!("Executing... (ESDLangRust v0.0.1 [Stupid not ful])");

    let filename = "trun.txt";
    let file = File::open(filename)?; // Open file

    let mut var_handler = VarHandler::new(); // Create new VarHandler instance
    let mut lines = io::BufReader::new(file).lines(); // Buffered reader to read lines

    // Loop through each line in the file
    while let Some(Ok(user_input)) = lines.next() {
        let input_list: Vec<&str> = user_input.split_whitespace().collect(); // Split line into words

        // Match first word of the line
        match input_list[0] {
            "write" => writer(&var_handler, &input_list[1..]), // Call writer function
            "var" => {
                if let Some(name) = input_list.get(1) { // Get variable name
                    if let Some(value) = input_list.get(2) { // Get variable value
                        if value == &"input" { // If the value is "input", call input_handler
                            input_handler(&mut var_handler, &input_list[2..]);
                        } else {
                            if let Ok(num) = value.parse::<f64>() { // Try to parse as number
                                var_handler.set_variable(name, Variable::Num(num)); // Set as numeric variable
                            } else {
                                var_handler.set_variable(name, Variable::Str(value.to_string())); // Set as string variable
                            }
                        }
                    }
                }
            }
            _ => {} // Do nothing for unrecognized commands
        }
    }

    Ok(()) // Return Ok if execution is successful
}
