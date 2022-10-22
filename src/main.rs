use clap::{arg, command, value_parser};
//use simple_grid::Grid;
use term_grid::{Grid, GridOptions, Direction, Filling, Cell};

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
    // if let Some(fen) = matches.get_one::<String>("fen") {
    //     println!("Value for fen: {}", fen);
    // }

    // if let Some(pretty) = matches.get_one::<bool>("pretty") {
    //     println!("Value for pretty: {}", pretty);
    // }

    // if let Some(debug) = matches.get_one::<bool>("debug") {
    //     println!("Value for debug: {}", debug);
    // }

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
        println!("BAD FEN!")
    } //else {
    //    println!("GOOD")
    //}
    let fen1: String = fenvec[0].clone();
    //println!("first part of fen: {:?}", fen1);

    // split at separators
    let fenarray: Vec<String> = fen1.split("/").map(str::to_string).collect();
    // println!("Split: {:?}", fenarray);

    //println!("Test piece translate: p = {}", translate_piece("p"));
    //println!("Test piece translate: P = {}", translate_piece("P"));
    //println!("Test piece translate: Q = {}", translate_piece("Q"));
    //println!("Test piece translate: q = {}", translate_piece("q"));
    //println!("Test piece translate: 4 = '{}'", translate_piece("4"));

    let mut fentranslated: Vec<Vec<String>> = Vec::with_capacity(64);
    for rank in fenarray {
        let mut rankchars: Vec<String> = Vec::new();
        for chars in rank.split_inclusive("").filter(|&x| !x.is_empty()) {
            let t: String = translate_piece(chars).to_string().split("").collect();
            let i = t.parse::<i32>();
            let tvec: Vec<String> = match i {
                Ok(v) => std::iter::repeat(String::from("")).take(v as usize).collect(),
                Err(_) => vec![t]
            };
            for el in tvec {
                rankchars.push(el);
            }
        }
        fentranslated.push(rankchars);
    }
    //println!("Translated: {:?}", fentranslated.concat());

    let mut grid = Grid::new(GridOptions {
        filling:     Filling::Spaces(1),
        direction:   Direction::LeftToRight,
    });

    for s in fentranslated.concat() {
        grid.add(Cell::from(s.to_string()));
    }

    println!("{}", grid.fit_into_columns(8));

} 

// sub symbols
fn translate_piece(x: &str) -> &str {
    let newsym: &str = match x {
        "p" => "♟",
        "n" => "♞",
        "b" => "♝",
        "r" => "♜",
        "q" => "♛",
        "k" => "♚",
        "P" => "♙",
        "N" => "♘",
        "B" => "♗",
        "R" => "♖",
        "Q" => "♕",
        "K" => "♔",
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",     
        _ => ""
    };
    newsym
}