extern crate clap;
use clap::{App, Arg};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
mod deck;
mod spread;

use crate::deck::Deck;
use crate::spread::{Spread, FilledSpread};

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
        .arg(Arg::with_name("spread_file")
             .long("spread_file")
             .takes_value(true)
             .required(true)
             .help("Path of file to use to generate tarot spread positions")
             )
        .arg(Arg::with_name("deck_file")
             .long("deck_file")
             .takes_value(true)
             .required(true)
             .help("Path of file used to pull tarot cards")
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

    let spread_path = Path::new(matches.value_of("spread_file").unwrap());
    let deck_path = Path::new(matches.value_of("deck_file").unwrap());

    fn calc_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    let mut deck = Deck::new_from_file(deck_path);
    let spread = Spread::new_from_file(spread_path);
    let filled_spread = FilledSpread::new(spread, &mut deck, calc_hash(&seed));
    filled_spread.print(calc_hash(&seed), &mut std::io::stdout());
}
