use clap::{Arg, ArgAction, Command};

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Let the fun begin!
fn main() {
    // Parse the command-line arguments.
    Command::new("Stem Cell")
        .version(VERSION)
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about(
            "A simple project to demonstrate the cross-platform release management process I use \
            for my open source work.",
        )
        .disable_version_flag(true)
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Print version information")
                .action(ArgAction::Version),
        )
        .get_matches();

    // Greet the user.
    println!("Hello, World!");
}
