extern crate clap;
extern crate rand;
extern crate rusqlite;

use clap::{App, Arg};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn generate_password(length: usize) -> String {
    let mut rng = thread_rng();
    rng.sample_iter(&Alphanumeric).take(length).collect()
}

fn main() {
    // Parse command-line arguments using the clap crate
    let matches = App::new("password-generator")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .value_name("LENGTH")
                .help("Specify the length of the password")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Get the password length specified by the user
    let length = value_t!(matches, "length", usize).unwrap_or_else(|e| e.exit());

    // Generate and print a random password
    let password = generate_password(length);
    println!("Your random password is: {}", password);
}
