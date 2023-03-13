pub mod interface {
    use std::env;
    use rusqlite::Connection;

    use crate::db_interaction::db_interaction;
/*
TODO write a function, which would incapsulate this module
 */

/*
TODO rewrite this module with GUI with gtk4 (MUCH MUCH LATER)
 */
    pub enum Command {
        Init,
        Add,
        Show,
        Remove,
        Error
    }

    pub fn get_env_args() -> Vec<String> {
        env::args().collect::<Vec<String>>()  
    }

    pub fn check_env_args(env_args: &Vec<String>) {
        if env_args.len() < 2 {
            eprintln!("Usage is \"key_storage <command> <arguments for command>\"");
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
            _ => {Command::Error}
        }
    }

    pub fn get_command_args(env_args: &[String]) -> &[String] {
        &env_args[2..]
    }

    fn parse_pass(env_args: &[String]) -> db_interaction::Pass {
        if env_args.len() < 3 {
            eprintln!("Error: not enough arguments for command");
            std::process::exit(1);
        }
        return db_interaction::Pass::new(env_args[0].to_string(), env_args[1].to_string(), env_args[2].to_string());
    }

    fn command_init() -> Result<Connection, rusqlite::Error> {
        let conn = connect();
        let stat = match conn {
            Ok(connection) => {connection},
            Err(e) => {return Err(e)},
        };
        let creating = db_interaction::create_table(&stat);
        match creating{
            Ok(_) => {return Ok(stat)},
            Err(e) => {return Err(e)}

        }
    }

    fn connect() -> Result<rusqlite::Connection, rusqlite::Error> {
        return db_interaction::make_connection(&String::from(".passwords.db3"));
    }

    fn command_add(command_args: &[String]) -> Result<usize, rusqlite::Error> {
        let pass = parse_pass(command_args);
        let conn = connect();
        let stat = match conn {
            Ok(connection) => {connection},
            Err(e) => {return Err(e)},
        };
        db_interaction::insert_in_table(&stat, &pass)
    }

    fn command_show(command_args: &[String]) -> Result<i32, rusqlite::Error>{
        if command_args.len() < 1 {
            eprintln!("Error: not enough arguments to this command");
            std::process::exit(1);
        }
        let site = command_args[0].to_string();
        let conn = connect();
        let stat = match conn {
            Ok(connection) => {connection},
            Err(e) => {return Err(e)},
        };
        db_interaction::print_passes_from_site(&stat, &site);
        return Ok(0);
    }

    fn command_remove(command_args: &[String]) -> Result<usize, rusqlite::Error> {
        let pass = parse_pass(command_args);
        let conn = connect();
        let stat = match conn {
            Ok(connection) => {connection},
            Err(e) => {return Err(e)},
        };
        db_interaction::remove_from_table(&stat, &pass)
    }

    pub fn do_interaction(command:Command, command_args: &[String]) {

        match command {
            Command::Init => {
                let status = command_init();
                match status {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: something gone wrong when tried to create database")}
                }
            }
            Command::Add => {
                match command_add(&command_args) {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: unable to add to database");
                    }
                }
            }
            Command::Show => {
                match command_show(&command_args) {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: unable to read from database")}
                }
            }
            Command::Remove => {
                match command_remove(&command_args) {
                    Ok(_) => {}
                    Err(_) => {eprintln!("Error: unable to remove from database")}
                }
            }
            _ => {
                eprintln!("Error: check the name of the command.");
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