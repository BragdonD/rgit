use std::io;
use crate::core::adder::add;
use crate::object::object::Object;

pub(crate) mod object;
pub(crate) mod core;

/// Note to myself:
/// unwrap() is not safe to use in production code 
/// because it will panic if an error occurs. You need to change it later

fn main() -> io::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let (command_name, parameter, additional_parameters) = parse_config(&argv).unwrap();
    match command_name {
        "add" => {
            add(parameter, additional_parameters).unwrap();
        },
        "--help" | _ => {
            // Needs to be implemented to show the usage
            println!("Unknown command: {}", command_name);
        }
    }

    Ok(())
}

fn parse_config(args: &[String]) -> io::Result<(&str, &str, &[String])> {
    if args.len() < 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not enough arguments",
        ));
    }

    let command_name = &args[1];
    let parameter = &args[2];
    let mut additional_parameters = &[] as &[String];

    if args.len() > 3 {
        additional_parameters = &args[3..];
    }
    Ok((command_name, parameter, additional_parameters))
}

fn handle_error(error: io::Error) {
    println!("Error: {}", error);
}