use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use rand::Rng; // Import the rand crate

fn main() -> io::Result<()> {
    println!("Executing... (ESDLang 4.0)");
    let filename = "trun.txt";
    let file = File::open(filename)?;
    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut lines = io::BufReader::new(file).lines();
    while let Some(Ok(user_input)) = lines.next() {
        let input_list: Vec<&str> = user_input.split_whitespace().collect();
        match input_list[0] {
            "var" => handle_variable(&mut variables, &input_list[1..]),
            "quit" => return Ok(()),
            "write" => print_values(&variables, &input_list[1..]),
            "if" => check_condition_for_if_statement(&mut lines, &mut variables, &input_list[1..]), // Call the renamed function
            "else" => skip_until_end_statement(&mut lines),
            _ => {}
        }
    }
    Ok(())
}

fn random_number(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..=max)
}

fn input_string(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn parse_float(string: &str) -> Option<f64> {
    string.parse().ok() // Adjusted to return Option<f64>
}

fn handle_variable(variables: &mut HashMap<String, f64>, input_list: &[&str]) {
    let variable_name = input_list[0].to_string();
    let variable_value = input_list[1].to_string();
    let variable_value_string = input_list[1..].join(" ");
    if let Some(&val) = variables.get(&variable_name) {
        let val_f64 = val;
        let var_val_f64 = parse_float(&variable_value).unwrap_or_else(|| { // Adjusted to use parse_float function
            if variable_value_string.starts_with("input") {
                input_string(&variable_value_string).parse().unwrap() // Adjusted to use parse_float function
            } else if variable_value_string.starts_with("random") {
                let min_max: Vec<&str> = variable_value_string.split_whitespace().skip(1).collect();
                random_number(min_max[0].parse().unwrap(), min_max[1].parse().unwrap()) // Adjusted to use random_number function
            } else {
                parse_float(&variable_value).unwrap() // Adjusted to use parse_float function
            }
        });
        if variable_value.starts_with("-") {
            variables.insert(variable_name, val_f64 - var_val_f64);
        } else if variable_value.starts_with("+") {
            variables.insert(variable_name, val_f64 + var_val_f64);
        } else {
            variables.insert(variable_name, var_val_f64);
        }
    } else {
        let var_val_f64 = parse_float(&variable_value).unwrap_or_else(|| { // Adjusted to use parse_float function
            if variable_value_string.starts_with("input") {
                input_string(&variable_value_string).parse().unwrap() // Adjusted to use parse_float function
            } else if variable_value_string.starts_with("random") {
                let min_max: Vec<&str> = variable_value_string.split_whitespace().skip(1).collect();
                random_number(min_max[0].parse().unwrap(), min_max[1].parse().unwrap()) // Adjusted to use random_number function
            } else {
                parse_float(&variable_value).unwrap() // Adjusted to use parse_float function
            }
        });
        variables.insert(variable_name, var_val_f64);
    }
}

fn print_values(variables: &HashMap<String, f64>, input_list: &[&str]) {
    for part in input_list {
        if let Some(val) = variables.get(*part) {
            print!("{} ", val);
        } else {
            print!("{} ", part);
        }
    }
    println!();
}

fn check_condition_for_if_statement(lines: &mut io::Lines<io::BufReader<File>>, variables: &mut HashMap<String, f64>, input_list: &[&str]) {
    if input_list.len() != 3 {
        println!("An if statement requires 3 arguments, {} are given", input_list.len());
    } else {
        let val1 = variables.get(input_list[0]).cloned().unwrap_or_else(|| input_list[0].parse().unwrap());
        let val2 = variables.get(input_list[2]).cloned().unwrap_or_else(|| input_list[2].parse().unwrap());
        let condition = input_list[1];
        let mut user_input = String::new();
        if check_condition(val1, condition, val2) {
            while user_input != "else" {
                lines.next();
                user_input.clear();
                if let Some(Ok(line)) = lines.next() {
                    user_input = line;
                }
            }
        } else {
            skip_until_end_statement(lines);
        }
    }
}

fn check_condition(val1: f64, condition: &str, val2: f64) -> bool {
    match condition {
        ">=" => val1 >= val2,
        "<=" => val1 <= val2,
        "=" => val1 == val2,
        "x" => val1 != val2,
        ">" => val1 > val2,
        "<" => val1 < val2,
        _ => false,
    }
}

fn skip_until_end_statement(lines: &mut io::Lines<io::BufReader<File>>) {
    let mut user_input = String::new();
    while user_input != "endstat" {
        user_input.clear();
        if let Some(Ok(line)) = lines.next() {
            user_input = line;
        }
    }
}
