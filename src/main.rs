//! Parse a Forsyth–Edwards Notation (FEN) string
//!
//! Validates most of the components and produces either
//! a terminal or graphical representation of the board
//!
//! # Examples
//! ```
//! cargo -q run -- "r1b1k2r/2qnbppp/p2ppn2/1p4B1/3NPPP1/2N2Q2/PPP4P/2KR1B1R w kq b6 0 11"
//! cargo -q run -- "r1b1k2r/2qnbppp/p2ppn2/1p4B1/3NPPP1/2N2Q2/PPP4P/2KR1B1R w kq b6 0 11" -i
//! cargo -q run -- "r1b1k2r/2qnbppp/p2ppn2/1p4B1/3NPPP1/2N2Q2/PPP4P/2KR1B1R w kq b6 0 11" -i -w
//! ```

use clap::Parser;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

#[derive(Parser)]
#[command(name = "fen")]
#[command(about = "Parse a Forsyth–Edwards Notation (FEN) string", long_about = None)]
struct Args {
    /// input FEN string
    #[arg(required = true)]
    fen: String,
    /// spawn a graphical window containing the board
    #[arg(short('w'), long("window"))]
    window: bool,
    /// show information extracted from the fen
    #[arg(short('i'), long("info"))]
    info: bool,
    /// (unused) use debug mode
    #[arg(short('d'), long("debug"))]
    debug: bool,
}

pub struct Board {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl Board {
    fn render(&mut self, args: &RenderArgs, pieces: &Vec<Vec<String>>) {
        use graphics::*;

        // https://colorswall.com/palette/166635
        const DARKSQ: [f32; 4] = [67.0 / 255.0, 74.0 / 255.0, 58.0 / 255.0, 1.0];
        const LIGHTSQ: [f32; 4] = [180.0 / 255.0, 188.0 / 255.0, 170.0 / 255.0, 1.0];
        let white_pieces: Vec<String> = vec!["♙", "♘", "♗", "♖", "♕", "♔"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let black_pieces: Vec<String> = ["♟", "♞", "♝", "♜", "♛", "♚"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();

        let mut glyph_cache =
            GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        let mut piece_glyph_cache =
            GlyphCache::new("assets/FreeSerif-4aeK.ttf", (), TextureSettings::new()).unwrap();

        // let rotation = self.rotation;
        let (x, y) = (args.window_size[0], args.window_size[1]);
        let square = rectangle::square(0.0, 0.0, x / 8.0);
        let rank_corner = (0.05 * (x / 8.0), 0.25 * (y / 8.0));
        let file_corner = (0.8 * (x / 8.0), 0.9 * (y / 8.0));
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(DARKSQ, gl);

            let mut offset_x = 0.0;
            let mut offset_y = 0.0;
            let mut sq_col = DARKSQ;
            let mut other_col;
            let mut piece_col: [f32; 4];
            let mut piece: String;
            let mut tmp_piece: &str;
            for _r in (1..=8).rev() {
                for _f in 1..=8 {
                    sq_col = if sq_col == LIGHTSQ { DARKSQ } else { LIGHTSQ };
                    rectangle(sq_col, square, c.transform.trans(offset_x, offset_y), gl);
                    other_col = if sq_col == LIGHTSQ { DARKSQ } else { LIGHTSQ };
                    // annotate files
                    if _r == 1 {
                        text(
                            other_col,
                            16_u32,
                            &files[_f - 1].to_string()[..],
                            &mut glyph_cache,
                            c.transform
                                .trans(offset_x + file_corner.0, offset_y + file_corner.1),
                            gl,
                        )
                        .unwrap();
                    }
                    // annotate ranks
                    if _f == 1 {
                        text(
                            other_col,
                            16_u32,
                            &_r.to_string()[..],
                            &mut glyph_cache,
                            c.transform
                                .trans(offset_x + rank_corner.0, offset_y + rank_corner.1),
                            gl,
                        )
                        .unwrap();
                    }
                    // add the pieces
                    piece = pieces[8 - _r][_f - 1].clone();
                    if white_pieces.contains(&piece) {
                        piece_col = [1.0, 1.0, 1.0, 1.0];
                        // replace with corresponding white piece for fill
                        tmp_piece = match &piece[..] {
                            "♙" => "♟",
                            "♘" => "♞",
                            "♗" => "♝",
                            "♖" => "♜",
                            "♕" => "♛",
                            "♔" => "♚",
                            _ => "",
                        };
                        piece = tmp_piece.to_string();
                    } else if black_pieces.contains(&piece) {
                        piece_col = [0.0, 0.0, 0.0, 1.0]
                    } else {
                        piece_col = [1.0, 0.0, 0.0, 0.0]
                    };
                    text(
                        piece_col,
                        60_u32,
                        &piece[..],
                        &mut piece_glyph_cache,
                        c.transform
                            .trans(offset_x + (0.15 * (x / 8.0)), offset_y + (0.8 * (y / 8.0))),
                        gl,
                    )
                    .unwrap();
                    offset_x = offset_x + (x / 8.0);
                }
                offset_x = 0.0;
                offset_y = offset_y + (y / 8.0);
                sq_col = if sq_col == LIGHTSQ { DARKSQ } else { LIGHTSQ };
            }
        });
    }

