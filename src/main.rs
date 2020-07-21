use std::collections::HashMap;

fn main() {
    println!("Welcome to rh_ust!");
    println!("List of commands:");
    println!("add");
    println!("list");
    println!("quit");

    let mut departments : HashMap<&str, Vec<String>> = HashMap::new();

    let mut input = String::new();
    loop {
    	input.clear();
    	std::io::stdin().read_line(&mut input).expect("failed to read line")	;
    	let mut input_words = input.split_whitespace();
    	let command = match input_words.next() {
    		Some(word) => {
    			match word {
    				"add" | "list" | "quit" => word,
    				_ => {
    					println!("Invalid command {}", word );
    					continue;
    				}
    			}
    		},
    		None => {
    			continue
    		}
    	};
    	match command {
    		"add" => {
	    		add()
	    	},
    		_ => ()
    	}
    }
}

fn add(parameters) -> (String, String) {
	let mut employee = String::new();
	if.parametes.next().starts_with("\"") {

	}
}

fn input_valid(valid : Vec<String>) -> String {
	let mut input = String::new();
	loop {
		input.clear();
		std::io::stdin().read_line(&mut input).expect("failed to read line");
		let mut words = input.split_whitespace();
		let command = match words.next() {
			Some(word) => {
				break word.to_string();
			},
			None => continue,
		};
	}
}