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
    
    // first piece of fen
    let full_fen: String = match matches.get_one::<String>("fen").unwrap().parse() {
        Ok(val) => val,
        Err(e) => {
            println!("Bad FEN; {}", e);
            String::from("")
        }
    };

    let fenvec: Vec<String> = full_fen.split_whitespace().map(str::to_string).collect();
    // starting FEN: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    // has 6 parts
    if fenvec.len() != 6 {
        println!("BAD")
    } else {
        println!("GOOD")
    }
    let fen1: String = fenvec[0].clone();
    println!("first part of fen: {:?}", fen1);

    // split at separators
    let fenarray: Vec<String> = fen1.split("/").map(str::to_string).collect();
    println!("Split: {:?}", fenarray);

    println!("Test piece translate: p = {}", translate_piece(String::from("p")));
    println!("Test piece translate: P = {}", translate_piece(String::from("P")));
    println!("Test piece translate: Q = {}", translate_piece(String::from("Q")));
    println!("Test piece translate: q = {}", translate_piece(String::from("q")));
} 

// sub symbols
fn translate_piece(x: String) -> String {
    let newsym: String = match x.as_str() {
        "p" => String::from("♙"),
        "n" => String::from("♘"),
        "b" => String::from("♗"),
        "r" => String::from("♖"),
        "q" => String::from("♕"),
        "k" => String::from("♔"),
        "P" => String::from("♟"),
        "N" => String::from("♞"),
        "B" => String::from("♝"),
        "R" => String::from("♜"),
        "Q" => String::from("♛"),
        "K" => String::from("♚"),
        _ => String::from("")
    };
    newsym.to_string()
}