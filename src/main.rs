// The CLI implementation was derived from https://rust-cli.github.io/book/index.html

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Rusty", about = "The Rusty CLI robot.")]
struct Cli {
    command: String,
}

mod commands;

fn main() {
    let args = Cli::from_args();

    if args.command == "hello" {
        commands::hello::run();
    } else if args.command == "hs12hdpw" {
        commands::hs12hdpw::run();
    }
}
