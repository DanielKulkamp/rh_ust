use std::collections::HashMap;

fn main() {
    println!("Welcome to rh_ust!");
    help();

    let mut departments : HashMap<&str, Vec<String>> = HashMap::new();

    loop {
    	match get_command(){
    		Some(Command::Add) => add(),
    		Some(Command::List) => list(),
    		Some(Command::Quit) => break,
    		Some(Command::Help) => help(),
    		None => continue,
    	}

    }
}


enum Command  {
	Add,
	List,
	Quit,
	Help,
}

fn add() {
	println!("dummy function add");
}

fn list() {
	println!("dummy function list");
}

fn help() {
	println!("List of commands:");
    println!("add");
    println!("list");
    println!("quit");
}

fn get_command() -> Option<Command> {
	loop {
		let input = user_input().unwrap_or_else(|error| {
			println!("Failed to read line! try again... or press Ctrl-C");
			"".to_string()
		});	
		let mut words = input.split_whitespace();
		let command = match words.next() {
			Some(word) => {
				match word {
					"add" => return Some(Command::Add),
					"list" => return Some(Command::List),
					"help" => return Some(Command::Help),
					"quit" => return Some(Command::Quit),
					_ => {
						println!("Invalid command.");
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
	Ok(text)
}