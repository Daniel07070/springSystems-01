use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}
fn perform_operation(operation: FileOperation){
    match operation{
        FileOperation::List(directory_path) =>{
           Command::new("ls").arg(&directory_path).status().expect("Failed to execute ls");
        }

        FileOperation::Display(file_path) =>{
           Command::new("cat").arg(&file_path).status().expect("Failed to execute cat");

        }

        FileOperation::Create(file_path, content) =>{
            let command = format!("echo '{}' > {}", content, file_path);
            let result = Command::new("sh").arg("-c").arg(command).status();
            
            match result {
                Ok(_) => println!("File '{}' created successfully", file_path),
                Err(_) => println!("Failed to create file"),
            }
        }
        
        FileOperation::Remove(file_path) =>{
            let result = Command::new("rm").arg(&file_path).status();

            match result {
                Ok(_) => println!("File '{}' removed successfully", file_path),
                Err(_) => println!("Failed to remove file"),
            }
        }

        FileOperation::Pwd =>{
            Command::new("pwd").status().expect("Failed to execute pwd");

        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new(); //input needs to be mutable 
    print!("{}", prompt);

    io::stdout().flush().unwrap(); //unwrap so it's readable
    //fail case
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
fn main(){
    println!("Welcome to the File Operations Program!");

    loop{
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit \n");

        let user_input = read_input("Enter your choice (0-5):");
        //match case to see what user_input corresponds to
        match user_input.as_str(){

            "1" => {
                let dir = read_input("Enter directory path: ");
                perform_operation(FileOperation::List(dir));
            }
            "2" => {
                let file = read_input("Enter file path: ");
                perform_operation(FileOperation::Display(file));
            }
            "3" => {
                let file = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                perform_operation(FileOperation::Create(file, content));
            }
            "4" => {
                let file = read_input("Enter file path: ");
                perform_operation(FileOperation::Remove(file));
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Please enter a valid option(0-5).");
            }

        }

    }
}