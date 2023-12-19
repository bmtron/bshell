use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    b_loop(&args);
}

fn b_loop(_args: &Vec<String>) {
    let mut line: String;
    let mut split_args: Vec<&str>;
    let mut status: Option<i32>;

    loop {
        print!("{}", "> ");
        io::stdout().flush().unwrap();
        line = match b_read_line() {
            Ok(res) => res,
            Err(_e) => return, 
        };
        split_args = b_split_line(&line);
        status = b_execute(&split_args);
        
        if !Option::is_some(&status) {
            break;
        }
    }
}

fn b_read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn b_split_line(line: &String) -> Vec<&str> {
    let whitespace_delim = line.split(' ').collect::<Vec<&str>>();
    whitespace_delim
}

fn b_execute(args: &Vec<&str>) -> Option<i32> {

    if args[0] == "echo" {
        //implement actual syscall here
        println!("{}", &args[1]);
        Some(1)
    } else if args[0] == "cd" {
        let new_path = args[1];
        let _ = env::set_current_dir(&new_path);
        Some(1)
    } else {
        None
    }
}

