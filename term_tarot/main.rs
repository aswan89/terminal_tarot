extern crate clap;
extern crate pager;
extern crate dialoguer;
use clap::{App, Arg};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use pager::Pager;
use dialoguer::Select;
use std::fs;

extern crate term_tarot_lib;
use term_tarot_lib::deck::Deck;
use term_tarot_lib::spread::{Spread, FilledSpread};

fn main() {
    let now = std::time::SystemTime::now();

    let matches = App::new("Terminal Tarot")
        .arg(Arg::with_name("interactive")
             .short("i")
             .long("interactive")
             .help("Set to view tarot cards/positions in interactive manner")
             )
        .arg(Arg::with_name("detailed")
             .short("d")
             .long("detailed")
             .help("Should multiple interpretations for cards/positions be displayed?")
             )
        .arg(Arg::with_name("seed")
             .short("s")
             .long("seed")
             .takes_value(true)
             .help("Value used to draw cards and select interpretations")
             )
        .arg(Arg::with_name("spread_path")
             .long("spread_path")
             .takes_value(true)
             .help("Path that holds desired spread files. Can be a single file or a directory")
             )
        .arg(Arg::with_name("deck_path")
             .long("deck_path")
             .takes_value(true)
             .help("Path that holds desired deck files. Can be a single file or a directory")
             )
        .get_matches();

    #[derive(Hash)]
    struct InputSeed {
        value: String, 
    }

    let seed = InputSeed {
        value: matches.value_of("seed").unwrap_or(
             &now.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
            ).to_string(),
    };

    let spread_path = Path::new(matches.value_of("spread_path").unwrap());
    let deck_path = Path::new(matches.value_of("deck_path").unwrap());

    fn calc_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
/*
    fn choose_reading_element<T>(path: Path) -> T {
        let mut found_elements = match path.is_dir() {
            true => {
                fs::read_dir(path)
                    .unwrap()
                    .map(|x| Deck::new_from_file(&x.unwrap().path()))
                    .collect()
            },
            false => vec![Deck::new_from_file(&path)],
        };

        return match found_elements.len() {
            1 => found_elements.remove(0),
            _ => {
                let menu = Select::new();
                for element in found_elements {
                    menu.item(element.name);
                }
                let taridx = menu.with_prompt(
            }
        }
    };*/

    let deck = Deck::new_from_file(deck_path);
    let spread = Spread::new_from_file(spread_path);
    let filled_spread = FilledSpread::new(spread, &mut deck, calc_hash(&seed));

    if !matches.is_present("interactive") {
      Pager::new().setup();
    }

    filled_spread.print(
        matches.is_present("interactive"),
        calc_hash(&seed), 
        &mut std::io::stdout()
        );
}
