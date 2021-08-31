#![deny(clippy::all, clippy::pedantic, warnings)]

use clap::App;

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Let the fun begin!
fn main() {
    // Parse the command-line arguments.
    App::new("Stem Cell")
        .version(VERSION)
        .version_short("v")
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about(
            "A simple project to demonstrate the cross-platform release management process I use \
            for my open source work.",
        )
        .get_matches();

    // Greet the user.
    println!("Hello, World!");
}
