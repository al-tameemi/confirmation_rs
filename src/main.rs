use clap::Parser;

mod app;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// The command to run on user's approval
    #[arg(short, long)]
    command: String,
    /// The arguments for the command to run
    #[arg(short, long)]
    arguments: Vec<String>,
    /// The message or question to be displayed for the user
    #[arg(short, long)]
    message: String,
    /// Override the Deny/Cancel button text
    #[arg(short = 'd', long, default_value = "Cancel")]
    cancel_text: String,
    /// Override the Submit/Accept button text
    #[arg(short = 's', long, default_value = "Yes")]
    accept_text: String,
}

fn main() {
    let args = Args::parse();
    app::run(args);
}
