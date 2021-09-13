use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    name: String,
    user: String,
    password: String,
}

fn main() {
    let args = Cli::from_args();
}


