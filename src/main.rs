use structopt::StructOpt;
use rpassword::read_password;
use mysql::*;
use mysql::prelude::*;
use std::process::{exit, Output};
use std::process::Command;
use std::fs;

#[derive(StructOpt)]
struct Cli {
    name: String,
    database_user: String,
}

struct DatabaseCredentials {
    name: String,
    user: String,
    password: String,
}

impl DatabaseCredentials {
    pub fn new(cli: Cli) -> Self {
        println!("Database password: ");

        DatabaseCredentials {
            name: String::from(cli.name),
            user: String::from(cli.database_user),
            password: read_password().unwrap(),
        }
    }

    pub fn password_display(&self) -> String {
        let mut display: String = String::from("");
        let mut i = 0;

        while i < self.password.chars().count() {
            display.push_str("*");
            i = i + 1;
        }

        return display;
    }

    pub fn output_arguments_message(&self) {
        println!(
            "Creating a project with values name: {}, user: {}, pass: {}",
            self.name,
            self.user,
            self.password_display()
        );
    }
}

fn create_database(credentials: DatabaseCredentials) {
    let opts = Opts::from_url(
        format!("mysql://{}:{}@localhost:3306", credentials.user, credentials.password).as_str()
    ).unwrap();

    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let create_result = conn.query_drop(format!("CREATE DATABASE {}", format!("wp_{}", credentials.name)));

    if create_result.is_err() {
        println!("Error: {}", create_result.unwrap_err());
        exit(1);
    } else {
        println!("Created database: {}", format!("wp_{}", credentials.name));
    }
}

fn main() {
    let arguments = Cli::from_args();
    let credentials: DatabaseCredentials = DatabaseCredentials::new(arguments);
    let name = credentials.name.as_str();

    // Output message to display args
    credentials.output_arguments_message();

    // Outputs success message or exits with MySQL error
    // create_database(credentials);

    match fs::create_dir(name) {
        Err(error) => {
            println!("Failed to create dir: \"{}\", Reason: {:?}", name, error.kind());
            exit(1);
        },
        Ok(_) => {}
    }

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(format!("composer create-project roots/bedrock {}", name))
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("composer create-project roots/bedrock {}", name))
            .output()
            .expect("failed to execute process")
    };

    println!("{}", String::from_utf8(output.stdout).unwrap().as_str());
    println!("Project: {}, was created successfully!", name);
}


