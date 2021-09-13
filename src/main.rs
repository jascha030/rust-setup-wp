use structopt::StructOpt;
use rpassword::read_password;

#[derive(StructOpt)]
struct Cli {
    name: String,
    database_user: String
}

struct DatabaseCredentials {
    name: String,
    user: String,
    password: String
}

impl DatabaseCredentials {
    pub fn new(cli: Cli) -> Self {
        println!("Database password: ");
        let password = read_password().unwrap();

        DatabaseCredentials {
            name: String::from(cli.name),
            user: String::from(cli.database_user),
            password
        }
    }

    pub fn password_display(&self) -> String {
        let mut display: String = String::from("");

        let mut i = 0;
        let length: usize = self.password.chars().count();

        while i < length {
            display.push_str("*");
            i = i + 1;
        }

        return display;
    }
}

fn output_arguments_message(credentials: DatabaseCredentials) {
    println!(
        "Creating a project with values name: {}, user: {}, pass: {}",
        credentials.name,
        credentials.user,
        credentials.password_display()
    );
}

fn main() {
    let arguments = Cli::from_args();
    let credentials: DatabaseCredentials = DatabaseCredentials::new(arguments);

    // Output message to display args
    output_arguments_message(credentials);
}


