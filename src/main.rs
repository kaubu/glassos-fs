use std::path::{PathBuf};
use std::io::{self, Write};
use std::fs::{self, File};

const PREFIX: &str = "./"; // Change to "/" if you want to start from real root directory

fn main() {
	let mut current_path = PathBuf::new();
	current_path.push(PREFIX);

	loop {
		let mut input = String::new();
		
		if current_path == PathBuf::from("") {
			println!("error: Can not go above root directory");
			current_path = PathBuf::from("./");
			continue;
		} else if current_path == PathBuf::from("/") {
			print!("{}> ", current_path.display());
		} else {
			print!("/{}> ", current_path.strip_prefix(PREFIX).unwrap().display());
		}
		
		if let Err(_) = io::stdout().flush() {
			println!("error: Could not flush stdout");
			continue;
		}

		if let Err(_) = io::stdin().read_line(&mut input) {
			println!("error: Could not read input");
			continue;
		}

		input = input.trim().to_string();

		let commands: Vec<&str> = input.split(" ").collect();

		if commands.len() < 1 {
			continue;
		}

		if commands[0] == "cd" {
			if commands.len() < 2 {
				println!("error: This command requires two arguments");
				continue;
			}

			let mut dirs = commands.clone();
			dirs.swap_remove(0);
			let dirs: Vec<&str> = dirs[0].split("/").collect(); // 0 because previous 0 was removed

			for dir in dirs.iter() {
				if *dir == "." {
					continue;
				} else if *dir == ".." {
					current_path.pop();
				} else {
					current_path.push(dir);
				}
				
				let current_path_str = current_path.to_string_lossy();
				if !current_path.is_dir() && current_path_str != "" {
					println!("error: '{}' is not a directory", dir);
					current_path.pop();
				}
			}
		} else if commands[0] == "ls" {
			let mut entries: Vec<String> = Vec::new();
			
			for entry in current_path.read_dir().expect("error: Could not read directory") {
				if let Ok(entry) = entry {
					let mut placeholder = String::from("");
					if entry.path().is_dir() {
						placeholder = String::from("/");
					}

					entries.push(format!("{}{}", entry.path().strip_prefix(format!("{}", current_path.display())).unwrap().display(), placeholder));
				} else {
					println!("error: Could not read file entry");
					continue;
				}
			}

			// Sort the vector, in the future sort by /, so whether it's a directory or not
			entries.sort();

			for entry in entries {
				println!("{}", entry);
			}
		} else if commands[0] == "clear" {
			print!("{esc}c", esc = 27 as char); // Might use crossterm for clearing
		} else if commands[0] == "quit" || commands[0] == "exit" {
			break;
		} else if commands[0] == "pwd" {
			println!("/{}", current_path.strip_prefix(PREFIX).unwrap().display());
		} else if commands[0] == "mkdir" {
			if commands.len() < 2 {
				println!("error: This command requires two arguments");
				continue;
			}

			let dir = commands[1];

			match fs::create_dir_all(format!("{}/{}", current_path.display(), dir)) {
				Ok(_) => println!("Successfully created '{}' directory", dir),
				Err(error) => {
					println!("error: Could not create '{}' directory", dir);
					println!("error: {}", error);
				},
			}
		} else if commands[0] == "rmdir" { // Removes an empty directory
			if commands.len() < 2 {
				println!("error: This command requires two arguments");
				continue;
			}

			let dir = commands[1];

			match fs::remove_dir(format!("{}/{}", current_path.display(), dir)) {
				Ok(_) => println!("Successfully removed '{}' directory", dir),
				Err(error) => {
					println!("error: Could not remove '{}' directory", dir);
					println!("error: {}", error);
				},
			}
		} else if commands[0] == "rm" { // Removes a file
			if commands.len() < 2 {
				println!("error: This command requires two arguments");
				continue;
			}

			let file = PathBuf::from(format!("{}/{}", current_path.display(), commands[1]));

			match fs::remove_file(file) {
				Ok(_) => println!("Successfully removed '{}' file", commands[1]),
				Err(error) => {
					println!("error: Could not remove '{}' file", commands[1]);
					println!("error: {}", error);
				},
			}
		} else if commands[0] == "rmall" { // Remove a directory recursively
			if commands.len() < 2 {
				println!("error: This command requires two arguments");
				continue;
			}
			
			print!("Doing this will remove the directory recursively, meaning it will delete all contents.\nAre you sure you want to do this? (Type 'yes' to continue, type anything else to stop.)\n>> ");
		
			let mut warning = String::new();

			// Put all this inside of an input function
			if let Err(_) = io::stdout().flush() {
				println!("error: Could not flush stdout");
				continue;
			}

			if let Err(_) = io::stdin().read_line(&mut warning) {
				println!("error: Could not read input");
				continue;
			}

			let warning = warning.trim();

			if warning == "yes".to_string() {

				let dir = PathBuf::from(format!("{}/{}", current_path.display(), commands[1]));

				match fs::remove_dir_all(dir) {
					Ok(_) => println!("Successfully removed '{}' directory and it's contents", commands[1]),
					Err(error) => {
						println!("error: Could not remove '{}' directory and it's contents", commands[1]);
						println!("error: {}", error);
					},
				}
			} else {
				println!("info: Cancelling operation");
			}
		} else if commands[0] == "mv" {
			if commands.len() < 3 {
				println!("error: This command requires three arguments");
				continue;
			}

			let from_file = PathBuf::from(format!("{}/{}", current_path.display(), commands[1]));
			let to_file = PathBuf::from(format!("{}/{}", current_path.display(), commands[2]));

			if !from_file.is_file() {
				println!("error: '{}' is not a file", from_file.display());
				continue;
			} else if !to_file.is_file() {
				println!("error: '{}' is not a file", to_file.display());
				continue;
			}

			match fs::rename(from_file, to_file) {
				Ok(_) => println!("Successfully renamed file from '{}' to '{}'", commands[1], commands[2]),
				Err(error) => {
					println!("error: Could not rename file from '{}' to '{}'", commands[1], commands[2]);
					println!("error: {}", error);
				},
			}
		} else if commands[0] == "cp" {
			if commands.len() < 3 {
				println!("error: This command requires three arguments");
				continue;
			}

			let from_file = PathBuf::from(format!("{}/{}", current_path.display(), commands[1]));
			let to_file = PathBuf::from(format!("{}/{}", current_path.display(), commands[2]));

			match fs::copy(from_file, to_file) {
				Ok(_) => println!("Successfully copied file from '{}' to '{}'", commands[1], commands[2]),
				Err(error) => {
					println!("error: Could not copy file from '{}' to '{}'", commands[1], commands[2]);
					println!("error: {}", error);
				},
			}
		} else if commands[0] == "cat" {
			if commands.len() < 2 {
				println!("error: This command requires two arguments");
				continue;
			}

			let file = PathBuf::from(format!("{}/{}", current_path.display(), commands[1]));

			match fs::read_to_string(file) {
				Ok(contents) => println!("{}", contents),
				Err(error) => {
					println!("error: Could not remove '{}' file", commands[1]);
					println!("error: {}", error);
				},
			}
		} else if commands[0] == "new" {
			if commands.len() < 2 {
				println!("error: This command requires at least two arguments");
				continue;
			}

			let file = PathBuf::from(format!("{}/{}", current_path.display(), commands[1]));

			if commands.len() == 2 { // Create an empty file
				match File::create(file) {
					Ok(_) => println!("Successfully created empty file '{}'", commands[1]),
					Err(error) => {
						println!("error: Could not create empty file '{}'", commands[1]);
						println!("error: {}", error);
					},
				}
			} else { // Create a file using the rest of the arguments
				let contents = commands[2..].to_vec().join(" ");
				
				match fs::write(file, contents) {
					Ok(_) => println!("Successfully created file '{}'", commands[1]),
					Err(error) => {
						println!("error: Could not create file '{}'", commands[1]);
						println!("error: {}", error);
					},
				}
			}
		}
	}
}