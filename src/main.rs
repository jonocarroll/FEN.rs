use clap::{arg, command, value_parser};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([fen] "Input FEN"))
        .arg(
            arg!(
                -p --pretty "Activate pretty printing"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(bool)),
        )
        .arg(arg!(
            -d --debug "Turn debugging information on"
        )
        .required(false)
        .value_parser(value_parser!(bool)),
    )
    .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(fen) = matches.get_one::<String>("fen") {
        println!("Value for fen: {}", fen);
    }

    if let Some(pretty) = matches.get_one::<bool>("pretty") {
        println!("Value for pretty: {}", pretty);
    }

    if let Some(debug) = matches.get_one::<bool>("debug") {
        println!("Value for debug: {}", debug);
    }
    // Continued program logic goes here...
} 