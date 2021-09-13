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
        let pass = read_password().unwrap();

        DatabaseCredentials {
            name: String::from(cli.name),
            user: String::from(cli.database_user),
            password: pass
        }
    }
}


fn get_password_display(password: String) -> String {
    let mut display: String = String::from("");

    let mut i = 0;
    let length = password.chars().count();

    while i < length {
        display.push_str("*");
        i = i + 1;
    }

    return display;
}

fn output_arguments_message(credentials: DatabaseCredentials) {
    let password_display: String = get_password_display(credentials.password);

    println!(
        "Creating a project with values name: {}, user: {}, pass: {}",
        credentials.name,
        credentials.user,
        password_display
    );
}

fn main() {
    let credentials: DatabaseCredentials = DatabaseCredentials::new(Cli::from_args());

    // Output message to display args
    output_arguments_message(credentials);
}


