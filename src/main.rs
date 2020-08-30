use std::collections::HashMap;
use std::fmt;
use rh_ust::{RhDbError, RhDatabase};


fn main() {
    println!("Welcome to rh_ust!");
    help();

    let mut departments : HashMap<String, Vec<String>> = HashMap::new();

    loop {
    	println!("Enter your command: ");
    	match get_command(){
    		Command::Add => add(&mut departments),
    		Command::List => list(&mut departments),
    		Command::Quit => break,
    		Command::Help => help(),
    	}
    }
}

enum Command  {
	Add,
	List,
	Quit,
	Help,
}

fn add(departments: &mut HashMap<String, Vec<String>>) {
	println!("Enter the name of the employee");
	let employee = user_input().unwrap_or_else(|_error| {
		println!("Something wrong happened when getting employee name.");
		return "".to_string();
	});
	println!("Enter the department name you want to add '{}' to", employee);
	let dept = user_input().unwrap_or_else(|_error| {
		println!("Something wrong happened when getting department name");
		return "".to_string();
	});
	if employee == "".to_string() || dept == "".to_string(){
		println!("command not executed!");
	}
	if get_confirmation(fmt::format(format_args!("Add '{}' to '{}'? (y/n):", &employee, &dept ))) {
		if !departments.contains_key(dept.as_str()) {
			if get_confirmation(fmt::format(format_args!("{} doesn't exist. Create it? (y/n):", &dept))){
				departments.entry(dept).or_insert(vec![employee]);
				println!("created deptarment. employee added");			
			} else{
				println!("Command canceled.");
			}
		} else {
			match departments.get_mut(&dept){
				Some(employee_list) => {
					employee_list.push(employee);
				},
				None => {
					println!("Failed to add employee. ")
				}

			}

		}
	} else {
		println!("not adding");
	}

}

fn list(departments: &mut HashMap<String, Vec<String>>) {
	println!("Enter the name of the department or '*' for all departments");
	let dept = user_input().unwrap_or_else(|_error| {
		println!("failed to read  department line.");
		"".to_string()
	});
	if dept.starts_with('*') {
		println!("listing all departments");
		for dept in departments.keys() {
			println!("Listing employees in {}", dept);
			let mut employees = match departments.get(dept) {
				Some(vector) => vector.clone(),
				None => Vec::new(),
			};
			employees.sort();
			for (number, employee) in employees.iter().enumerate(){
				println!("{}: {}", number, employee );
			}
		}
	} else if !departments.contains_key(&dept) {
		println!("No such department {}", dept);
		return;
	} else {
		println!("Listing employees in {}", dept);
		let mut employees = match departments.get(&dept) {
			Some(vector) => vector.clone(),
			None => Vec::new(),
		};
		employees.sort();
		for (number, employee) in employees.iter().enumerate(){
			println!("{}: {}", number, employee );
		}
	}
}

fn help() {
	println!("List of commands:");
    println!("add");
    println!("list");
    println!("quit");
}

fn get_confirmation(prompt : String) -> bool {
	loop {
		println!("{}", prompt );
		let input = user_input().unwrap_or_else(|_error| {
			println!("Failed to read line! try again... o press Ctrl-C");
			"".to_string()
		});
		let mut words = input.split_whitespace();
		match words.next(){
			Some(word) => {
				match word{
					"y" | "yes" => return true,
					"n" | "no" => return false,
					_ => continue,
				}
			},
			None => continue,
		}
	}
}

fn get_command() -> Command {
	loop {
		let input = user_input().unwrap_or_else(|error| {
			println!("{} Failed to read line! try again or press Ctrl-C", error);
			"".to_string()
		});	
		let mut words = input.split_whitespace();
		match words.next() {
			Some(word) => {
				match word {
					"add" => return Command::Add,
					"list" => return Command::List,
					"help" => return Command::Help,
					"quit" => return Command::Quit,
					_ => {
						println!("Invalid command: '{}'", word);
						continue;
					}
				}
			},
			None => continue,
		};
	}
}

fn user_input() -> Result<String, std::io::Error> {
	let mut text = String::new();
	std::io::stdin().read_line(&mut text)?;
	Ok(text.as_str().trim().to_string())
}




