use std::collections::HashMap;

fn main() {
    println!("Welcome to rh_ust!");
    help();

    let mut departments : HashMap<&str, Vec<String>> = HashMap::new();

    loop {
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

fn add(departments: &mut HashMap<&str, Vec<String>>) {
	println!("Enter the name of the employee");
	let employee = user_input().unwrap_or_else(|error| {
		println!("Something wrong happened when getting employee name.");
		return "".to_string();
	});
	println!("Enter the departmant name you want to add '{}' to", employee);
	let dept = user_input().unwrap_or_else(|error| {
		println!("Something wrong happened when getting department name");
		return "".to_string();
	});
	if employee == "".to_string() || dept == "".to_string(){
		println!("command not executed!");
	}
	if get_confirmation(std::fmt::format(format_args!("Add '{}' to '{}'? (y/n):", employee, dept ))) {
		println!("Adding...");
	} else {
		println!("not adding");
	}

}

fn list(departments: &mut HashMap<&str, Vec<String>>) {
	println!("dummy function list");
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
		let input = user_input().unwrap_or_else(|error| {
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
			println!("Failed to read line! try again... or press Ctrl-C");
			"".to_string()
		});	
		let mut words = input.split_whitespace();
		let command = match words.next() {
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