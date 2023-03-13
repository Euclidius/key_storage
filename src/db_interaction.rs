pub mod db_interaction {
    use rusqlite::{Connection};
    use rusqlite::types::ToSql;
    
    #[derive(Debug)]
    pub struct Pass {
        pub site: String,
        pub login: String,
        pub pass: String,
    }

    //creates a new struct Pass with initialized params
    impl Pass {
        pub fn new(site:String, login:String, pass:String) -> Pass {
            Pass {
                site,
                login,
                pass,
            }
        }
    }

    //connects to db. if not exist, creates file of it
    pub fn make_connection(path:&String) -> Result<Connection, rusqlite::Error> {
        Connection::open(path)
    }

    //puts an empty table to db
    pub fn create_table(conn:&Connection) -> Result<usize, rusqlite::Error> {
        conn.execute(
            "CREATE TABLE passwords (
                id INTEGER PRIMARY KEY,
                site TEXT NOT NULL,
                login TEXT NOT NULL,
                pass TEXT NOT NULL
            )",
            [],
        )
    }

    //inputs a Pass struct in the db
    pub fn insert_in_table(conn:&Connection, pass:&Pass) -> Result<usize, rusqlite::Error> {
        conn.execute(
            "INSERT INTO passwords (site, login, pass)
                    VALUES (?1, ?2, ?3)",
            &[&pass.site as &dyn ToSql, &pass.login as &dyn ToSql, &pass.pass as &dyn ToSql],
        )
    }

    pub fn print_passes(conn:&Connection) {
        let mut stmt = conn
            .prepare("SELECT id, site, login, pass FROM passwords")
            .unwrap();

        let pass_iter = stmt.query_map([], |row| {
            Ok(Pass {
                site: row.get(1).unwrap(),
                login: row.get(2).unwrap(),
                pass: row.get(3).unwrap(),
            })
        }).unwrap();

        for line in pass_iter {
            println!("Login: {}, Password: {}", line.as_ref().unwrap().login, line.as_ref().unwrap().pass);
        }
    }

    pub fn print_passes_from_site(conn:&Connection, site:&String) {
        let sql = format!("SELECT id, site, login, pass FROM passwords WHERE site = '{}'", site);

        let mut stmt = conn
            .prepare(&sql)
            .unwrap();    


        let pass_iter = stmt.query_map([], |row| {
            Ok(Pass {
                site: row.get(1).unwrap(),
                login: row.get(2).unwrap(),
                pass: row.get(3).unwrap(),
            })
        }).unwrap();

        for line in pass_iter {
            println!("Login: {}, Password: {}", line.as_ref().unwrap().login, line.as_ref().unwrap().pass);
        }
    }

    pub fn remove_from_table(conn:&Connection, pass:&Pass) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM passwords WHERE site = '{}' AND login = '{}' AND pass = '{}'", pass.site, pass.login, pass.pass);
        conn.execute(&sql, [])

    }

}
