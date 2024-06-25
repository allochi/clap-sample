use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "echor", version = "0.1.0", author="allochi", about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(short = 'f', long)]
    first: Option<String>, // optional argument

    #[arg(short = 'l', long)]
    last: Option<String>, // optional argument

    #[arg(value_enum, default_value = "normal")]
    mode: Mode,

    #[command(subcommand)]
    command: Option<Commands>, // subcommand
}

#[derive(Subcommand)]
enum Commands {
    Hello(HelloArgs),                   // positional argument
    Bye { name: String },               // positional argument
    HelloTimes { name: String, n: u8 }, // multiple positional arguments
    HelloEveryone(HelloEveryoneArgs),   // define args in a struct (cleaner!)
}

#[derive(Args)]
struct HelloArgs {
    #[arg(default_value = "World")]
    name: String,
}

#[derive(Args)]
struct HelloEveryoneArgs {
    names: Vec<String>,
    #[arg(short, long)]
    upper: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Upper,
    Normal,
    Lower,
}

fn main() {
    let cli = Cli::parse();

    if cli.first.is_some() || cli.last.is_some() {
        println!("printing in {:?} mode", cli.mode);
        println!(
            "Hello, {} {}!",
            cli.first.unwrap_or_default(),
            cli.last.unwrap_or_default()
        );
    }

    match &cli.command {
        Some(Commands::Hello(args)) => {
            hello(&args.name);
        }
        Some(Commands::Bye { name }) => {
            bye(name);
        }
        Some(Commands::HelloTimes { name, n }) => {
            hello_times(name, *n);
        }
        // Some(Commands::HelloEveryone()) => {
        //     hello_everyone(names.clone(), *upper);
        // }
        Some(Commands::HelloEveryone(args)) => {
            hello_everyone(args.names.clone(), args.upper);
        }

        None => {}
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn bye(name: &str) {
    println!("Bye, {}!", name);
}

fn hello_times(name: &str, n: u8) {
    for _ in 0..n {
        println!("Hello, {}!", name);
    }
}

fn hello_everyone(names: Vec<String>, upper: bool) {
    for name in names {
        if upper {
            println!("Hello, {}!", name.to_uppercase());
        } else {
            println!("Hello, {}!", name);
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
