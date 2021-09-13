use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    name: String,
    user: String,
    password: String,
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

fn output_arguments_message(args: Cli) {
    let password_display: String = get_password_display(args.password);

    println!(
        "Creating a project with values name: {}, user: {}, pass: {}",
        args.name,
        args.user,
        password_display
    );
}

fn main() {
    let args = Cli::from_args();

    // Output message to display args
    output_arguments_message(args);
}


