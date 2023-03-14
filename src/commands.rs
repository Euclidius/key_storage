pub mod commands {
    use crate::db_interaction::db_interaction;

    use rusqlite::Connection;
    use passwords::PasswordGenerator;

    fn connect() -> Result<rusqlite::Connection, rusqlite::Error> {
        return db_interaction::make_connection(&String::from(".passwords.db3"));
    }

    fn get_inf_for_pass(len: &String) -> PasswordGenerator {
        let a = len.trim().parse();
        let a = match a {
            Ok(c) => {c}
            Err(_) => {
                eprintln!("Unable to convert {} to a number.", len);
                std::process::exit(1);
            }
        };

        return PasswordGenerator {
            length: a,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        };
    }


    pub fn print_help() {
        print!("
SYNOPSIS
\t $key_storage <command> <arguments for command>
COMMANDS
\t help - to see this mesage
\t init - creates a file .passwords.db3 in your current directory
\t add <site_name> <login> <password> - adds line to database
\t show <site_name> - shows all lines with given site
\t remove <site_name> <login> <password> - removes line from database
\t keygen <len_of_password> - generates password of given length
")
    }

    pub fn command_init() -> Result<Connection, rusqlite::Error> {
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

    pub fn command_add(command_args: &[String]) -> Result<usize, rusqlite::Error> {
        let pass = db_interaction::parse_pass(command_args);
        let conn = connect();
        let stat = match conn {
            Ok(connection) => {connection},
            Err(e) => {return Err(e)},
        };
        db_interaction::insert_in_table(&stat, &pass)
    }

    pub fn command_show(command_args: &[String]) -> Result<i32, rusqlite::Error>{
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

    pub fn command_keygen(command_args: &[String]) -> String {
        if command_args.len() < 1 {
            eprintln!("Error: not enough arguments for command");
            std::process::exit(1);
        }
        let pg = get_inf_for_pass(&command_args[0]);

        return pg.generate_one().expect("Error: unable to generate the password!");
    }

    pub fn command_remove(command_args: &[String]) -> Result<usize, rusqlite::Error> {
        let pass = db_interaction::parse_pass(command_args);
        let conn = connect();
        let stat = match conn {
            Ok(connection) => {connection},
            Err(e) => {return Err(e)},
        };
        db_interaction::remove_from_table(&stat, &pass)
    }
}