    #[allow(unused)]
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let args = Args::parse();

    let fenvec: Vec<String> = args.fen.split_whitespace().map(str::to_string).collect();

    // starting FEN: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    // has 6 parts
    if fenvec.len() != 6 {
        eprintln!("Error: FEN does not contain 6 elements");
        eprintln!("Example FEN: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        if fenvec.len() < 1 {
            std::process::exit(1)
        };
    }

    // process layout
    let legal_chars = "KQBNRPkqbnrp12345678/".chars().collect::<Vec<_>>();
    if fenvec.len() > 0 {
        if !fenvec[0].chars().all(|s| legal_chars.contains(&s)) {
            eprintln!("Error: Unexpected symbol in layout string {}", fenvec[0]);
            std::process::exit(1)
        }
    }

    // process nextmove
    if fenvec.len() > 1 && args.info {
        match fenvec[1].as_str() {
            "w" => println!("{}", "White to move"),
            "b" => println!("{}", "Black to move"),
            _ => eprintln!("Error: Expected 'w' or 'b' in second element"),
        }
    }

    // process castling rights
    if fenvec.len() > 2 {
        if fenvec[2].as_str() == "-" && args.info {
            println!("{}", "Neither side can castle");
        } else if args.info {
            if fenvec[2].find('K').is_some() {
                println!("{}", "White can castle kingside");
            }
            if fenvec[2].find('Q').is_some() {
                println!("{}", "White can castle queenside");
            }
            if fenvec[2].find('k').is_some() {
                println!("{}", "Black can castle kingside");
            }
            if fenvec[2].find('q').is_some() {
                println!("{}", "Black can castle queenside");
            }
        }
        if fenvec[2].find(['-', 'K', 'Q', 'k', 'q']).is_none() {
            eprintln!(
                "Error: Expected one or more of [KQkq-] in third element to denote castling rights"
            )
        }
        if !fenvec[2]
            .chars()
            .all(|s| vec!['-', 'K', 'Q', 'k', 'q'].contains(&s))
        {
            eprintln!("Error: Unexpected symbol in third element (castling rights)")
        }
    }

    // process en-passant
    if fenvec.len() > 3 && args.info {
        if fenvec[3].as_str() == "-" {
            println!("{}", "No en-passant target square is available")
        } else {
            println!("En-passant target square is {}", fenvec[3])
        }
    }

    // split at separators
    let fenarray: Vec<String> = fenvec[0].split("/").map(str::to_string).collect();

    // translate to glyphs
    let mut fentranslated: Vec<Vec<String>> = Vec::with_capacity(64);
    for rank in fenarray {
        let mut rankchars: Vec<String> = Vec::new();
        for chars in rank.split_inclusive("").filter(|&x| !x.is_empty()) {
            let t: String = translate_piece(chars).to_string().split("").collect();
            let i = t.parse::<i32>();
            let tvec: Vec<String> = match i {
                Ok(v) => std::iter::repeat(String::from(""))
                    .take(v as usize)
                    .collect(),
                Err(_) => vec![t],
            };
            for el in tvec {
                rankchars.push(el);
            }
        }
        fentranslated.push(rankchars);
    }

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for s in fentranslated.concat() {
        grid.add(Cell::from(s.to_string()));
    }

    // print board in terminal
    println!("\n{}", grid.fit_into_columns(8));

    // spawn graphical window and show pieces
    if args.window {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create a Glutin window.
        let mut window: Window = WindowSettings::new("Chess Board", [600, 600])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        let mut board = Board {
            gl: GlGraphics::new(opengl),
        };

        // allow for the option of events and updating; not currently used
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                board.render(&args, &fentranslated);
            }

            if let Some(args) = e.update_args() {
                board.update(&args);
            }
        }
    }
}

// sub symbols
// White pieces are designated using uppercase letters ("PNBRQK"),
// while black pieces use lowercase letters ("pnbrqk").
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
        _ => "",
    };
    newsym
}
