use structopt::StructOpt;
use rpassword::read_password;

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

fn main() {
    let arguments = Cli::from_args();
    let credentials: DatabaseCredentials = DatabaseCredentials::new(arguments);

    // Output message to display args
    credentials.output_arguments_message();
}


