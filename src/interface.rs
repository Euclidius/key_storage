pub mod interface {
    use std::env;

    use crate::commands::commands;

    pub enum Command {
        Init,
        Add,
        Show,
        Remove,
        Keygen,
        Help,
        Error
    }

    pub fn get_env_args() -> Vec<String> {
        env::args().collect::<Vec<String>>()  
    }

    pub fn check_env_args(env_args: &Vec<String>) {
        if env_args.len() < 2 {
            eprintln!("Usage is \"key_storage <command> <arguments for command>\". \nTry \"key_storage help\".");
            std::process::exit(1);
        }
    }

    pub fn get_command(env_args: &Vec<String>) -> Command {
        let command:&str = &env_args[1];
        match command {
            "init" => {Command::Init}
            "add" => {Command::Add}
            "show" => {Command::Show}
            "remove" => {Command::Remove}
            "keygen" => {Command::Keygen}
            "help" => {Command::Help}
            _ => {Command::Error}
        }
    }

    pub fn get_command_args(env_args: &[String]) -> &[String] {
        &env_args[2..]
    }

    pub fn do_interaction(command:Command, command_args: &[String]) {

        match command {
            Command::Init => {
                let status = commands::command_init();
                match status {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: something gone wrong when tried to create database")}
                }
            }
            Command::Add => {
                match commands::command_add(&command_args) {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: unable to add to database");
                    }
                }
            }
            Command::Show => {
                match commands::command_show(&command_args) {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: unable to read from database")}
                }
            }
            Command::Remove => {
                match commands::command_remove(&command_args) {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: unable to remove from database")}
                }
            }
            Command::Keygen => {
                let password = commands::command_keygen(&command_args);
                println!("{}", password);
            }
            Command::Help => {
                commands::print_help();
            }
            _ => {
                eprintln!("Error: check the name of the command.\nTry \"key_storage help\".");
            }
        }
    }

    pub fn pre_main() {
        let args = get_env_args();
        check_env_args(&args);
        let command = get_command(&args);
        let command_args = get_command_args(&args);

        do_interaction(command, command_args);
    }


}